// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

pub mod device {
    use futures::{FutureExt, Stream, StreamExt};
    use iced_futures::Subscription;
    use std::{fmt::Debug, hash::Hash};
    use upower_dbus::{BatteryType, DeviceProxy, UPowerProxy};

    pub fn device_subscription<I: 'static + Hash + Copy + Send + Sync + Debug>(
        id: I,
    ) -> iced_futures::Subscription<DeviceDbusEvent> {
        Subscription::run_with_id(
            id,
            async move {
                match events().await {
                    Ok(stream) => stream,
                    Err(err) => {
                        log::error!("Error getting events from upower device: {}", err);
                        futures::future::pending().await
                    }
                }
            }
            .flatten_stream(),
        )
    }

    async fn display_device() -> zbus::Result<(UPowerProxy<'static>, DeviceProxy<'static>)> {
        let connection = zbus::Connection::system().await?;
        let upower: UPowerProxy<'_> = UPowerProxy::new(&connection).await?;
        let device_path = upower.get_display_device().await?;
        Ok((upower, device_path))
    }

    async fn events() -> zbus::Result<impl Stream<Item = DeviceDbusEvent>> {
        let (upower, device) = display_device().await?;
        let devices = upower.enumerate_devices().await?;
        let mut has_battery = false;
        for device in devices {
            let Ok(d) = DeviceProxy::builder(upower.inner().connection()).path(device) else {
                continue;
            };
            let Ok(d) = d.build().await else {
                continue;
            };
            if d.type_().await == Ok(BatteryType::Battery)
                && d.power_supply().await.unwrap_or_default()
            {
                has_battery = true;
                break;
            }
        }

        let initial = futures::stream::iter(if has_battery {
            None
        } else {
            Some(DeviceDbusEvent::NoBattery)
        });

        let stream = futures::stream_select!(
            upower.receive_on_battery_changed().await.map(|_| ()),
            device.receive_percentage_changed().await.map(|_| ()),
            device.receive_time_to_empty_changed().await.map(|_| ()),
        );

        Ok(initial.chain(stream.map(move |_| {
            DeviceDbusEvent::Update {
                on_battery: upower
                    .cached_on_battery()
                    .unwrap_or_default()
                    .unwrap_or_default(),
                percent: device
                    .cached_percentage()
                    .unwrap_or_default()
                    .unwrap_or_default(),
                time_to_empty: device
                    .cached_time_to_empty()
                    .unwrap_or_default()
                    .unwrap_or_default(),
            }
        })))
    }

    #[derive(Debug, Clone)]
    pub enum DeviceDbusEvent {
        NoBattery,
        Update {
            on_battery: bool,
            percent: f64,
            time_to_empty: i64,
        },
    }
}

pub mod kbdbacklight {
    // TODO Test how this handles upower starting after applet does, if it ever is
    // started or restarted.

    use futures::{FutureExt, Stream, StreamExt};
    use iced_futures::Subscription;
    use std::{fmt::Debug, hash::Hash};
    use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
    use tokio_stream::wrappers::UnboundedReceiverStream;
    use upower_dbus::{BrightnessChanged, KbdBacklightProxy};

    pub fn kbd_backlight_subscription<I: 'static + Hash + Copy + Send + Sync + Debug>(
        id: I,
    ) -> iced_futures::Subscription<KeyboardBacklightUpdate> {
        Subscription::run_with_id(
            id,
            async move {
                match events().await {
                    Ok(stream) => stream,
                    Err(err) => {
                        log::error!("Error listening to KbdBacklight: {}", err);
                        futures::future::pending().await
                    }
                }
            }
            .flatten_stream(),
        )
    }

    enum Event {
        BrightnessChanged(BrightnessChanged),
        Request(KeyboardBacklightRequest),
    }

    async fn events() -> zbus::Result<impl Stream<Item = KeyboardBacklightUpdate>> {
        let conn = zbus::Connection::system().await?;
        let kbd_proxy = KbdBacklightProxy::builder(&conn).build().await?;
        let (tx, rx) = unbounded_channel();

        let max_brightness = kbd_proxy.get_max_brightness().await?;
        let brightness = kbd_proxy.get_brightness().await?;

        let brightness_changed_stream = kbd_proxy.receive_brightness_changed().await?;

        let initial = futures::stream::iter([
            KeyboardBacklightUpdate::Sender(tx),
            KeyboardBacklightUpdate::MaxBrightness(max_brightness),
            KeyboardBacklightUpdate::Brightness(brightness),
        ]);
        let stream = futures::stream::select(
            UnboundedReceiverStream::new(rx).map(Event::Request),
            brightness_changed_stream.map(Event::BrightnessChanged),
        );
        Ok(initial.chain(stream.filter_map(move |event| {
            let kbd_proxy = kbd_proxy.clone();
            async move {
                match event {
                    Event::BrightnessChanged(changed) => {
                        if let Ok(args) = changed.args() {
                            Some(KeyboardBacklightUpdate::Brightness(*args.value()))
                        } else {
                            None
                        }
                    }
                    Event::Request(req) => match req {
                        KeyboardBacklightRequest::Get => {
                            if let Ok(brightness) = kbd_proxy.get_brightness().await {
                                Some(KeyboardBacklightUpdate::Brightness(brightness))
                            } else {
                                None
                            }
                        }
                        KeyboardBacklightRequest::Set(value) => {
                            let _ = kbd_proxy.set_brightness(value).await;
                            None
                        }
                    },
                }
            }
        })))
    }

    #[derive(Debug, Clone)]
    pub enum KeyboardBacklightUpdate {
        Sender(UnboundedSender<KeyboardBacklightRequest>),
        Brightness(i32),
        MaxBrightness(i32),
    }

    #[derive(Debug, Clone)]
    pub enum KeyboardBacklightRequest {
        Get,
        Set(i32),
    }
}
