use std::any::TypeId;

use ashpd::desktop::location::{Location, LocationProxy};
use cosmic::iced::{
    Subscription,
    futures::{SinkExt, StreamExt, channel::mpsc::Sender, future},
    stream,
};
use sunrise::{Coordinates, SolarDay, SolarEvent};
use tokio::select;

pub fn daytime() -> cosmic::iced::Subscription<bool> {
    struct Sunset;
    Subscription::run_with_id(
        TypeId::of::<Sunset>(),
        stream::channel(2, |tx| async {
            if let Err(err) = inner(tx).await {
                tracing::error!("Sunset subscription error: {:?}", err);
            }
            future::pending().await
        }),
    )
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
        let now = chrono::Local::now();
        let date = now.date_naive();
        let now_in_seconds = now.timestamp();
        let current_solar_day = SolarDay::new(coord, date);
        let sunrise = current_solar_day
            .event_time(SolarEvent::Sunrise)
            .timestamp();
        let sunset = current_solar_day.event_time(SolarEvent::Sunset).timestamp();
        let daytime = now_in_seconds >= sunrise && now_in_seconds <= sunset;
        tx.send(daytime).await?;

        let sleep = if daytime {
            sunset - now_in_seconds
        } else if now_in_seconds < sunset {
            sunrise - now_in_seconds
        } else {
            let tmrw = now + chrono::Duration::days(1);
            let tmrw_sunrise = SolarDay::new(coord, tmrw.date_naive())
                .event_time(SolarEvent::Sunrise)
                .timestamp();
            tmrw_sunrise - now_in_seconds
        };
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
