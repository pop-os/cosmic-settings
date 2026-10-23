use std::path::PathBuf;

use cosmic::iced::core::{Alignment, Length};
use cosmic::widget::{self, button, container, icon, row, settings, space, text};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;
use cosmic_settings_wallpaper as wallpaper;
use cosmic_theme_bundle_config::{Mode, Part, Store};

use super::Message;
use super::custom::Page;
use crate::pages::desktop::wallpaper::widgets::color_image;

#[derive(Debug, PartialEq)]
enum Wallpaper {
    Image(PathBuf),
    Color(wallpaper::Color),
}

fn active_wallpapers(mode: Mode) -> Vec<Wallpaper> {
    let Ok(store) = Store::new() else {
        return Vec::new();
    };

    let Ok(config) = store.live_config(Part::Background, mode) else {
        return Vec::new();
    };

    let Ok(backgrounds) = cosmic_bg_config::Config::load(&cosmic_bg_config::Context(config)) else {
        return Vec::new();
    };

    let mut wallpapers: Vec<Wallpaper> = Vec::new();

    let entries: Vec<&cosmic_bg_config::Entry> =
        if backgrounds.same_on_all || backgrounds.backgrounds.is_empty() {
            vec![&backgrounds.default_background]
        } else {
            backgrounds.backgrounds.iter().collect()
        };

    for entry in entries {
        let wallpaper = match &entry.source {
            cosmic_bg_config::Source::Path(path) if path.is_file() => {
                Wallpaper::Image(path.clone())
            }
            cosmic_bg_config::Source::Path(_) => continue,
            cosmic_bg_config::Source::Color(color) => Wallpaper::Color(color.clone()),
        };

        if !wallpapers.contains(&wallpaper) {
            wallpapers.push(wallpaper);
        }
    }

    wallpapers
}

fn wallpaper_card(wallpaper: &Wallpaper) -> Element<'static, Message> {
    match wallpaper {
        Wallpaper::Image(path) => container(
            widget::image(path.clone())
                .width(Length::Fixed(240.))
                .height(Length::Fixed(160.))
                .content_fit(cosmic::iced::ContentFit::Cover)
                .border_radius(cosmic::theme::active().cosmic().corner_radii.radius_s),
        )
        .width(Length::Fill)
        .class(cosmic::theme::Container::custom(|_| container::Style {
            snap: true,
            ..Default::default()
        }))
        .into(),
        Wallpaper::Color(color) => color_image(color.clone(), 240, 160, None),
    }
}

#[allow(clippy::too_many_lines)]
pub fn section() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("wallpaper"))
        .view::<Page>(move |_binder, page, section| {
            let spacing = cosmic::theme::spacing();
            let wallpapers = active_wallpapers(page.mode);

            let change = button::custom(
                row::with_capacity(2)
                    .push(text::body(fl!("change-wallpaper")))
                    .push(icon::from_name("go-next-symbolic").size(16).icon())
                    .align_y(Alignment::Center)
                    .spacing(spacing.space_xxxs),
            )
            .class(button::ButtonClass::Link)
            .padding([0, spacing.space_s])
            .on_press(Message::WallpaperPage);

            let title = row::with_capacity(3)
                .push(text::heading(section.title.as_str()))
                .push(space::horizontal().width(spacing.space_xxs))
                .push(change)
                .align_y(Alignment::Center);

            settings::section()
                .header(title)
                .add(
                    container(cosmic::widget::flex_row(
                        wallpapers
                            .iter()
                            .map(|w_path| {
                                button::custom_image_button(wallpaper_card(w_path), None)
                                    .padding(0)
                                    .class(button::ButtonClass::Image)
                                    .on_press(Message::WallpaperPage)
                                    .width(Length::Fixed(240.0))
                                    .height(Length::Fixed(160.0))
                                    .into()
                            })
                            .collect(),
                    ))
                    .center_x(Length::Fill),
                )
                .apply(Element::from)
                .map(crate::pages::Message::AppearanceTheme)
        })
}
