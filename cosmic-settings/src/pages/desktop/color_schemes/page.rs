use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic::iced::alignment::Vertical;
use cosmic::iced::{Alignment, Color, Length};
use cosmic::{widget, Apply, Command, Element};
use cosmic_settings_page::{AutoBind, Info, Page as SettingsPage, Section};

use crate::app;
use crate::pages::desktop::appearance::color_button;
use crate::pages::desktop::color_schemes::config::ColorSchemeVariant;
use crate::pages::desktop::color_schemes::{ColorScheme, COLOR_SCHEMES};
use cosmic_settings_page::section::Entity;

pub struct Page {
    current_theme: ColorSchemeVariant,
    theme_config: Option<Config>,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetTheme(bool, ColorSchemeVariant),
}

impl Default for Page {
    fn default() -> Self {
        let theme_config = ColorSchemeVariant::config().ok();
        let Some(config) = &theme_config else {
            return Self::default();
        };
        let Ok(current_theme) = ColorSchemeVariant::get_entry(config) else {
            return Self::default();
        };
        Self {
            current_theme,
            theme_config,
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::SetTheme(is_dark, color_scheme) => {
                self.current_theme = color_scheme.clone();
                if let Some(ref config) = self.theme_config {
                    let _ = self.current_theme.write_entry(config);
                }

                Command::batch(vec![
                    cosmic::command::message(app::Message::PageMessage(
                        crate::pages::Message::Appearance(
                            crate::pages::desktop::appearance::Message::DarkMode(is_dark),
                        ),
                    )),
                    cosmic::command::message(app::Message::PageMessage(
                        crate::pages::Message::Appearance(
                            crate::pages::desktop::appearance::Message::ImportSuccess(Box::new(
                                color_scheme.theme().unwrap_or_default(),
                            )),
                        ),
                    )),
                ])
            }
        }
    }

    pub fn theme_list(&self) -> Section<crate::pages::Message> {
        Section::default()
            .descriptions(vec![fl!("color-schemes-description").into()])
            .view::<Page>(move |_binder, page, _section| {
                let spacing = cosmic::theme::active().cosmic().spacing;

                let Ok(()) = ColorScheme::fetch_color_schemes() else {
                    return widget::row::with_children(vec![
                        widget::text(fl!("color-schemes-error")).into()
                    ])
                    .align_items(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into();
                };

                let themes: Vec<Element<crate::pages::Message>> = COLOR_SCHEMES
                    .get()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|color_scheme| {
                        widget::container(
                            widget::column::with_children(vec![
                                widget::text::heading(color_scheme.name.clone()).into(),
                                widget::flex_row(vec![
                                    Self::theme_button(
                                        color_scheme.light.clone(),
                                        page.current_theme.clone(),
                                        false,
                                    ),
                                    Self::theme_button(
                                        color_scheme.dark.clone(),
                                        page.current_theme.clone(),
                                        true,
                                    ),
                                ])
                                .row_spacing(spacing.space_xs)
                                .column_spacing(spacing.space_xs)
                                .into(),
                            ])
                            .align_items(Alignment::Center)
                            .width(Length::Fill)
                            .spacing(spacing.space_xs)
                            .padding(spacing.space_xs)
                            .apply(Element::from)
                            .map(crate::pages::Message::Themes),
                        )
                        .align_y(Vertical::Center)
                        .style(cosmic::theme::Container::Card)
                        .into()
                    })
                    .collect();

                widget::scrollable(widget::column::with_children(themes).spacing(spacing.space_xs))
                    .into()
            })
    }

    pub fn theme_button<'a>(
        selected_color_scheme: ColorSchemeVariant,
        current_theme: ColorSchemeVariant,
        is_dark: bool,
    ) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;
        let icon_size = 40;
        let selected = selected_color_scheme == current_theme;
        let properties = selected_color_scheme.theme().unwrap_or_default();
        widget::button(
            widget::column::with_children(vec![
                widget::text(selected_color_scheme.name.clone()).into(),
                widget::row::with_children(vec![
                    color_button(
                        None,
                        Color::from(properties.bg_color.unwrap_or_default()),
                        false,
                        icon_size,
                        icon_size,
                    )
                    .into(),
                    color_button(
                        None,
                        Color::from(properties.primary_container_bg.unwrap_or_default()),
                        false,
                        icon_size,
                        icon_size,
                    )
                    .into(),
                    color_button(
                        None,
                        Color::from(properties.accent.unwrap_or_default()),
                        false,
                        icon_size,
                        icon_size,
                    )
                    .into(),
                    color_button(
                        None,
                        Color::from(properties.text_tint.unwrap_or_default()),
                        false,
                        icon_size,
                        icon_size,
                    )
                    .into(),
                ])
                .spacing(spacing.space_xxxs)
                .into(),
            ])
            .spacing(spacing.space_xxs)
            .padding(spacing.space_xs)
            .align_items(Alignment::Center),
        )
        .selected(selected)
        .style(widget::button::Style::Image)
        .on_press(Message::SetTheme(is_dark, selected_color_scheme.clone()))
        .into()
    }
}

impl AutoBind<crate::pages::Message> for Page {}

impl SettingsPage<crate::pages::Message> for Page {
    fn info(&self) -> Info {
        Info::new("color_schemes", "utilities-tweak-tool-symbolic")
            .title(fl!("color-schemes"))
            .description(fl!("color-schemes-description"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<cosmic_settings_page::Content> {
        Some(vec![sections.insert(self.theme_list())])
    }
}
