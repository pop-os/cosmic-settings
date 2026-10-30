use cosmic::widget::{list, segmented_button, settings};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;
use std::collections::HashMap;

use super::{Message, Page};

pub fn section() -> Section<crate::pages::Message> {
    #[inline]
    fn i18n() -> (slab::Slab<String>, HashMap<String, usize>) {
        let mut descriptions = slab::Slab::new();
        let keys: HashMap<String, usize> = HashMap::from([
            (
                "auto_switch".into(),
                descriptions.insert(fl!("auto-switch")),
            ),
            (
                "auto_switch_desc/sunrise".into(),
                descriptions.insert(fl!("auto-switch", "sunrise")),
            ),
            (
                "auto_switch_desc/sunset".into(),
                descriptions.insert(fl!("auto-switch", "sunset")),
            ),
            (
                "auto_switch_desc/next-sunrise".into(),
                descriptions.insert(fl!("auto-switch", "next-sunrise")),
            ),
            (
                "auto_switch_desc/next-sunset".into(),
                descriptions.insert(fl!("auto-switch", "next-sunset")),
            ),
            ("mode".into(), descriptions.insert(fl!("mode"))),
        ]);

        (descriptions, keys)
    }

    fn auto_switch<'a>(
        page: &Page,
        section: &'a Section<crate::pages::Message>,
        labels: &HashMap<String, usize>,
    ) -> list::ListButton<'a, Message> {
        let descriptions = &section.descriptions;

        settings::item::builder(&descriptions[labels["auto_switch"]])
            .description(
                if !page.day_time && page.theme_manager.mode().is_dark {
                    &descriptions[labels["auto_switch_desc/sunrise"]]
                } else if page.day_time && !page.theme_manager.mode().is_dark {
                    &descriptions[labels["auto_switch_desc/sunset"]]
                } else if page.day_time && page.theme_manager.mode().is_dark {
                    &descriptions[labels["auto_switch_desc/next-sunrise"]]
                } else {
                    &descriptions[labels["auto_switch_desc/next-sunset"]]
                }
                .clone(),
            )
            .toggler(page.theme_manager.mode().auto_switch, Message::Autoswitch)
    }

    fn theme_mode<'a, 'b>(
        page: &'a Page,
        section: &'b Section<crate::pages::Message>,
        labels: &HashMap<String, usize>,
    ) -> impl Into<Element<'b, Message>>
    where
        'a: 'b,
    {
        let descriptions = &section.descriptions;

        settings::item::builder(&descriptions[labels["mode"]]).control(
            segmented_button::horizontal(&page.mode_model)
                .button_alignment(cosmic::iced::Alignment::Center)
                .style(cosmic::theme::style::SegmentedButton::Control)
                .on_activate(Message::DarkMode),
        )
    }

    let (descriptions, label_keys) = i18n();

    Section::default()
        .title(fl!("mode"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let label_keys = label_keys.clone();

            settings::section()
                .add(auto_switch(page, section, &label_keys))
                .add(theme_mode(page, section, &label_keys))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}
