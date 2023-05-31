//! Test application to list all available outputs.

use std::error::Error;

use sctk::{
    delegate_output, delegate_registry,
    output::{OutputHandler, OutputInfo, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};
use wayland_client::{globals::registry_queue_init, protocol::wl_output, Connection, QueueHandle};

pub fn outputs() -> Result<impl Iterator<Item = OutputInfo>, Box<dyn Error>> {
    let conn = Connection::connect_to_env()?;

    let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
    let qh = event_queue.handle();

    let registry_state = RegistryState::new(&globals);

    let output_delegate = OutputState::new(&globals, &qh);

    let mut list_outputs = ListOutputs {
        registry_state,
        output_state: output_delegate,
    };

    event_queue.roundtrip(&mut list_outputs)?;

    // Now our outputs have been initialized with data, we may access what outputs exist and information about
    // said outputs using the output delegate.
    Ok(list_outputs
        .output_state
        .outputs()
        .filter_map(move |output| list_outputs.output_state.info(&output)))
}

struct ListOutputs {
    registry_state: RegistryState,
    output_state: OutputState,
}

impl OutputHandler for ListOutputs {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

delegate_output!(ListOutputs);
delegate_registry!(ListOutputs);

impl ProvidesRegistryState for ListOutputs {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers! {
        OutputState
    }
}
