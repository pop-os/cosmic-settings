use cosmic::{
    iced::subscription,
    iced_futures::futures::{self, SinkExt},
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fmt::Debug;
use std::hash::Hash;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

#[derive(Debug)]
pub enum State {
    Ready,
    Waiting {
        watcher: RecommendedWatcher,
        rx: UnboundedReceiver<notify::Result<Event>>,
    },
    Finished,
}

#[derive(Debug, Clone, Copy)]
pub enum DesktopFileEvent {
    Changed,
}

pub fn desktop_files<I: 'static + Hash + Copy + Send + Sync + Debug>(
    id: I,
) -> cosmic::iced::Subscription<DesktopFileEvent> {
    subscription::channel(id, 50, move |mut output| async move {
        let mut state = State::Ready;

        loop {
            state = start_watching(state, &mut output).await;
        }
    })
}

async fn start_watching(
    state: State,
    output: &mut futures::channel::mpsc::Sender<DesktopFileEvent>,
) -> State {
    match state {
        State::Ready => {
            let paths = freedesktop_desktop_entry::default_paths();
            // TODO log errors
            if let Ok((mut watcher, rx)) = async_watcher() {
                for path in paths {
                    let _ = watcher.watch(path.as_ref(), RecursiveMode::Recursive);
                }

                _ = output.send(DesktopFileEvent::Changed).await;
                State::Waiting { watcher, rx }
            } else {
                State::Finished
            }
        }
        State::Waiting { watcher, rx } => {
            if let Some(rx) = async_watch(rx).await {
                _ = output.send(DesktopFileEvent::Changed).await;
                State::Waiting { watcher, rx }
            } else {
                State::Finished
            }
        }
        State::Finished => cosmic::iced::futures::future::pending().await,
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, UnboundedReceiver<notify::Result<Event>>)>
{
    let (tx, rx) = unbounded_channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            _ = tx.send(res);
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch(
    mut rx: UnboundedReceiver<notify::Result<Event>>,
) -> Option<UnboundedReceiver<notify::Result<Event>>> {
    // TODO log errors
    if let Some(res) = rx.recv().await {
        match res {
            Ok(_) => return Some(rx),
            Err(_) => return None,
        }
    }

    None
}
