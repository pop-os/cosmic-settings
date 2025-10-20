// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// Make sure not to fail if pulse not found, and reconnect?
// change to device shouldn't send osd?

use cosmic::iced_futures::{self, Subscription, stream};
use futures::{SinkExt, executor::block_on};
use libpulse_binding::{
    callbacks::ListResult,
    channelmap::Map,
    context::{
        Context, FlagSet, State,
        introspect::{Introspector, ServerInfo, SinkInfo, SourceInfo},
        subscribe::{Facility, InterestMaskSet, Operation},
    },
    def::Retval,
    mainloop::{
        api::MainloopApi,
        events::io::IoEventInternal,
        standard::{IterateResult, Mainloop},
    },
    volume::{ChannelVolumes, Volume},
};
use std::{
    cell::{Cell, RefCell},
    io::{Read, Write},
    os::{
        fd::{FromRawFd, IntoRawFd, RawFd},
        raw::c_void,
    },
    rc::Rc,
    sync::mpsc,
};

use crate::pipewire::Availability;

pub fn subscription() -> iced_futures::Subscription<Event> {
    Subscription::run_with_id(
        "pulse",
        stream::channel(20, |sender| async {
            std::thread::spawn(move || thread(sender));
            futures::future::pending().await
        }),
    )
}

pub fn thread(sender: futures::channel::mpsc::Sender<Event>) {
    let Some(mut main_loop) = Mainloop::new() else {
        log::error!("Failed to create PA main loop");
        return;
    };

    let Some(mut context) = Context::new(&main_loop, "cosmic-osd") else {
        log::error!("Failed to create PA context");
        return;
    };

    let data = Rc::new(Data {
        main_loop: RefCell::new(Mainloop {
            _inner: Rc::clone(&main_loop._inner),
        }),
        introspector: context.introspect(),
        sink_port: RefCell::new(None),
        sink_volume: Cell::new(None),
        sink_mute: Cell::new(None),
        source_port: RefCell::new(None),
        source_volume: Cell::new(None),
        source_mute: Cell::new(None),
        default_sink_name: RefCell::new(None),
        default_source_name: RefCell::new(None),
        sender: RefCell::new(sender.clone()),
    });

    let data_clone = data.clone();
    context.set_subscribe_callback(Some(Box::new(move |facility, operation, index| {
        data_clone.subscribe_cb(facility.unwrap(), operation, index);
    })));

    let _ = context.connect(None, FlagSet::NOFAIL, None);

    loop {
        if sender.is_closed() {
            return;
        }

        match main_loop.iterate(false) {
            IterateResult::Success(_) => {}
            IterateResult::Err(_e) => {
                return;
            }
            IterateResult::Quit(_e) => {
                return;
            }
        }

        if context.get_state() == State::Ready {
            break;
        }
    }

    data.get_server_info();
    context.subscribe(
        InterestMaskSet::SERVER | InterestMaskSet::SINK | InterestMaskSet::SOURCE,
        |_| {},
    );

    if let Err((err, retval)) = main_loop.run() {
        log::error!("PA main loop returned {:?}, error {}", retval, err);
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    Balance(Option<f32>),
    Channels(PulseChannels),
    DefaultSink(String),
    DefaultSource(String),
    SinkPortChange(String, Availability),
    SinkMute(bool),
    SinkVolume(u32),
    SourcePortChange(String, Availability),
    SourceMute(bool),
    SourceVolume(u32),
}

enum Request {
    Volume(u32, f32),
    Balance(u32, f32),
    Quit,
}

#[derive(Debug)]
pub struct PulseChannels {
    tx: mpsc::Sender<Request>,
    pipe_tx: std::fs::File,
    index: u32,
}

impl Clone for PulseChannels {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            pipe_tx: self
                .pipe_tx
                .try_clone()
                .expect("failed to clone PulseChannels pipe writer"),
            index: self.index,
        }
    }
}

/// Data used by the [`handle_balance_io_new`] callback.
struct HandleBalanceData(
    Context,
    ChannelVolumes,
    Map,
    std::sync::mpsc::Receiver<Request>,
);

/// Callback for creating an IO event source [`MainloopApi::io_new`].
extern "C" fn handle_balance_io_new(
    api: *const MainloopApi,
    event: *mut IoEventInternal,
    reader_fd: RawFd,
    _flags: libpulse_binding::mainloop::events::io::FlagSet,
    data: *mut c_void,
) {
    // Take ownership of the data and borrow its contents.
    let mut data = unsafe { Box::<HandleBalanceData>::from_raw(data as _) };
    let HandleBalanceData(ctx, volumes, map, rx) = data.as_mut();

    // Return early if the context is not ready, and give the data back.
    if ctx.get_state() != State::Ready {
        let _ = Box::leak(data);
        return;
    }

    // If the first byte cannot be read, destroy this event source with its reader and data.
    let mut buf = [0u8; 1];
    let mut reader = unsafe { std::fs::File::from_raw_fd(reader_fd) };
    if reader.read_exact(&mut buf).is_err() {
        (unsafe { &*api })
            .io_free
            .as_ref()
            .expect("io_free function is missing")(event);
        return;
    }

    // Give ownership of the reader back.
    _ = reader.into_raw_fd();

    while let Ok(req) = rx.try_recv() {
        match req {
            Request::Volume(index, volume_scale) => {
                let mut intro = ctx.introspect();

                let new_scale = Volume((volume_scale * Volume::NORMAL.0 as f32).round() as u32);

                if let Some(v) = volumes.scale(new_scale) {
                    _ = intro.set_sink_volume_by_index(
                        index,
                        v,
                        Some(Box::new(|success| {
                            if !success {
                                tracing::error!("Failed to set sink balance");
                            }
                        })),
                    );
                }
            }
            Request::Balance(index, new_balance) => {
                if map.can_balance() {
                    if let Some(v) = volumes.set_balance(&map, new_balance) {
                        let mut intro = ctx.introspect();

                        _ = intro.set_sink_volume_by_index(
                            index,
                            v,
                            Some(Box::new(|success| {
                                if !success {
                                    tracing::error!("Failed to set sink balance");
                                }
                            })),
                        );
                    }
                }
            }
            Request::Quit => unsafe { &*api }
                .quit
                .as_ref()
                .expect("quit function missing")(api, 0),
        }
    }

    let _ = Box::leak(data);
}

impl PulseChannels {
    fn new(
        volumes: ChannelVolumes,
        map: Map,
        api: &MainloopApi,
        index: u32,
        ctx: Context,
    ) -> PulseChannels {
        let (reader, writer) = rustix::pipe::pipe_with(rustix::pipe::PipeFlags::CLOEXEC)
            .expect("failed to crate pipe");

        let (tx, rx) = mpsc::channel::<Request>();

        // Create IO event source object for handling speaker balance.
        let event_source = api.io_new.as_ref().unwrap()(
            api as *const _,
            reader.into_raw_fd(),
            libpulse_binding::mainloop::events::io::FlagSet::INPUT,
            Some(handle_balance_io_new),
            Box::into_raw(Box::new(HandleBalanceData(ctx, volumes, map, rx))) as *mut c_void,
        );

        if let Some(enable) = api.io_enable.as_ref() {
            enable(
                event_source,
                libpulse_binding::mainloop::events::io::FlagSet::INPUT,
            );
        }

        Self {
            tx,
            pipe_tx: std::fs::File::from(writer),
            index,
        }
    }

    /// Change the active index.
    #[inline]
    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    /// Set the speaker balance of the active sink.
    pub fn set_balance(&mut self, balance: f32) {
        if let Err(err) = self.tx.send(Request::Balance(self.index, balance)) {
            tracing::error!(?err, "Failed to send new balance to channel");
        } else {
            self.pipe_tx
                .write_all(&[1])
                .expect("PulseChannels pipe write failed");
        }
    }

    /// Set the volume of the active sink.
    pub fn set_volume(&mut self, volume: f32) {
        if let Err(err) = self.tx.send(Request::Volume(self.index, volume)) {
            tracing::error!(?err, "Failed to send new volume to channel");
        } else {
            self.pipe_tx
                .write_all(&[1])
                .expect("PulseChannels pipe write failed");
        }
    }

    /// Request the pulse thread to quit.
    pub fn quit(mut self) {
        _ = self.tx.send(Request::Quit);
        _ = self.pipe_tx.write_all(&[1]);
    }
}

struct Data {
    main_loop: RefCell<Mainloop>,
    default_sink_name: RefCell<Option<String>>,
    default_source_name: RefCell<Option<String>>,
    sink_port: RefCell<Option<String>>,
    sink_volume: Cell<Option<u32>>,
    sink_mute: Cell<Option<bool>>,
    source_port: RefCell<Option<String>>,
    source_volume: Cell<Option<u32>>,
    source_mute: Cell<Option<bool>>,
    introspector: Introspector,
    sender: RefCell<futures::channel::mpsc::Sender<Event>>,
}

impl Data {
    fn server_info_cb(self: &Rc<Self>, server_info: &ServerInfo) {
        let new_default_sink_name = server_info
            .default_sink_name
            .as_ref()
            .map(|x| x.clone().into_owned());
        let mut default_sink_name = self.default_sink_name.borrow_mut();
        if new_default_sink_name != *default_sink_name {
            if let Some(name) = &new_default_sink_name {
                _ = block_on(
                    self.sender
                        .borrow_mut()
                        .send(Event::DefaultSink(name.clone())),
                );
                self.get_sink_info_by_name(name);
            }
            *default_sink_name = new_default_sink_name;
        }

        let new_default_source_name = server_info
            .default_source_name
            .as_ref()
            .map(|x| x.clone().into_owned());
        let mut default_source_name = self.default_source_name.borrow_mut();
        if new_default_source_name != *default_source_name {
            if let Some(name) = &new_default_source_name {
                _ = block_on(
                    self.sender
                        .borrow_mut()
                        .send(Event::DefaultSource(name.clone())),
                );
                self.get_source_info_by_name(name);
            }
            *default_source_name = new_default_source_name;
        }
    }

    fn get_server_info(self: &Rc<Self>) {
        let data = self.clone();
        self.introspector
            .get_server_info(move |server_info| data.server_info_cb(server_info));
    }

    fn sink_info_cb(&self, sink_info_res: ListResult<&SinkInfo>) {
        if let ListResult::Item(sink_info) = sink_info_res {
            if sink_info.name.as_deref() != self.default_sink_name.borrow().as_deref() {
                return;
            }

            if let Some(port) = sink_info.active_port.as_deref() {
                let port_name = port.name.as_deref();
                if self.sink_port.borrow().as_deref() != port_name {
                    *self.sink_port.borrow_mut() = port_name.map(str::to_owned);
                    if let Some(name) = port_name {
                        if block_on(self.sender.borrow_mut().send(Event::SinkPortChange(
                            name.to_owned(),
                            match port.available {
                                libpulse_binding::def::PortAvailable::No => Availability::No,
                                libpulse_binding::def::PortAvailable::Yes => Availability::Yes,
                                _ => Availability::Unknown,
                            },
                        )))
                        .is_err()
                        {
                            self.main_loop.borrow_mut().quit(Retval(0));
                        }
                    }
                }
            }

            let balance = (sink_info.channel_map.can_balance()
                && sink_info.base_volume.is_normal())
            .then(|| sink_info.volume.get_balance(&sink_info.channel_map));

            let volume = sink_info.volume.max().0 / (Volume::NORMAL.0 / 100);
            if self.sink_mute.get() != Some(sink_info.mute) {
                self.sink_mute.set(Some(sink_info.mute));
                if block_on(
                    self.sender
                        .borrow_mut()
                        .send(Event::SinkMute(sink_info.mute)),
                )
                .is_err()
                {
                    self.main_loop.borrow_mut().quit(Retval(0));
                }
            }
            if self.sink_volume.get() != Some(volume) {
                self.sink_volume.set(Some(volume));
                if block_on(self.sender.borrow_mut().send(Event::SinkVolume(volume))).is_err() {
                    self.main_loop.borrow_mut().quit(Retval(0));
                }
            }
            if block_on(self.sender.borrow_mut().send(Event::Balance(balance))).is_err() {
                self.main_loop.borrow_mut().quit(Retval(0));
            }
            let mut main_loop = self.main_loop.borrow_mut();
            let api = main_loop.get_api();
            if let Some(mut ctx) = Context::new(&*main_loop, "balance") {
                let _ = ctx.connect(None, FlagSet::NOFAIL, None);

                let channels = PulseChannels::new(
                    sink_info.volume,
                    sink_info.channel_map,
                    api,
                    sink_info.index,
                    ctx,
                );

                if block_on(self.sender.borrow_mut().send(Event::Channels(channels))).is_err() {
                    main_loop.quit(Retval(0));
                }
            }
        }
    }

    fn source_info_cb(&self, source_info_res: ListResult<&SourceInfo>) {
        if let ListResult::Item(source_info) = source_info_res {
            if source_info.name.as_deref() != self.default_source_name.borrow().as_deref() {
                return;
            }

            if let Some(port) = source_info.active_port.as_deref() {
                let port_name = port.name.as_deref();
                if self.source_port.borrow().as_deref() != port_name {
                    *self.source_port.borrow_mut() = port_name.map(str::to_owned);
                    if let Some(name) = port_name {
                        if block_on(self.sender.borrow_mut().send(Event::SourcePortChange(
                            name.to_owned(),
                            match port.available {
                                libpulse_binding::def::PortAvailable::No => Availability::No,
                                libpulse_binding::def::PortAvailable::Yes => Availability::Yes,
                                _ => Availability::Unknown,
                            },
                        )))
                        .is_err()
                        {
                            self.main_loop.borrow_mut().quit(Retval(0));
                        }
                    }
                }
            }

            let volume = source_info.volume.max().0 / (Volume::NORMAL.0 / 100);
            if self.source_mute.get() != Some(source_info.mute) {
                self.source_mute.set(Some(source_info.mute));
                if block_on(
                    self.sender
                        .borrow_mut()
                        .send(Event::SourceMute(source_info.mute)),
                )
                .is_err()
                {
                    self.main_loop.borrow_mut().quit(Retval(0));
                }
            }
            if self.source_volume.get() != Some(volume) {
                self.source_volume.set(Some(volume));
                if block_on(self.sender.borrow_mut().send(Event::SourceVolume(volume))).is_err() {
                    self.main_loop.borrow_mut().quit(Retval(0));
                }
            }
        }
    }

    fn get_sink_info_by_index(self: &Rc<Self>, index: u32) {
        let data = self.clone();
        self.introspector.get_sink_info_by_index(
            index,
            move |sink_info_res: ListResult<&SinkInfo<'_>>| {
                data.sink_info_cb(sink_info_res);
            },
        );
    }

    fn get_sink_info_by_name(self: &Rc<Self>, name: &str) {
        let data = self.clone();
        self.introspector
            .get_sink_info_by_name(name, move |sink_info_res| {
                data.sink_info_cb(sink_info_res);
            });
    }

    fn get_source_info_by_index(self: &Rc<Self>, index: u32) {
        let data = self.clone();
        self.introspector
            .get_source_info_by_index(index, move |source_info_res| {
                data.source_info_cb(source_info_res);
            });
    }

    fn get_source_info_by_name(self: &Rc<Self>, name: &str) {
        let data = self.clone();
        self.introspector
            .get_source_info_by_name(name, move |source_info_res| {
                data.source_info_cb(source_info_res);
            });
    }

    fn subscribe_cb(
        self: &Rc<Self>,
        facility: Facility,
        _operation: Option<Operation>,
        index: u32,
    ) {
        match facility {
            Facility::Server => {
                self.get_server_info();
            }
            Facility::Sink => {
                self.get_sink_info_by_index(index);
            }
            Facility::Source => {
                self.get_source_info_by_index(index);
            }
            _ => {}
        }
    }
}
