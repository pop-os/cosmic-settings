use std::any::TypeId;

use ashpd::desktop::location::{Location, LocationProxy};
use chrono::NaiveDate;
use cosmic::iced::{
    Subscription,
    futures::{SinkExt, StreamExt, channel::mpsc::Sender, future},
    stream,
};
use sunrise::{Coordinates, SolarDay, SolarEvent};
use tokio::select;

pub fn daytime() -> cosmic::iced::Subscription<bool> {
    struct Sunset;
    Subscription::run_with(TypeId::of::<Sunset>(), |_| {
        stream::channel(2, |tx: Sender<bool>| async {
            if let Err(err) = inner(tx).await {
                tracing::error!("Sunset subscription error: {:?}", err);
            }
            future::pending().await
        })
    })
}

enum Event {
    Daytime,
    LocationUpdated(Location),
}

async fn inner(mut tx: Sender<bool>) -> anyhow::Result<()> {
    let location_proxy = LocationProxy::new().await?;
    let mut updates = location_proxy.receive_location_updated().await?;

    let mut next = updates.next().await.map(Event::LocationUpdated);
    let mut loc = None;

    while let Some(e) = next {
        match e {
            Event::LocationUpdated(l) => {
                loc = Some(l);
            }
            Event::Daytime => {}
        };
        let Some(loc) = loc.as_ref() else {
            break;
        };

        let coord = Coordinates::new(loc.latitude(), loc.longitude()).unwrap();
        let now = jiff::Zoned::now();
        let now_in_seconds = now.timestamp().as_second();
        // roughly matches the dates of the spring and autumn equinoxes
        let northern_tilt = (79..=266).contains(&now.day_of_year());
        let is_north = coord.lat() > 0.0;
        // TODO: remove chrono if sunrise adds support for jiff - https://github.com/nathan-osman/rust-sunrise/pull/20
        let date = NaiveDate::from_ymd_opt(now.year() as i32, now.month() as u32, now.day() as u32)
            .expect("jiff date is valid");
        let current_solar_day = SolarDay::new(coord, date);
        let sunrise = current_solar_day
            .event_time(SolarEvent::Sunrise)
            .map(|s| s.timestamp());
        let sunset = current_solar_day
            .event_time(SolarEvent::Sunset)
            .map(|s| s.timestamp());
        let daytime = match (sunrise, sunset) {
            (Some(sunrise), Some(sunset)) => now_in_seconds >= sunrise && now_in_seconds <= sunset,
            // transition into polar day
            (Some(sunrise), None) => now_in_seconds >= sunrise,
            // transition out of polar day
            (None, Some(sunset)) => now_in_seconds <= sunset,
            // polar day
            (None, None) => is_north == northern_tilt,
        };
        tx.send(daytime).await?;

        let next_event = if daytime {
            sunset
        } else if now_in_seconds < sunrise.unwrap_or(0) {
            sunrise
        } else {
            let tmrw = now.checked_add(jiff::Span::new().days(1))?;
            let tmrw_sunrise =
                NaiveDate::from_ymd_opt(tmrw.year() as i32, tmrw.month() as u32, tmrw.day() as u32)
                    .expect("jiff date is valid");

            SolarDay::new(coord, tmrw_sunrise)
                .event_time(SolarEvent::Sunrise)
                .map(|s| s.timestamp())
        };
        let sleep = next_event
            .map(|ts| (ts - now_in_seconds).max(60))
            .unwrap_or(3600);
        next = select! {
            () = tokio::time::sleep(tokio::time::Duration::from_secs(sleep as u64)) => {
                Some(Event::Daytime)
            },
            l = updates.next() => if let Some(l) = l {
                Some(Event::LocationUpdated(l))
            } else {
                break;
            }
        };
    }
    Err(anyhow::anyhow!("Location updates ended unexpectedly."))
}
