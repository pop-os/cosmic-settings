// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use futures::{FutureExt, StreamExt};
use iced_futures::Subscription;
use std::collections::{BTreeMap, HashMap};

/// Values match `enum rfkill_type` in `/usr/include/linux/rfkill.h`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RfkillType {
    All,
    Wlan,
    Bluetooth,
    Uwb,
    Wimax,
    Wwan,
    Gps,
    Fm,
    Nfc,
    Unknown(u8),
}

impl From<u8> for RfkillType {
    fn from(type_: u8) -> Self {
        match type_ {
            0 => Self::All,
            1 => Self::Wlan,
            2 => Self::Bluetooth,
            3 => Self::Uwb,
            4 => Self::Wimax,
            5 => Self::Wwan,
            6 => Self::Gps,
            7 => Self::Fm,
            8 => Self::Nfc,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RfkillUpdate {
    pub airplane_mode: bool,
    pub blocked: BTreeMap<RfkillType, bool>,
}

pub fn subscription() -> iced_futures::Subscription<bool> {
    struct MyId;

    Subscription::run_with(std::any::TypeId::of::<MyId>(), |_| {
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
        .flatten_stream()
    })
}

pub fn rfkill_subscription() -> iced_futures::Subscription<RfkillUpdate> {
    struct MyId;

    Subscription::run_with(std::any::TypeId::of::<MyId>(), |_| {
        async {
            match rfkill::rfkill_updates() {
                Ok(updates) => updates.filter_map(|state| async {
                    match state {
                        Ok(state) => Some(RfkillUpdate {
                            airplane_mode: is_airplane_mode(&state),
                            blocked: blocked_by_type(&state),
                        }),
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
        .flatten_stream()
    })
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

// A kind is blocked only once every device of that kind is blocked.
fn blocked_by_type(rfkill_state: &HashMap<u32, rfkill::DeviceState>) -> BTreeMap<RfkillType, bool> {
    let mut blocked = BTreeMap::new();
    for device_state in rfkill_state.values() {
        let entry = blocked
            .entry(RfkillType::from(device_state.type_))
            .or_insert(true);
        *entry &= device_state.hard || device_state.soft;
    }
    blocked
}

mod rfkill {
    use futures::stream::Stream;
    use std::collections::HashMap;
    use std::os::unix::fs::OpenOptionsExt;
    use std::{fs, io, mem, slice};
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

#[cfg(test)]
mod tests {
    use super::*;

    const WLAN: u8 = 1;
    const BLUETOOTH: u8 = 2;

    fn devices(
        devices: impl IntoIterator<Item = (u32, u8, bool)>,
    ) -> HashMap<u32, rfkill::DeviceState> {
        devices
            .into_iter()
            .map(|(idx, type_, blocked)| {
                (
                    idx,
                    rfkill::DeviceState {
                        type_,
                        soft: blocked,
                        hard: false,
                    },
                )
            })
            .collect()
    }

    #[test]
    fn test_bluetooth_only_machine() {
        let blocked = blocked_by_type(&devices([(0, BLUETOOTH, false)]));

        assert_eq!(
            blocked.keys().copied().collect::<Vec<_>>(),
            [RfkillType::Bluetooth]
        );
        assert!(!blocked[&RfkillType::Bluetooth]);
    }

    #[test]
    fn test_every_kind_is_reported() {
        let blocked = blocked_by_type(&devices([(0, BLUETOOTH, true), (1, WLAN, false)]));

        assert!(blocked[&RfkillType::Bluetooth]);
        assert!(!blocked[&RfkillType::Wlan]);
    }

    #[test]
    fn test_two_adapters_of_one_kind() {
        let one_of_two = blocked_by_type(&devices([(0, BLUETOOTH, true), (1, BLUETOOTH, false)]));
        assert!(!one_of_two[&RfkillType::Bluetooth]);

        let both = blocked_by_type(&devices([(0, BLUETOOTH, true), (1, BLUETOOTH, true)]));
        assert!(both[&RfkillType::Bluetooth]);
    }

    #[test]
    fn test_hard_block_counts_as_blocked() {
        let hard = HashMap::from([(
            0,
            rfkill::DeviceState {
                type_: BLUETOOTH,
                soft: false,
                hard: true,
            },
        )]);

        assert!(blocked_by_type(&hard)[&RfkillType::Bluetooth]);
        assert!(is_airplane_mode(&hard));
    }

    #[test]
    fn test_airplane_mode_unchanged() {
        assert!(!is_airplane_mode(&devices([])));
        assert!(!is_airplane_mode(&devices([
            (0, BLUETOOTH, true),
            (1, WLAN, false)
        ])));
        assert!(is_airplane_mode(&devices([
            (0, BLUETOOTH, true),
            (1, WLAN, true)
        ])));
    }
}
