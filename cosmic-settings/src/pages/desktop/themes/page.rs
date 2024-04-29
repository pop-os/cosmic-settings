use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic::iced::alignment::Vertical;
use cosmic::iced::{Alignment, Color, Length};
use cosmic::widget::horizontal_space;
use cosmic::{widget, Apply, Command, Element};
use cosmic_settings_page::{AutoBind, Info, Page as SettingsPage, Section};

use crate::app;
use crate::pages::desktop::appearance::color_button;
use crate::pages::desktop::themes::config::Theme;
use crate::pages::desktop::themes::{GlobalTheme, THEMES};
use cosmic_settings_page::section::Entity;

pub struct Page {
    current_theme: Theme,
    theme_config: Option<Config>,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetTheme(bool, GlobalTheme),
}

impl Default for Page {
    fn default() -> Self {
        let theme_config = Theme::config().ok();
        let Some(config) = &theme_config else {
            return Self::default();
        };
        let Ok(current_theme) = Theme::get_entry(config) else {
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
            Message::SetTheme(is_dark, theme) => {
                let theme = if is_dark { theme.dark } else { theme.light };
                self.current_theme = theme.clone();
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
                                theme.properties().unwrap_or_default(),
                            )),
                        ),
                    )),
                ])
            }
        }
    }

    pub fn theme_list(&self) -> Section<crate::pages::Message> {
        Section::default()
            .descriptions(vec![fl!("reset-to-default").into()])
            .view::<Page>(move |_binder, page, _section| {
                let Ok(()) = GlobalTheme::init_themes() else {
                    return widget::row::with_children(vec![
                        widget::text("Error fetching themes").into()
                    ])
                    .align_items(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into();
                };

                let themes: Vec<Element<crate::pages::Message>> = THEMES
                    .get()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|theme| {
                        widget::container(
                            widget::column::with_children(vec![
                                widget::text::heading(theme.name.clone()).into(),
                                widget::flex_row(vec![
                                    Self::theme_button(
                                        theme.clone(),
                                        page.current_theme.clone(),
                                        false,
                                    ),
                                    Self::theme_button(
                                        theme.clone(),
                                        page.current_theme.clone(),
                                        true,
                                    ),
                                ])
                                .row_spacing(cosmic::theme::active().cosmic().space_xs())
                                .column_spacing(cosmic::theme::active().cosmic().space_xs())
                                .into(),
                            ])
                            .align_items(Alignment::Center)
                            .width(Length::Fill)
                            .spacing(cosmic::theme::active().cosmic().space_xs())
                            .padding(cosmic::theme::active().cosmic().space_xs())
                            .apply(Element::from)
                            .map(crate::pages::Message::Themes),
                        )
                        .align_y(Vertical::Center)
                        .style(cosmic::theme::Container::Card)
                        .into()
                    })
                    .collect();

                widget::scrollable(
                    widget::column::with_children(themes)
                        .spacing(cosmic::theme::active().cosmic().space_xs()),
                )
                .into()
            })
    }

    pub fn theme_button<'a>(
        selected_theme: GlobalTheme,
        current_theme: Theme,
        is_dark: bool,
    ) -> Element<'a, Message> {
        let icon_size = 40;
        let name = if is_dark {
            selected_theme.dark.name.clone()
        } else {
            selected_theme.light.name.clone()
        };
        let variant = if is_dark {
            selected_theme.dark.clone()
        } else {
            selected_theme.light.clone()
        };
        let selected = variant == current_theme;
        let properties = variant.properties().unwrap_or_default();
        widget::button(
            widget::column::with_children(vec![
                widget::text(name).into(),
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
                .spacing(cosmic::theme::active().cosmic().space_xxxs())
                .into(),
            ])
            .spacing(cosmic::theme::active().cosmic().space_xxs())
            .padding(cosmic::theme::active().cosmic().space_xs())
            .align_items(Alignment::Center),
        )
        .selected(selected)
        .style(widget::button::Style::Image)
        .on_press(Message::SetTheme(is_dark, selected_theme.clone()))
        .into()
    }
}

impl AutoBind<crate::pages::Message> for Page {}

impl SettingsPage<crate::pages::Message> for Page {
    fn info(&self) -> Info {
        Info::new("themes", "utilities-tweak-tool-symbolic")
            .title("Themes")
            .description("Change the system theme.")
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<cosmic_settings_page::Content> {
        Some(vec![sections.insert(self.theme_list())])
    }
}
