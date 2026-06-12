// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{Message, Page};
use cosmic::{Apply, Element, widget};
use cosmic_settings_page::{self as page, Section};
use pop_system_updater::{Frequency, client};

#[derive(Default)]
pub(super) struct Model {
    pub(super) clients: Option<(
        client::ClientProxy<'static>,
        client::notification::ClientProxy<'static>,
    )>,
    pub(super) notification_config: pop_system_updater::NotificationConfig,
    pub(super) config: pop_system_updater::Config,
}

pub fn section() -> page::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        header_text = fl!("system-and-app-updates");
        automatic_updates_text = fl!("system-and-app-updates", "automatic");
        notifications_text = fl!("system-and-app-updates", "notifications");
        frequency_daily = fl!("frequency-daily");
        frequency_weekly = fl!("frequency-weekly");
        frequency_monthly = fl!("frequency-monthly");
        disabled = fl!("disabled");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;

            let notification_dropdown = widget::dropdown::popup_dropdown(
                vec![
                    desc[disabled].clone(),
                    desc[frequency_daily].clone(),
                    desc[frequency_weekly].clone(),
                    desc[frequency_monthly].clone(),
                ],
                Some(if !page.system_updates.notification_config.enabled {
                    0
                } else {
                    match page
                        .system_updates
                        .notification_config
                        .notification_frequency
                    {
                        Frequency::Daily => 1,
                        Frequency::Weekly => 2,
                        Frequency::Monthly => 3,
                    }
                }),
                |selected| {
                    Message::SetNotificationFrequency(match selected {
                        1 => Some(Frequency::Daily),
                        2 => Some(Frequency::Weekly),
                        3 => Some(Frequency::Monthly),
                        _ => None,
                    })
                },
                cosmic::iced::window::Id::RESERVED,
                Message::Surface,
                crate::app::Message::from,
            );

            let automatic_updates_toggle =
                widget::settings::item::builder(&desc[automatic_updates_text])
                    .toggler(page.system_updates.config.auto_update, |enable| {
                        Message::SetAutomaticUpdates(enable)
                    });

            let update_notification_frequency =
                widget::settings::item::builder(&desc[notifications_text])
                    .control(notification_dropdown);

            widget::settings::section()
                .title(&desc[header_text])
                .add(automatic_updates_toggle)
                .add(update_notification_frequency)
                .apply(Element::from)
                .map(crate::pages::Message::SystemUpdater)
        })
}

pub async fn load_clients() -> crate::pages::Message {
    let system = zbus::Connection::system();
    let session = zbus::Connection::session();

    let (system, session) = match futures::try_join!(system, session) {
        Ok(v) => v,
        Err(why) => return Message::Error(format!("DBus connection failed: {why}")).into(),
    };

    let system_client = client::ClientProxy::new(&system);
    let session_client = client::notification::ClientProxy::new(&session);

    match futures::try_join!(system_client, session_client) {
        Ok((system_client, session_client)) => {
            Message::SystemUpdateClient(system_client, session_client).into()
        }
        Err(why) => Message::Error(format!("DBus proxy connection failed: {why}")).into(),
    }
}

pub async fn load_configs() -> crate::pages::Message {
    let (config, notification_config) = futures::join!(
        pop_system_updater::Config::load(),
        pop_system_updater::NotificationConfig::load(),
    );
    Message::SystemUpdateConfigs(config, notification_config).into()
}
