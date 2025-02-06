use futures::{pin_mut, Stream, StreamExt};

use super::super::{users, CompletedEvent, InitiatedEvent, PopUpgradeProxy, UiEvent};
use pop_upgrade_client::{DaemonStatus, ReleaseInfo};

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Found {
        is_current: bool,
        is_lts: bool,
        refresh: bool,
        status_failed: bool,
        reboot_ready: bool,
        upgrading_recovery: bool,
        urgent: bool,

        current: Option<Box<str>>,
        upgrade_text: Box<str>,

        upgrade: Option<ReleaseInfo>,
    },
    PermissionDenied,
}

pub fn scan<'a>(client: &'a PopUpgradeProxy<'static>) -> impl Stream<Item = UiEvent> + 'a {
    async_stream::stream! {
        yield UiEvent::Initiated(InitiatedEvent::Scanning);

        let mut current = None;
        let mut upgrade = None;
        let mut is_current = false;
        let mut is_lts = false;
        let mut status_failed = false;
        let mut urgent = false;

        if !users::is_admin() {
            yield UiEvent::Completed(CompletedEvent::Scan(ScanEvent::PermissionDenied));
            return;
        }

        let reboot_ready = pop_upgrade_client::reboot_is_ready();

        let upgrading_recovery =
            client.daemon_status_is(DaemonStatus::RecoveryUpgrade).await.unwrap_or(false);

        let upgrade_text: String = if !reboot_ready && pop_upgrade_client::upgrade_in_progress() {
            fl!("upgrade-downloading")
        } else {
            let devel = pop_upgrade_client::development_releases_enabled();
            let result = client.release_check(devel).await;
            match result {
                Ok((current_check, next, build, urgent_check, is_lts_check)) => {
                    current = Some(current_check.clone().into());
                    let urgent_check = if urgent_check > -1 { Some(urgent_check as u16) } else { None };

                    match client.recovery_version().await {
                        Ok((rversion, rbuild)) => {
                            urgent = urgent_check.map_or(false, |urgent| {
                                rversion != current_check
                                    || rbuild < 0
                                    || (rbuild as u16) < urgent
                            }) || rversion != current_check;
                        }
                        Err(_) => {
                            urgent = urgent_check.unwrap_or(0) > 0;
                        }
                    }



                    is_lts = is_lts_check;
                    if devel || build >= 0 {
                        tracing::info!(
                            "{}",
                            fl!(
                                "upgrade-from-to",
                                current = (&*current_check),
                                next = (&*next)
                            )
                        );

                        let upgrade_text = if reboot_ready {
                            fl!("upgrade-ready", version = (&*next))
                        } else {
                            fl!("upgrade-available", version = (&*next))
                        };

                        upgrade = Some(pop_upgrade_client::ReleaseInfo {
                            current: current_check.into(), next: next.into(), build, urgent: urgent_check, is_lts
                        });
                        upgrade_text
                    } else {
                        status_failed = true;
                        match build {
                            -1 => fl!("error-build-status"),
                            -2 | -4 => {
                                is_current = true;
                                status_failed = false;
                                fl!("release-current")
                            }
                            -3 => fl!("error-connection"),
                            _ => fl!("error-unknown-status"),
                        }
                    }
                }
                Err(why) => {
                    status_failed = true;
                    let msg = fl!("error-update-check");
                    tracing::error!("{}: {}", msg, why);
                    msg
                }
            }
        };

        yield UiEvent::Completed(CompletedEvent::Scan(ScanEvent::Found {
            current,
            is_current,
            is_lts,
            reboot_ready,
            refresh: pop_upgrade_client::recovery_exists().unwrap_or(false),
            status_failed,
            upgrade_text: Box::from(upgrade_text),
            upgrade,
            upgrading_recovery,
            urgent,
        }));

        if upgrading_recovery {
            let stream = super::recovery::upgrade_listen(client);
            pin_mut!(stream);

            while let Some(event) = stream.next().await {
                yield event;
            }
        }
    }
}
