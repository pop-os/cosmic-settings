mod background;
mod dbus;
use dbus::*;
mod errors;
pub mod users;

use background::scan::ScanEvent;
use background::BackgroundEvent;
use errors::UiError;

use cosmic::{
    widget::{self, button},
    Apply, Element,
};
use cosmic_settings_page::{self as page, section, Section};
use fomat_macros::fomat;
use futures::StreamExt;
use pop_upgrade_client::{DaemonStatus, RecoveryEvent, ReleaseInfo, UpgradeEvent};
use slotmap::SlotMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::mpsc;

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    bg_context: Option<(mpsc::Sender<BackgroundEvent>, tokio::task::JoinHandle<()>)>,

    /// The control for the state of the upgrade button.
    upgrade: SectionOption,
    /// The control for the state of the recovery button.
    recovery: SectionOption,
    /// The control for the state of the refresh button.
    refresh: SectionOption,
    /// Set when an error occurred that hasn't been dismissed.
    error_message: Option<Box<str>>,
    /// TODO: is this different from `current_release`?
    upgrading_from: Box<str>,
    /// The release that the current system is on.
    current_release: Box<str>,
    /// The release that the system is upgrading to.
    next_release: Box<str>,
    /// Label to display while scanning for a new release.
    scanning_label: Box<str>,
    /// Upgrade progress, from 0 to 100.
    upgrade_progress: u64,
    /// Download progress of recovery image in MBytes.
    recovery_progress: u64,
    /// Total size of recovery image in Mbytes.
    recovery_total: u64,
    /// Scanning for new releases.
    scanning: bool,
    /// The system is on a LTS release
    is_lts: bool,
    /// Fetching the latest release packages
    fetching_release: bool,
    /// Set to true if a recovery upgrade is required.
    recovery_urgent: bool,
    /// Upgrading the recovery partition
    upgrading_recovery: bool,
    /// A possible upgrade was found
    upgrade_found: bool,
    /// Refresh option found
    refresh_found: bool,
    /// Set to true when OS release updates have been downloaded, and are ready to commence.
    upgrade_downloaded: bool,
    /// The upgrade is ready to commence on the next boot
    /// A reboot dialog should be presented when this is true.
    reboot_ready: bool,

    upgrade_text: Box<str>,

    release_info: Option<ReleaseInfo>,
}

#[derive(Debug)]
struct SectionOption {
    pub state: OptionState,
    pub label: Box<str>,
    pub sublabel: Option<Box<str>>,
    pub sensitive: bool,
}

impl Default for SectionOption {
    fn default() -> Self {
        Self {
            state: OptionState::default(),
            label: Box::default(),
            sublabel: None,
            sensitive: true,
        }
    }
}

/// Events received for the UI to handle.
#[derive(Debug, Clone)]
pub enum UiEvent {
    Completed(CompletedEvent),
    Error(Arc<UiError>),
    Initiated(InitiatedEvent),
    Progress(ProgressEvent),
    Recovery(OsRecoveryEvent),
    Shutdown,
    StatusChanged(DaemonStatus, DaemonStatus, Box<str>),
    Updated,
    Updating,
    Upgrade(OsUpgradeEvent),
    WaitingOnLock,
}

/// Data for tracking progress of an action.
#[derive(Clone, Debug)]
pub struct Progress {
    pub progress: u64,
    pub total: u64,
}

#[derive(Debug, Default, Clone)]
enum OptionState {
    #[default]
    Hide,
    HideControls,
    ShowButton,
    ShowProgress,
}

#[derive(Debug, Clone)]
pub enum OsUpgradeEvent {
    Cancelled,
    Dialog,
    Dismissed(bool),
    Event(UpgradeEvent),
    Notification,
    Upgrade,
}

#[derive(Debug, Clone)]
pub enum OsRecoveryEvent {
    Event(RecoveryEvent),
    Failed,
    Refresh,
    Update,
}

#[derive(Debug, Clone)]
pub enum InitiatedEvent {
    Download(Box<str>),
    Recovery,
    Refresh,
    Scanning,
}

#[derive(Debug, Clone)]
pub enum CompletedEvent {
    Download,
    Recovery(bool),
    Refresh,
    Scan(ScanEvent),
}

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    Fetching(u64, u64),
    Recovery(u64, u64),
    Updates(u8),
}

#[repr(u8)]
pub enum Event {
    NotUpgrading = 0,
    Upgrading = 1,
    UpgradeReady = 2,
}

#[derive(Clone, Debug)]
pub enum Message {
    AutomaticUpdates(bool),
    Background(UiEvent),
    RecoveryUpdate,
    RefreshOS,
    Scan,
    Shutdown,
    UpgradeDownload,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Upgrade(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Upgrade(message)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(os_upgrade()),
            sections.insert(os_recovery()),
        ])
    }

    fn info(&self) -> page::Info {
        // TODO:
        page::Info::new("upgrade", "system-users-symbolic")
            .title(fl!("os-update"))
            .description(fl!("os-update", "desc"))
    }

    fn on_enter(
        &mut self,
        sender: mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Task<crate::pages::Message> {
        if let Some((_sender, upgrade_thread)) = self.bg_context.take() {
            upgrade_thread.abort();
        }

        let (tx, rx) = mpsc::channel(1);

        let task_handle = tokio::task::spawn(async move {
            let Ok(conn) = zbus::Connection::system().await else {
                return;
            };

            let Ok(client) = PopUpgradeProxy::new(&conn).await else {
                return;
            };

            let messages = background::run(client, conn, rx);
            futures::pin_mut!(messages);

            while let Some(message) = messages.next().await {
                _ = sender
                    .send(crate::pages::Message::Upgrade(Message::Background(message)))
                    .await;
            }
        });

        self.bg_context = Some((tx, task_handle));

        cosmic::Task::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        tracing::debug!("{:?}", message);
        match message {
            Message::UpgradeDownload => {
                if let Some(release_info) = self.release_info.clone() {
                    self.send_bg(BackgroundEvent::DownloadUpgrade(release_info));
                }
            }

            Message::RecoveryUpdate => {
                self.send_bg(BackgroundEvent::UpdateRecovery(
                    self.current_release.clone(),
                ));
            }

            Message::RefreshOS => {
                self.send_bg(BackgroundEvent::RefreshOS);
            }

            Message::Scan => {
                self.send_bg(BackgroundEvent::Scan);
            }

            Message::Shutdown => self.send_bg(BackgroundEvent::Shutdown),

            Message::AutomaticUpdates(bool) => {}

            Message::Background(UiEvent::Progress(event)) => match event {
                ProgressEvent::Fetching(progress, total) => {
                    self.upgrade.state = OptionState::ShowProgress;
                    self.upgrade_progress = self.calculate_fetching_progress(progress, total);
                }

                ProgressEvent::Recovery(progress, total) => {
                    self.recovery.state = OptionState::ShowProgress;
                    self.recovery_progress = progress / 1024;
                    self.recovery_total = total / 1024;
                    self.recovery.label =
                        fl!("recovery-progress", current = progress, total = total).into();
                    self.recovery.sublabel = None;
                }

                ProgressEvent::Updates(percent) => {
                    self.upgrade.state = OptionState::ShowProgress;
                    self.upgrade_progress = percent as u64 / 4 + 25;
                }
            },

            // Signals that a process in the background has begun.
            Message::Background(UiEvent::Initiated(event)) => match event {
                InitiatedEvent::Download(version) => {
                    self.upgrade.state = OptionState::ShowProgress;
                    self.upgrade_progress = 0;
                    self.upgrade.label =
                        fl!("download-os", os = "Pop!_OS", version = (&*version)).into();
                    self.next_release = version;
                }

                InitiatedEvent::Refresh => {
                    self.refresh.state = OptionState::Hide;
                }

                InitiatedEvent::Scanning => {
                    self.upgrade_progress = 0;
                    self.recovery_progress = 0;
                    self.scanning = true;
                    self.scanning_label = fl!("checking-for-updates").into();
                    self.upgrade.state = OptionState::HideControls;
                }

                InitiatedEvent::Recovery => {
                    self.recovery.label = fl!("recovery-downloading").into();
                    self.recovery.sublabel = None;
                    self.recovery_progress = 0;
                    self.recovery.state = OptionState::ShowProgress;
                }
            },

            // Events pertaining to the upgrade section
            Message::Background(UiEvent::Upgrade(event)) => match event {
                OsUpgradeEvent::Cancelled => {
                    // (state.callback_event.borrow())(Event::NotUpgrading);

                    self.upgrade_downloaded = false;
                    self.upgrade.label = self.upgrade_text.clone();
                    self.upgrade_progress = 0;
                    self.upgrade.state = OptionState::ShowButton;
                }

                OsUpgradeEvent::Dialog => {
                    // release_upgrade_dialog(state, widgets)
                }

                OsUpgradeEvent::Dismissed(dismissed) => {
                    // tracing::info!(
                    //     "{} release",
                    //     if dismissed {
                    //         "dismissed"
                    //     } else {
                    //         "un-dismissed"
                    //     }
                    // );
                    // if let Some(dismisser) = state.dismisser.as_mut() {
                    //     dismisser.set_dismissed(dismissed);
                    // }
                }

                OsUpgradeEvent::Event(UpgradeEvent::UpgradingPackages) => {
                    self.upgrade_progress = 25;
                }

                OsUpgradeEvent::Event(UpgradeEvent::UpdatingSourceLists) => {
                    self.upgrade_progress = 25;
                    self.fetching_release = true;
                }

                OsUpgradeEvent::Event(_) => (),

                OsUpgradeEvent::Notification => {
                    // (state.callback_ready.borrow())()
                }

                OsUpgradeEvent::Upgrade => {
                    // upgrade_clicked(state, widgets)
                }
            },

            Message::Background(UiEvent::Updating) => {
                self.scanning_label = fl!("daemon-updating").into();
                self.scanning = true;
            }

            Message::Background(UiEvent::Updated) => {
                self.scanning = false;
            }

            // Events pertaining to the recovery section
            Message::Background(UiEvent::Recovery(event)) => match event {
                OsRecoveryEvent::Event(event) => match event {
                    RecoveryEvent::Verifying => {
                        self.recovery.label = fl!("recovery-verify").into();
                        self.recovery.state = OptionState::HideControls;
                    }

                    RecoveryEvent::Syncing => {
                        self.recovery.label = fl!("recovery-sync").into();
                    }

                    _ => (),
                },

                OsRecoveryEvent::Failed => {
                    self.recovery.state = OptionState::ShowButton;
                }

                OsRecoveryEvent::Refresh => {
                    // if gtk::ResponseType::Accept == RefreshDialog::new().run() {
                    //     let _ = state.sender.send(BackgroundEvent::RefreshOS);
                    // } else {
                    //     widgets.recovery.options[REFRESH_OS].show_button();
                    // }
                }

                OsRecoveryEvent::Update => {
                    // recovery::clicked(state, widgets)
                }
            },

            Message::Background(UiEvent::Completed(event)) => match event {
                CompletedEvent::Download => {
                    self.upgrade_downloaded = true;

                    let description = fl!(
                        "notification-description",
                        os = "Pop!_OS",
                        version = (&*self.next_release)
                    );

                    // std::thread::spawn(move || {
                    //     notify::notify(
                    //         "distributor-logo",
                    //         &fl!("notification-title"),
                    //         &description,
                    //         || {
                    //             if let Some(sender) = sender.upgrade() {
                    //                 let _ =
                    //                     sender.send(UiEvent::Upgrade(OsUpgradeEvent::Notification));
                    //             }
                    //         },
                    //     );
                    // });

                    self.upgrade_text = fl!(
                        "download-os-complete",
                        os = "Pop!_OS",
                        version = (&*self.next_release)
                    )
                    .into();

                    self.upgrade.label = fl!("button-upgrade").into();
                    self.upgrade.state = OptionState::ShowButton;

                    // (state.callback_event.borrow())(Event::UpgradeReady);
                }

                CompletedEvent::Recovery(enable_refresh) => {
                    self.refresh.sensitive = enable_refresh;
                    self.upgrade.sensitive = true;

                    self.recovery.label = fl!("recovery-header").into();
                    self.recovery.sublabel = Some(fl!("most-current-recovery").into());
                    self.recovery.state = OptionState::HideControls;
                }

                CompletedEvent::Refresh => {
                    // reboot()
                }

                CompletedEvent::Scan(event) => {
                    self.scanning = false;
                    // widgets.stack.set_visible_child_name("updated");

                    match event {
                        ScanEvent::PermissionDenied => {}

                        ScanEvent::Found {
                            is_current,
                            is_lts,
                            refresh,
                            status_failed,
                            reboot_ready,
                            upgrading_recovery,
                            urgent,
                            mut current,
                            upgrade_text,
                            upgrade,
                        } => {
                            self.is_lts = is_lts;
                            self.reboot_ready = reboot_ready;
                            self.upgrade_text = upgrade_text;
                            self.release_info = upgrade;
                            self.upgrading_recovery = upgrading_recovery;
                            self.refresh_found = refresh;

                            if let Some(release) = current.take() {
                                self.recovery_urgent = urgent;
                                self.current_release = release;
                            }

                            if is_current {
                                self.upgrade.label = fl!("release-current").into();
                                self.upgrade.state = OptionState::Hide;
                            } else if status_failed {
                                self.upgrade.label = fl!("error-upgrade-status").into();
                                self.upgrade.state = OptionState::Hide;
                            } else {
                                if let Some(info) = self.release_info.as_ref() {
                                    self.upgrade_found = true;
                                    self.upgrading_from = info.current.clone();
                                }

                                // self.upgrade.sub_label = match EolDate::fetch() {
                                //     Ok(eol) => {
                                //         let (y, m, d) = eol.ymd;
                                //         match eol.status() {
                                //             EolStatus::Exceeded => Some(fl!(
                                //                 "eol-exceeded",
                                //                 current = fomat!((eol.version)),
                                //                 next = fomat!((eol.version.next_release()))
                                //             )),
                                //             EolStatus::Imminent => Some(fl!(
                                //                 "eol-imminent",
                                //                 current = fomat!((eol.version)),
                                //                 date = fomat!((Utc.ymd(y as i32, m, d).format("%B %-d, %Y")))
                                //             )),
                                //             EolStatus::Ok => None,
                                //         }
                                //     }
                                //     Err(why) => {
                                //         error!("{}: {}", fl!("eol-error"), why);
                                //         None
                                //     }
                                // };
                                self.upgrade.state = OptionState::ShowButton;
                            }

                            if refresh {
                                self.recovery.state = OptionState::ShowButton;

                                // refresh button send UiEvent::Recovery(OsRecoveryEvent::Refresh)

                                let allow_refresh = if self.recovery_urgent {
                                    self.recovery.label = fl!("recovery-update-found").into();
                                    self.recovery.sublabel = None;
                                    false
                                    // recovery button send UiEvent::Recovery(OsRecoveryEvent::Update)
                                } else if upgrading_recovery {
                                    self.upgrade.sensitive = false;
                                    self.recovery.sensitive = false;
                                    true
                                } else if status_failed {
                                    self.recovery.label = fl!("recovery-header").into();
                                    self.recovery.sublabel =
                                        Some(fl!("error-recovery-check").into());
                                    true
                                } else {
                                    self.recovery.state = OptionState::HideControls;
                                    self.recovery.label = fl!("recovery-header").into();
                                    self.recovery.sublabel =
                                        Some(fl!("most-current-recovery").into());
                                    true
                                };

                                self.refresh.sensitive = allow_refresh;
                            } else {
                                self.recovery.state = OptionState::Hide;
                            }
                        }
                    }
                }
            },

            Message::Background(UiEvent::StatusChanged(from, to, why)) => {
                tracing::warn!("status changed from {} to {}: {}", from, to, why);
                self.send_bg(BackgroundEvent::GetStatus(from));
            }

            Message::Background(UiEvent::Error(why)) => {
                if let Some(why) = Arc::into_inner(why) {
                    self.error(why);
                }
            }

            Message::Background(UiEvent::WaitingOnLock) => (),

            Message::Background(UiEvent::Shutdown) => {
                // return false
            }
        }

        cosmic::Task::none()
    }

    fn calculate_fetching_progress(&self, mut progress: u64, total: u64) -> u64 {
        progress = if self.fetching_release {
            progress / 2
        } else {
            progress / 4
        };

        progress = progress * 100 / total;

        if self.fetching_release {
            progress += 50;
        }

        progress
    }

    fn error(&mut self, why: UiError) {
        static GENERIC: LazyLock<String> = LazyLock::new(|| {
            fomat!(
                (fl!("error-header")) "\n\n"
                "* /etc/apt/sources.list\n"
                "* /etc/apt/sources.list.d/\n"
                "* /etc/fstab\n\n"
                (fl!("error-collect-logs")) "\n\n"
                (fl!("error-package-manager")) "\n\n"
                "sudo apt clean\n"
                "sudo apt update -m\n"
                "sudo dpkg --configure -a\n"
                "sudo apt install -f\n"
                "sudo apt dist-upgrade\n"
                "sudo apt autoremove --purge\n"
            )
        });

        let error_message = &mut format!("{}", why);
        why.iter_sources().for_each(|source| {
            error_message.push_str(": ");
            error_message.push_str(format!("{}", source).as_str());
        });

        self.error_message = Some(
            if let UiError::Recovery(ref why) = why {
                self.recovery.label = fl!("error-recovery-download").into();
                self.recovery.sublabel = Some(fl!("error-try-again").into());
                self.recovery.state = OptionState::HideControls;
                format!("{}:\n\n{:#?}", fl!("error-recovery-update"), why)
            } else {
                fomat!((&*GENERIC) "\n\n" (fl!("error-originating-cause")) "\n\n" (error_message))
            }
            .into(),
        );

        tracing::error!("{}", error_message);

        if let UiError::Dismiss(dismissed, _) = why {
            // if let Some(dismisser) = state.dismisser.as_mut() {
            //     dismisser.set_dismissed(!dismissed);
            // }
        } else {
            // (state.callback_event.borrow())(Event::NotUpgrading);
            self.fetching_release = false;

            if self.recovery_urgent {
                self.recovery.state = OptionState::ShowButton;
            }

            if self.refresh_found {
                self.recovery.state = OptionState::ShowButton;
            }

            if self.upgrade_found {
                self.upgrade.state = OptionState::ShowButton;
            }
        }
    }

    fn send_bg(&self, message: BackgroundEvent) {
        if let Some((sender, _)) = self.bg_context.as_ref() {
            let sender = sender.clone();
            tokio::task::spawn(async move {
                _ = sender.send(message).await;
            });
        }
    }
}

fn os_upgrade() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        os_upgrade_txt = fl!("os-upgrade");
    });

    Section::default()
        .descriptions(descriptions)
        .show_while::<Page>(|page| !matches!(page.upgrade.state, OptionState::Hide))
        .view::<Page>(move |_binder, page, section| {
            let txt = &section.descriptions;

            let control: Element<_> = match page.upgrade.state {
                OptionState::Hide => return widget::row().into(),

                OptionState::HideControls => widget::row().into(),

                OptionState::ShowButton => {
                    let message = page.upgrade.sensitive.then(|| Message::UpgradeDownload);

                    button::standard(fl!("button-download"))
                        .on_press_maybe(message)
                        .into()
                }

                OptionState::ShowProgress => {
                    widget::progress_bar(0.0..=100.0, page.upgrade_progress as f32).into()
                }
            };

            let mut os_upgrade_row = widget::settings::item::builder(&*page.upgrade.label);

            if let Some(description) = page.upgrade.sublabel.as_deref() {
                os_upgrade_row = os_upgrade_row.description(description);
            }

            widget::settings::section()
                .title(&txt[os_upgrade_txt])
                .add(os_upgrade_row.control(control))
                .apply(Element::from)
                .map(Into::into)
        })
}

fn os_recovery() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        os_upgrade_txt = fl!("os-recovery");
    });

    Section::default()
        .descriptions(descriptions)
        .show_while::<Page>(|page| !matches!(page.refresh.state, OptionState::Hide))
        .view::<Page>(move |_binder, page, section| {
            let txt = &section.descriptions;

            let recovery_control: Option<Element<_>> = match page.recovery.state {
                OptionState::Hide => None,

                OptionState::HideControls => Some(widget::row().into()),

                OptionState::ShowButton => {
                    let message = page.recovery.sensitive.then(|| Message::RecoveryUpdate);

                    let button = button::standard(fl!("button-update")).on_press_maybe(message);

                    Some(button.into())
                }

                OptionState::ShowProgress => {
                    let progress = widget::progress_bar(
                        0.0..=(page.recovery_total as f32),
                        page.recovery_progress as f32,
                    );
                    Some(progress.into())
                }
            };

            let refresh_control: Option<Element<_>> = match page.refresh.state {
                OptionState::Hide | OptionState::ShowProgress => None,

                OptionState::HideControls => Some(widget::row().into()),

                OptionState::ShowButton => {
                    let message = page.refresh.sensitive.then(|| Message::RefreshOS);

                    let button = button::standard(fl!("button-refresh")).on_press_maybe(message);

                    Some(button.into())
                }
            };

            let mut os_recovery_row = widget::settings::item::builder(&*page.recovery.label);

            if let Some(description) = page.recovery.sublabel.as_deref() {
                os_recovery_row = os_recovery_row.description(description);
            }

            let mut os_refresh_row = widget::settings::item::builder(&*page.refresh.label);

            if let Some(description) = page.refresh.sublabel.as_deref() {
                os_refresh_row = os_refresh_row.description(description);
            }

            widget::settings::section()
                .title(&txt[os_upgrade_txt])
                .add_maybe(recovery_control.map(|control| os_recovery_row.control(control)))
                .add_maybe(refresh_control.map(|control| os_refresh_row.control(control)))
                .apply(Element::from)
                .map(Into::into)
        })
}
