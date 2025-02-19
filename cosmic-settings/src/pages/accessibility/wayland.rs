use cosmic_protocols::a11y::v1::client::cosmic_a11y_manager_v1;
use sctk::{
    reexports::{
        calloop::{self, channel, LoopSignal},
        calloop_wayland_source::WaylandSource,
        client::{
            globals::{registry_queue_init, GlobalListContents},
            protocol::wl_registry,
            ConnectError, Connection, Dispatch, Proxy,
        },
    },
    registry::RegistryState,
};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub enum AccessibilityEvent {
    Magnifier(bool),
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub enum AccessibilityRequest {
    Magnifier(bool),
}

pub type Sender = calloop::channel::Sender<AccessibilityRequest>;

pub fn spawn_wayland_connection() -> Result<
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
        if let Err(err) = wayland_thread(conn, event_tx.clone(), request_rx) {
            tracing::warn!("Accessibility protocol wayland thread crashed: {}", err);
            let _ = event_tx.send(AccessibilityEvent::Closed);
        }
    });

    Ok((request_tx, event_rx))
}

fn wayland_thread(
    conn: Connection,
    tx: mpsc::Sender<AccessibilityEvent>,
    rx: channel::Channel<AccessibilityRequest>,
) -> anyhow::Result<()> {
    struct State {
        loop_signal: LoopSignal,
        tx: mpsc::Sender<AccessibilityEvent>,
        global: cosmic_a11y_manager_v1::CosmicA11yManagerV1,

        magnifier: bool,
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
        1..=1,
        (),
    ) else {
        return Ok(());
    };

    loop_handle
        .insert_source(rx, |request, _, state| match request {
            channel::Event::Msg(AccessibilityRequest::Magnifier(val)) => {
                state.global.set_magnifier(if val {
                    cosmic_a11y_manager_v1::ActiveState::Enabled
                } else {
                    cosmic_a11y_manager_v1::ActiveState::Disabled
                });
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
    };

    event_loop.run(None, &mut state, |_| {})?;
    Ok(())
}
