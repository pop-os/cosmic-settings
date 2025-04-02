use std::any::TypeId;

use ashpd::desktop::location::{Location, LocationProxy};
use chrono::Datelike;
use cosmic::iced::{
    Subscription,
    futures::{SinkExt, StreamExt, channel::mpsc::Sender, future},
    stream,
};
use sunrise::sunrise_sunset;
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
        let (lat, long) = (loc.latitude(), loc.longitude());
        let now = chrono::Local::now();
        let date = now.date_naive();
        let (sunrise, sunset) = sunrise_sunset(lat, long, date.year(), date.month0(), date.day0());
        let now_in_seconds = now.timestamp();
        let daytime = now_in_seconds >= sunrise && now_in_seconds <= sunset;
        tx.send(daytime).await?;

        let sleep = if daytime {
            sunset - now_in_seconds
        } else if now_in_seconds < sunset {
            sunrise - now_in_seconds
        } else {
            let tmrw = now + chrono::Duration::days(1);
            let tmrw_date = tmrw.date_naive();
            let (tmrw_sunrise, _) = sunrise_sunset(
                lat,
                long,
                tmrw_date.year(),
                tmrw_date.month0(),
                tmrw_date.day0(),
            );
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
