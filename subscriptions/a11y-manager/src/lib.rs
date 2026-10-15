use cosmic_protocols::a11y::v1::client::cosmic_a11y_manager_v1;
use num_derive::{FromPrimitive, ToPrimitive};
use sctk::reexports::calloop::{self, LoopSignal, channel};
use sctk::reexports::calloop_wayland_source::WaylandSource;
use sctk::reexports::client::globals::{GlobalListContents, registry_queue_init};
use sctk::reexports::client::protocol::wl_registry;
use sctk::reexports::client::{ConnectError, Connection, Dispatch, Proxy, WEnum};
use sctk::registry::RegistryState;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub enum AccessibilityEvent {
    Bound(u32),
    Magnifier(bool),
    ScreenFilter {
        inverted: bool,
        filter: ColorFilter,
        filter_state: bool,
    },
    Closed,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ColorFilter {
    Greyscale,
    Deuteranopia,
    Protanopia,
    Tritanopia,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum AccessibilityRequest {
    Magnifier(bool),
    ScreenFilter {
        inverted: bool,
        filter: ColorFilter,
        filter_state: bool,
    },
}

pub type Sender = calloop::channel::Sender<AccessibilityRequest>;

pub fn spawn_wayland_connection(
    a11y_manager_min: u32,
) -> Result<
    (
        channel::Sender<AccessibilityRequest>,
        mpsc::Receiver<AccessibilityEvent>,
    ),
    ConnectError,
> {
    let (event_tx, event_rx) = mpsc::channel(10);
    let (request_tx, request_rx) = channel::channel();
    let conn = Connection::connect_to_env()?;

    std::thread::spawn(move || {
        if let Err(err) = wayland_thread(conn, event_tx.clone(), request_rx, a11y_manager_min) {
            tracing::warn!("Accessibility protocol wayland thread crashed: {}", err);
            let _ = event_tx.blocking_send(AccessibilityEvent::Closed);
        }
    });

    Ok((request_tx, event_rx))
}

fn protocol_to_color_filter(protocol: WEnum<cosmic_a11y_manager_v1::Filter>) -> ColorFilter {
    match protocol {
        WEnum::Value(cosmic_a11y_manager_v1::Filter::Greyscale) => ColorFilter::Greyscale,
        WEnum::Value(cosmic_a11y_manager_v1::Filter::DaltonizeProtanopia) => {
            ColorFilter::Protanopia
        }
        WEnum::Value(cosmic_a11y_manager_v1::Filter::DaltonizeDeuteranopia) => {
            ColorFilter::Deuteranopia
        }
        WEnum::Value(cosmic_a11y_manager_v1::Filter::DaltonizeTritanopia) => {
            ColorFilter::Tritanopia
        }
        WEnum::Unknown(_) | WEnum::Value(_) => ColorFilter::Unknown,
    }
}

fn color_filter_to_protocol(filter: ColorFilter) -> cosmic_a11y_manager_v1::Filter {
    match filter {
        ColorFilter::Greyscale => cosmic_a11y_manager_v1::Filter::Greyscale,
        ColorFilter::Protanopia => cosmic_a11y_manager_v1::Filter::DaltonizeProtanopia,
        ColorFilter::Deuteranopia => cosmic_a11y_manager_v1::Filter::DaltonizeDeuteranopia,
        ColorFilter::Tritanopia => cosmic_a11y_manager_v1::Filter::DaltonizeTritanopia,
        ColorFilter::Unknown => cosmic_a11y_manager_v1::Filter::Unknown,
    }
}

fn wayland_thread(
    conn: Connection,
    tx: mpsc::Sender<AccessibilityEvent>,
    rx: channel::Channel<AccessibilityRequest>,
    a11y_manager_min: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    struct State {
        loop_signal: LoopSignal,
        tx: mpsc::Sender<AccessibilityEvent>,
        global: cosmic_a11y_manager_v1::CosmicA11yManagerV1,

        magnifier: bool,
        screen_inverted: bool,
        screen_filter: ColorFilter,
        screen_filter_state: bool,
    }

    impl Dispatch<cosmic_a11y_manager_v1::CosmicA11yManagerV1, ()> for State {
        fn event(
            state: &mut Self,
            _proxy: &cosmic_a11y_manager_v1::CosmicA11yManagerV1,
            event: <cosmic_a11y_manager_v1::CosmicA11yManagerV1 as Proxy>::Event,
            _data: &(),
            _conn: &Connection,
            _qhandle: &sctk::reexports::client::QueueHandle<Self>,
        ) {
            let mut handle_screen_filter_event =
                |inverted: bool,
                 filter: ColorFilter,
                 filter_state: bool,
                 set_filter_selection: bool| {
                    if inverted != state.screen_inverted
                        || filter != state.screen_filter
                        || filter_state != state.screen_filter_state
                    {
                        if state
                            .tx
                            .blocking_send(AccessibilityEvent::ScreenFilter {
                                inverted,
                                filter: if set_filter_selection {
                                    filter
                                } else {
                                    state.screen_filter
                                },
                                filter_state,
                            })
                            .is_err()
                        {
                            state.loop_signal.stop();
                            state.loop_signal.wakeup();
                        };
                        state.screen_inverted = inverted;
                        state.screen_filter_state = filter_state;
                        if set_filter_selection {
                            state.screen_filter = filter;
                        }
                    }
                };

            match event {
                cosmic_a11y_manager_v1::Event::Magnifier { active } => {
                    let magnifier = active
                        .into_result()
                        .unwrap_or(cosmic_a11y_manager_v1::ActiveState::Disabled)
                        == cosmic_a11y_manager_v1::ActiveState::Enabled;
                    if magnifier != state.magnifier {
                        if state
                            .tx
                            .blocking_send(AccessibilityEvent::Magnifier(magnifier))
                            .is_err()
                        {
                            state.loop_signal.stop();
                            state.loop_signal.wakeup();
                        };
                        state.magnifier = magnifier;
                    }
                }
                cosmic_a11y_manager_v1::Event::ScreenFilter { inverted, filter } => {
                    let inverted = inverted
                        .into_result()
                        .unwrap_or(cosmic_a11y_manager_v1::ActiveState::Disabled)
                        == cosmic_a11y_manager_v1::ActiveState::Enabled;
                    let filter_state =
                        filter != WEnum::Value(cosmic_a11y_manager_v1::Filter::Disabled);
                    let filter = protocol_to_color_filter(filter);

                    handle_screen_filter_event(inverted, filter, filter_state, filter_state);
                }
                cosmic_a11y_manager_v1::Event::ScreenFilter2 {
                    inverted,
                    filter,
                    filter_state,
                } => {
                    let inverted = inverted
                        .into_result()
                        .unwrap_or(cosmic_a11y_manager_v1::ActiveState::Disabled)
                        == cosmic_a11y_manager_v1::ActiveState::Enabled;
                    let filter_state = filter_state
                        .into_result()
                        .unwrap_or(cosmic_a11y_manager_v1::ActiveState::Disabled)
                        == cosmic_a11y_manager_v1::ActiveState::Enabled;
                    let filter = protocol_to_color_filter(filter);

                    handle_screen_filter_event(inverted, filter, filter_state, true);
                }
                _ => unreachable!(),
            }
        }
    }
    impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
        fn event(
            _state: &mut Self,
            _proxy: &wl_registry::WlRegistry,
            _event: <wl_registry::WlRegistry as Proxy>::Event,
            _data: &GlobalListContents,
            _conn: &Connection,
            _qhandle: &sctk::reexports::client::QueueHandle<Self>,
        ) {
            // We don't care about any dynamic globals
        }
    }

    let mut event_loop = calloop::EventLoop::<State>::try_new().unwrap();

    let loop_handle = event_loop.handle();
    let (globals, event_queue) = registry_queue_init(&conn).unwrap();
    let qhandle = event_queue.handle();

    WaylandSource::new(conn, event_queue)
        .insert(loop_handle.clone())
        .map_err(|err| err.error)?;

    let registry_state = RegistryState::new(&globals);
    let Ok(global) = registry_state.bind_one::<cosmic_a11y_manager_v1::CosmicA11yManagerV1, _, _>(
        &qhandle,
        a11y_manager_min..=3,
        (),
    ) else {
        return Ok(());
    };

    let _ = tx.blocking_send(AccessibilityEvent::Bound(global.version()));

    loop_handle
        .insert_source(rx, |request, _, state| match request {
            channel::Event::Msg(AccessibilityRequest::Magnifier(val)) => {
                state.global.set_magnifier(if val {
                    cosmic_a11y_manager_v1::ActiveState::Enabled
                } else {
                    cosmic_a11y_manager_v1::ActiveState::Disabled
                });
            }
            channel::Event::Msg(AccessibilityRequest::ScreenFilter {
                inverted,
                filter,
                filter_state,
            }) => {
                let filter = color_filter_to_protocol(filter);
                if state.global.version() >= cosmic_a11y_manager_v1::EVT_SCREEN_FILTER2_SINCE {
                    state.global.set_screen_filter2(
                        if inverted {
                            cosmic_a11y_manager_v1::ActiveState::Enabled
                        } else {
                            cosmic_a11y_manager_v1::ActiveState::Disabled
                        },
                        filter,
                        if filter_state {
                            cosmic_a11y_manager_v1::ActiveState::Enabled
                        } else {
                            cosmic_a11y_manager_v1::ActiveState::Disabled
                        },
                    );
                } else if filter_state == false && state.screen_filter_state == false {
                    let filter = protocol_to_color_filter(WEnum::Value(filter));
                    if state
                        .tx
                        .blocking_send(AccessibilityEvent::ScreenFilter {
                            inverted,
                            filter,
                            filter_state,
                        })
                        .is_err()
                    {
                        state.loop_signal.stop();
                        state.loop_signal.wakeup();
                    };
                    state.screen_filter = filter;
                } else {
                    state.global.set_screen_filter(
                        if inverted {
                            cosmic_a11y_manager_v1::ActiveState::Enabled
                        } else {
                            cosmic_a11y_manager_v1::ActiveState::Disabled
                        },
                        if filter_state {
                            filter
                        } else {
                            cosmic_a11y_manager_v1::Filter::Disabled
                        },
                    );
                }
            }
            channel::Event::Closed => {
                state.loop_signal.stop();
                state.loop_signal.wakeup();
            }
        })
        .map_err(|err| err.error)?;

    let mut state = State {
        loop_signal: event_loop.get_signal(),
        tx,
        global,

        magnifier: false,
        screen_inverted: false,
        screen_filter: ColorFilter::Unknown,
        screen_filter_state: false,
    };

    event_loop.run(None, &mut state, |_| {})?;
    Ok(())
}
