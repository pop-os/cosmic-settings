use cosmic::{
    iced::{Subscription, stream},
    iced_futures::futures::{self, SinkExt},
};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fmt::Debug;
use std::hash::Hash;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Changed,
}

pub fn desktop_files<I: 'static + Hash + Copy + Send + Sync + Debug>(
    id: I,
) -> cosmic::iced::Subscription<Event> {
    Subscription::run_with_id(
        id,
        stream::channel(1, move |mut output| async move {
            let handle = tokio::runtime::Handle::current();
            let (tx, mut rx) = mpsc::channel(4);
            let mut last_update = std::time::Instant::now();

            // Automatically select the best implementation for your platform.
            // You can also access each implementation directly e.g. INotifyWatcher.
            let watcher = RecommendedWatcher::new(
                move |res: Result<notify::Event, notify::Error>| {
                    if let Ok(event) = res {
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                let now = std::time::Instant::now();
                                if now.duration_since(last_update).as_secs() > 3 {
                                    _ = handle.block_on(tx.send(()));
                                    last_update = now;
                                }
                            }

                            _ => (),
                        }
                    }
                },
                Config::default(),
            );

            if let Ok(mut watcher) = watcher {
                for path in cosmic::desktop::fde::default_paths() {
                    let _ = watcher.watch(path.as_ref(), RecursiveMode::Recursive);
                }

                while rx.recv().await.is_some() {
                    _ = output.send(Event::Changed).await;
                }
            }

            futures::future::pending().await
        }),
    )
}
