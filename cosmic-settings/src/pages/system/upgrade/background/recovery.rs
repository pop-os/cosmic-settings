use std::sync::Arc;

use super::super::{
    errors::UiError, CompletedEvent, InitiatedEvent, OsRecoveryEvent, Progress, ProgressEvent,
    UiEvent,
};

use crate::pages::system::upgrade::dbus::{PopUpgradeProxy, Signal};
use futures::{Stream, StreamExt};
use pop_upgrade_client::{DaemonStatus, Status};

pub fn upgrade<'a>(
    client: &'a PopUpgradeProxy<'static>,
    version: String,
) -> impl Stream<Item = UiEvent> + 'a {
    async_stream::stream! {
        yield UiEvent::Initiated(InitiatedEvent::Recovery);
        let arch = "nvidia".into();

        if let Err(why) = client.recovery_upgrade_release(version, arch, 0).await {
            yield UiEvent::Error(Arc::new(UiError::Recovery(why.into())));
            return;
        }

        let signals = upgrade_listen(client);

        futures::pin_mut!(signals);

        while let Some(message) = signals.next().await {
            yield message;
        }
    }
}

pub fn upgrade_listen<'a>(
    client: &'a PopUpgradeProxy<'static>,
) -> impl Stream<Item = UiEvent> + 'a {
    async_stream::stream! {
        let inactivity_timeout = client.inactivity_timeout();
        let signals = client.watch_signals();
        futures::pin_mut!(inactivity_timeout);
        futures::pin_mut!(signals);

        loop {
            tokio::select! {
                _ = &mut inactivity_timeout => {
                    if let Ok((status, why)) = client.recovery_upgrade_release_status().await {
                        yield super::status_changed(Status { status, why: why.into() }, DaemonStatus::RecoveryUpgrade);
                    }

                    tracing::error!("breaking connection to pop-upgrade daemon due to inactivity");
                    break
                }

                signal = signals.next() => {
                    let Some(signal) = signal else {
                        break
                    };

                    match signal {
                        Signal::RecoveryDownloadProgress(Progress { progress, total }) => {
                            yield UiEvent::Progress(ProgressEvent::Recovery(progress, total));
                        }

                        Signal::RecoveryEvent(event) => {
                            yield UiEvent::Recovery(OsRecoveryEvent::Event(event));
                        }

                        Signal::RecoveryResult(status) => {
                            if status.status != 0 {
                                yield UiEvent::Error(Arc::new(UiError::Recovery(status.why.into())));
                                return;
                            }

                            break
                        }
                        _ => (),
                    }
                }
            }
        }

        yield UiEvent::Completed(CompletedEvent::Recovery(true));
    }
}
