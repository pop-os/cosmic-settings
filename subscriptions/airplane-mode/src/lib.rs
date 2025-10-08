// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use futures::{FutureExt, StreamExt};
use iced_futures::Subscription;
use std::collections::HashMap;

pub fn subscription() -> iced_futures::Subscription<bool> {
    Subscription::run_with_id(
        "airplane-mode",
        async {
            match rfkill::rfkill_updates() {
                Ok(updates) => updates.filter_map(|state| async {
                    match state {
                        Ok(state) => Some(is_airplane_mode(&state)),
                        Err(err) => {
                            log::error!("Failed to read rfkill: {}", err);
                            None
                        }
                    }
                }),
                Err(err) => {
                    log::error!("Failed to monitor rfkill: {}", err);
                    futures::future::pending().await
                }
            }
        }
        .flatten_stream(),
    )
}

// Test that:
// - There is at least one device
// - All devices have either a hard or soft block active
fn is_airplane_mode(rfkill_state: &HashMap<u32, rfkill::DeviceState>) -> bool {
    !rfkill_state.is_empty()
        && rfkill_state
            .values()
            .all(|device_state| device_state.hard || device_state.soft)
}

mod rfkill {
    use futures::stream::Stream;
    use std::os::unix::fs::OpenOptionsExt;
    use std::{collections::HashMap, fs, io, mem, slice};
    use tokio::io::unix::AsyncFd;

    // /usr/include/linux/rfkill.h
    // https://www.kernel.org/doc/html/latest/driver-api/rfkill.html#id5
    //
    // The preferred way to get rfkill events is by reading /dev/rfkill. We can
    // simply poll the file descriptor (using tokio's async reactor) and reading
    // one event per `read` system call.

    const RFKILL_OP_ADD: u8 = 0;
    const RFKILL_OP_DEL: u8 = 1;
    const RFKILL_OP_CHANGE: u8 = 2;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Default)]
    #[allow(non_camel_case_types)]
    pub struct rfkill_event {
        pub idx: u32,
        pub type_: u8,
        pub op: u8,
        pub soft: u8,
        pub hard: u8,
    }

    #[derive(Debug, Copy, Clone)]
    #[allow(dead_code)]
    pub struct DeviceState {
        pub type_: u8,
        pub soft: bool,
        pub hard: bool,
    }

    pub fn rfkill_updates()
    -> io::Result<impl Stream<Item = io::Result<HashMap<u32, DeviceState>>> + Unpin> {
        struct State {
            file: AsyncFd<fs::File>,
            devices: HashMap<u32, DeviceState>,
        }

        let file = fs::File::options()
            .read(true)
            .custom_flags(rustix::fs::OFlags::NONBLOCK.bits() as _)
            .open("/dev/rfkill")?;

        let state = State {
            file: AsyncFd::new(file)?,
            devices: HashMap::new(),
        };

        Ok(futures::stream::unfold(state, |mut state| {
            Box::pin(async {
                let mut guard = match state.file.readable().await {
                    Ok(guard) => guard,
                    Err(err) => {
                        return Some((Err(err), state));
                    }
                };
                let mut event = rfkill_event::default();
                // Read as many events as we can until it returns `EWOULDBLOCK`,
                // then yield new state after these updates.
                loop {
                    match read_event(guard.get_inner(), &mut event) {
                        Ok(()) => (),
                        Err(rustix::io::Errno::WOULDBLOCK) => {
                            break;
                        }
                        Err(err) => {
                            return Some((Err(err.into()), state));
                        }
                    };
                    match event.op {
                        RFKILL_OP_ADD | RFKILL_OP_CHANGE => {
                            state.devices.insert(
                                event.idx,
                                DeviceState {
                                    type_: event.type_,
                                    soft: event.soft != 0,
                                    hard: event.hard != 0,
                                },
                            );
                        }
                        RFKILL_OP_DEL => {
                            state.devices.remove(&event.idx);
                        }
                        _ => {}
                    }
                }
                guard.clear_ready();
                Some((Ok(state.devices.clone()), state))
            })
        }))
    }

    fn read_event(dev: &fs::File, event: &mut rfkill_event) -> rustix::io::Result<()> {
        let bytes = unsafe {
            slice::from_raw_parts_mut(event as *mut _ as *mut u8, mem::size_of::<rfkill_event>())
        };
        rustix::io::read(dev, bytes)?;
        Ok(())
    }
}
