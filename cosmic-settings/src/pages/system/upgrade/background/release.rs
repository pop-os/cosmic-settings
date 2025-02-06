use apt_cmd::AptUpgradeEvent;
use futures::{pin_mut, Stream, StreamExt};
use pop_upgrade_client::{DaemonStatus, ReleaseInfo, Status};
use std::sync::Arc;

use crate::pages::system::upgrade::{
    errors::UiError, CompletedEvent, InitiatedEvent, OsRecoveryEvent, OsUpgradeEvent,
    PopUpgradeProxy, Progress, ProgressEvent, Signal, UiEvent,
};

pub fn download<'a>(
    client: &'a PopUpgradeProxy<'static>,
    info: &'a ReleaseInfo,
) -> impl Stream<Item = UiEvent> + 'a {
    tracing::info!("downloading updates for {}", info.next);

    async_stream::stream! {
        let mut should_continue = false;

        {
            let messages = update(client, &mut should_continue);
            pin_mut!(messages);

            while let Some(message) = messages.next().await {
                yield message;
            }
        }

        if !should_continue {
            return;
        }

        let &ReleaseInfo {
            ref current,
            ref next,
            ..
        } = &info;

        yield UiEvent::Initiated(InitiatedEvent::Download(next.clone()));

        if let Err(why) = client.release_upgrade(1, current, next).await {
            yield UiEvent::Error(Arc::new(UiError::Upgrade(why.into())));
            return;
        }

        let mut result = Ok(());
        let signals = client.watch_signals();
        pin_mut!(signals);

        while let Some(signal) = signals.next().await {
            match signal {
                Signal::PackageFetched(status) => {
                    yield UiEvent::Progress(ProgressEvent::Fetching(
                        status.completed.into(),
                        status.total.into(),
                    ));
                }

                Signal::PackageUpgrade(event) => {
                    match AptUpgradeEvent::from_dbus_map(event.into_iter()) {
                        Ok(AptUpgradeEvent::Progress { percent }) => {
                            yield UiEvent::Progress(ProgressEvent::Updates(percent));
                        }
                        Ok(AptUpgradeEvent::WaitingOnLock) => {
                            yield UiEvent::WaitingOnLock;
                        }
                        _ => (),
                    }
                }

                Signal::RecoveryDownloadProgress(Progress { progress, total }) => {
                    eprintln!("Progress {}/{}", progress, total);
                    yield UiEvent::Progress(ProgressEvent::Recovery(progress, total));
                }

                Signal::RecoveryResult(status) => yield if status.status == 0 {
                    UiEvent::Completed(CompletedEvent::Recovery(false))
                } else {
                    UiEvent::Recovery(OsRecoveryEvent::Failed)
                },

                Signal::RecoveryEvent(event) => {
                    yield UiEvent::Recovery(OsRecoveryEvent::Event(event));
                }

                Signal::ReleaseEvent(event) => {
                    yield UiEvent::Upgrade(OsUpgradeEvent::Event(event));
                }

                Signal::ReleaseResult(status) => {
                    if status.status != 0 {
                        result = Err(status.why);
                    }

                    break;
                }
                _ => (),
            }
        }

        yield match result {
            Ok(()) => UiEvent::Completed(CompletedEvent::Download),
            Err(why) => UiEvent::Error(Arc::new(UiError::Upgrade(why.into()))),
        };
    }
}

pub fn update<'a>(
    client: &'a PopUpgradeProxy<'static>,
    should_continue: &'a mut bool,
) -> impl Stream<Item = UiEvent> + 'a {
    tracing::info!("checking if updates are required");
    async_stream::stream! {
        let (updates_available, completed, total) = match client.fetch_updates(Vec::new(), false).await {
            Ok(updates) => updates,
            Err(why) => {
                yield UiEvent::Error(Arc::new(UiError::Updates(why.into())));
                *should_continue = false;
                return;
            }
        };

        if updates_available {
            yield UiEvent::Progress(ProgressEvent::Fetching(
                completed.into(),
                total.into(),
            ));

            let inactivity_timeout = client.inactivity_timeout();
            let signals = client.watch_signals();
            pin_mut!(inactivity_timeout);
            pin_mut!(signals);

            let mut error = None;

            loop {
                tokio::select! {
                    _ = &mut inactivity_timeout => {
                        if let Ok((status, why)) = client.fetch_updates_status().await {
                            yield super::status_changed(Status { status, why: why.into() }, DaemonStatus::FetchingPackages);
                        }

                        tracing::error!("breaking connection to pop-upgrade daemon due to inactivity");
                        break
                    }

                    signal = signals.next() => {
                        let Some(signal) = signal else {
                            break
                        };

                        match signal {
                            Signal::PackageFetchResult(status) => {
                                if status.status != 0 {
                                    error = Some(status.why);
                                }

                                break
                            }
                            Signal::PackageFetched(status) => {
                                yield UiEvent::Progress(ProgressEvent::Fetching(
                                    status.completed.into(),
                                    status.total.into(),
                                ));
                            }
                            Signal::PackageUpgrade(event) => {
                                match AptUpgradeEvent::from_dbus_map(event.into_iter()) {
                                    Ok(AptUpgradeEvent::Progress { percent }) => {
                                        yield UiEvent::Progress(ProgressEvent::Updates(percent));
                                    }
                                    Ok(AptUpgradeEvent::WaitingOnLock) => {
                                        yield UiEvent::WaitingOnLock;
                                    }
                                    _ => (),
                                }
                            }
                            Signal::ReleaseEvent(event) => {
                                yield UiEvent::Upgrade(OsUpgradeEvent::Event(event));
                            }
                            _ => (),
                        }
                    }
                }
            }

            // let _ = client.event_listen(
            //     Client::fetch_updates_status,
            //     |status| status_changed(send, status, DaemonStatus::FetchingPackages),

            if let Some(why) = error.take() {
                yield UiEvent::Error(Arc::new(UiError::Updates(why.into())));
                *should_continue = false;
                return;
            }
        }

        *should_continue = true;
    }
}
