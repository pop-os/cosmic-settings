pub mod recovery;
pub mod release;
pub mod scan;

// use self::scan::scan;

use futures::{pin_mut, Stream, StreamExt};
use num_traits::FromPrimitive;
use pop_upgrade_client::{DaemonStatus, DismissEvent, RefreshOp, ReleaseInfo, Status};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::pages::system::upgrade::{
    errors::UiError, CompletedEvent, InitiatedEvent, OsUpgradeEvent,
};

use super::{PopUpgradeProxy, UiEvent};

/// Events sent to this widget's background thread.
#[derive(Debug)]
pub enum BackgroundEvent {
    DownloadUpgrade(ReleaseInfo),
    Finalize,
    GetStatus(DaemonStatus),
    // IsActive(SyncSender<bool>),
    DismissNotification(bool),
    #[allow(clippy::upper_case_acronyms)]
    RefreshOS,
    Reset,
    Scan,
    Shutdown,
    UpdateRecovery(Box<str>),
}

pub fn run(
    mut client: PopUpgradeProxy<'static>,
    conn: zbus::Connection,
    mut events: mpsc::Receiver<BackgroundEvent>,
) -> impl Stream<Item = UiEvent> {
    async_stream::stream! {
        if std::env::var_os("S76_TEST").is_none() {
            tracing::info!("Checking for updates to daemon");
            if client.update_check().await.unwrap_or(0) == 1 {
                yield UiEvent::Updating;

                let file = std::path::Path::new(pop_upgrade_client::RESTART_SCHEDULED);
                while file.exists() {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }

                if let Ok(c) = PopUpgradeProxy::new(&conn).await {
                    client = c;
                }
            }
        }

        yield UiEvent::Updated;

        while let Some(event) = events.recv().await {
            tracing::trace!("received background event: {:?}", event);
            match event {
                BackgroundEvent::DismissNotification(dismiss) => {
                    let dismiss_event = if dismiss {
                        DismissEvent::ByUser
                    } else {
                        DismissEvent::Unset
                    };

                    yield match client.dismiss_notification(dismiss_event as u8).await {
                        Ok(dismissed) => UiEvent::Upgrade(OsUpgradeEvent::Dismissed(dismissed)),
                        Err(why) => {
                            UiEvent::Error(Arc::new(UiError::Dismiss(dismiss, why.into())))
                        }
                    };
                }

                BackgroundEvent::DownloadUpgrade(info) => {
                    let stream = self::release::download(&client, &info);
                    pin_mut!(stream);
                    while let Some(event) = stream.next().await {
                        yield event;
                    }
                }

                BackgroundEvent::Finalize => match client.release_upgrade_finalize().await {
                    Ok(()) => {
                        // reboot()
                    },
                    Err(why) => yield UiEvent::Error(Arc::new(UiError::Finalize(why))),
                },

                BackgroundEvent::GetStatus(from) => {
                    if let Some(event) = get_status(&client, from).await {
                        yield event;
                    }
                }

                // BackgroundEvent::IsActive(tx) => {
                //     let _ = tx.send(client.status().is_ok());
                // }

                BackgroundEvent::RefreshOS => {
                    let stream = refresh_os(&client);
                    pin_mut!(stream);

                    while let Some(event) = stream.next().await {
                        yield event;
                    }
                }

                BackgroundEvent::Reset => {
                    yield match client.reset().await {
                        Ok(()) => UiEvent::Upgrade(OsUpgradeEvent::Cancelled),
                        Err(why) => UiEvent::Error(Arc::new(UiError::Cancel(why))),
                    };
                }

                BackgroundEvent::Scan => {
                    let stream = scan::scan(&client);
                    pin_mut!(stream);

                    while let Some(event) = stream.next().await {
                        yield event;
                    }
                }

                BackgroundEvent::Shutdown => {
                    yield UiEvent::Shutdown;
                    break;
                }

                BackgroundEvent::UpdateRecovery(version) => {
                    let stream = recovery::upgrade(&client, version.into());
                    pin_mut!(stream);

                    while let Some(event) = stream.next().await {
                        yield event;
                    }
                }
            }
        }
    }
}

async fn get_status(client: &PopUpgradeProxy<'_>, from: DaemonStatus) -> Option<UiEvent> {
    match from {
        DaemonStatus::RecoveryUpgrade => Some(status_recovery_upgrade(client).await),
        DaemonStatus::ReleaseUpgrade => Some(status_release_upgrade(client).await),
        _ => None,
    }
}

async fn status_recovery_upgrade(client: &PopUpgradeProxy<'_>) -> UiEvent {
    match client.recovery_upgrade_release_status().await {
        Ok((status, why)) => {
            if status == 0 {
                UiEvent::Completed(CompletedEvent::Recovery(true))
            } else {
                UiEvent::Error(Arc::new(UiError::Recovery(why.into())))
            }
        }
        Err(why) => UiEvent::Error(Arc::new(UiError::Recovery(why.into()))),
    }
}

async fn status_release_upgrade(client: &PopUpgradeProxy<'_>) -> UiEvent {
    match client.release_upgrade_status().await {
        Ok((status, why)) => {
            if status == 0 {
                UiEvent::Completed(CompletedEvent::Download)
            } else {
                UiEvent::Error(Arc::new(UiError::Upgrade(why.into())))
            }
        }
        Err(why) => UiEvent::Error(Arc::new(UiError::Upgrade(why.into()))),
    }
}

fn refresh_os<'a>(client: &'a PopUpgradeProxy<'_>) -> impl Stream<Item = UiEvent> + 'a {
    async_stream::stream! {
        yield UiEvent::Initiated(InitiatedEvent::Refresh);

        if let Err(why) = client.refresh_os(RefreshOp::Enable as u8).await {
            yield UiEvent::Error(Arc::new(UiError::Refresh(why.into())));
            return;
        }

        yield UiEvent::Completed(CompletedEvent::Refresh);
    }
}

fn status_changed(new_status: Status, expected: DaemonStatus) -> UiEvent {
    let status = DaemonStatus::from_u8(new_status.status).expect("unknown daemon status value");
    UiEvent::StatusChanged(expected, status, new_status.why)
}
