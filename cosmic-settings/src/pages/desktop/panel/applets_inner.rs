use cosmic::app::ContextDrawer;
use cosmic::cosmic_theme::palette::WithAlpha;
use cosmic::iced::clipboard::dnd::{
    DndAction, DndDestinationRectangle, DndEvent, OfferEvent, SourceEvent,
};
use cosmic::iced::clipboard::mime::AsMimeTypes;
use cosmic::iced::id::Internal;

use cosmic::iced_core;
use cosmic::iced_core::clipboard::IconSurface;
use cosmic::widget::{Column, button, column, container, icon, list_column, row, text, text_input};

use cosmic::{
    Apply, Element,
    cosmic_config::{Config, CosmicConfigEntry},
    iced::{
        Alignment, Border, Color, Length, Point, Rectangle, Size, Vector, core::window, event,
        mouse, overlay, touch,
    },
    iced_runtime::{Task, core::id::Id},
    iced_widget::core::{
        Clipboard, Shell, Widget, layout, renderer,
        widget::{Operation, Tree, tree},
    },
    theme,
};

use std::path::{Path, PathBuf};
use std::{borrow::Cow, fmt::Debug, mem, sync::LazyLock};

use crate::{app, pages};
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, Section, section};
use freedesktop_desktop_entry::DesktopEntry;
use slotmap::{Key, SlotMap};
use tracing::error;

const MIME_TYPE: &str = "text/uri-list";

// pub type OnDndTask<'a, Message> = Box<
//     dyn Fn(
//             Box<dyn Send + Sync + Fn() -> platform_specific::wayland::data_device::ActionInner>,
//         ) -> Message
//         + 'a,
// >;

// radius is 8.0
const DRAG_START_DISTANCE_SQUARED: f32 = 64.0;

pub static APPLET_DND_ICON_ID: LazyLock<window::Id> = LazyLock::new(window::Id::unique);

pub struct Page {
    pub(crate) entity: page::Entity,
    pub(crate) available_entries: Vec<Applet<'static>>,
    pub(crate) config_helper: Option<Config>,
    pub(crate) current_config: Option<CosmicPanelConfig>,
    pub(crate) reorder_widget_state: Option<(Applet<'static>, CosmicPanelConfig)>,
    pub(crate) search: String,
    pub(crate) context: Option<ContextDrawerVariant>,
}

impl Default for Page {
    fn default() -> Self {
        let config_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let current_config = config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            // If the config is not present, it will be created with the default values and the name will not match
            (panel_config.name == "Panel").then_some(panel_config)
        });
        Self {
            entity: page::Entity::null(),
            available_entries: freedesktop_desktop_entry::Iter::new(
                freedesktop_desktop_entry::default_paths(),
            )
            .filter_map(|p| Applet::try_from(Cow::from(p)).ok())
            .collect(),
            config_helper,
            current_config,
            reorder_widget_state: None,
            search: String::new(),
            context: None,
        }
    }
}

pub trait AppletsPage {
    fn inner(&self) -> &Page;

    fn inner_mut(&mut self) -> &mut Page;
}

impl AppletsPage for Page {
    fn inner(&self) -> &Page {
        self
    }

    fn inner_mut(&mut self) -> &mut Page {
        self
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(lists::<Page, _>(pages::Message::PanelApplet)),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("panel_applets", "preferences-dock-symbolic").title(fl!("applets"))
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let content = row::with_capacity(1)
            .push(button::standard(fl!("add-applet")).on_press(Message::AddAppletDrawer))
            .apply(container)
            .width(Length::Fill)
            .align_x(Alignment::End)
            .apply(Element::from)
            .map(crate::pages::Message::PanelApplet);

        Some(content)
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, pages::Message>> {
        Some(match self.context {
            Some(ContextDrawerVariant::AddApplet) => {
                let search_input = text_input::search_input(fl!("search-applets"), &self.search)
                    .on_input(Message::Search)
                    .on_paste(Message::Search)
                    .width(Length::Fixed(312.0))
                    .apply(Element::from)
                    .map(crate::pages::Message::PanelApplet);

                cosmic::app::context_drawer(
                    self.add_applet_view(crate::pages::Message::PanelApplet),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("add-applet"))
                .header(search_input)
            }
            None => return None,
        })
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

#[derive(Clone)]
pub enum Message {
    RemoveStart(String),
    RemoveCenter(String),
    RemoveEnd(String),
    DetailStart(String),
    DetailCenter(String),
    DetailEnd(String),
    ReorderStart(Vec<Applet<'static>>),
    ReorderCenter(Vec<Applet<'static>>),
    ReorderEnd(Vec<Applet<'static>>),
    Applets(Vec<Applet<'static>>),
    PanelConfig(Box<CosmicPanelConfig>),
    StartDnd(Applet<'static>),
    // DnDTask(Arc<Box<dyn Send + Sync + Fn() -> ActionInner>>),
    Search(String),
    AddApplet(Applet<'static>),
    AddAppletDrawer,
    Save,
    Cancel,
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::ReorderStart(_) => write!(f, "ReorderStart"),
            Message::ReorderCenter(_) => write!(f, "ReorderCenter"),
            Message::ReorderEnd(_) => write!(f, "ReorderEnd"),
            Message::Applets(_) => write!(f, "Applets"),
            Message::PanelConfig(_) => write!(f, "PanelConfig"),
            Message::StartDnd(_) => write!(f, "StartDnd"),
            // Message::DnDTask(_) => write!(f, "DnDTask"),
            Message::Save => write!(f, "ApplyReorder"),
            Message::RemoveStart(_) => write!(f, "RemoveStart"),
            Message::RemoveCenter(_) => write!(f, "RemoveCenter"),
            Message::RemoveEnd(_) => write!(f, "RemoveEnd"),
            Message::DetailStart(_) => write!(f, "DetailStart"),
            Message::DetailCenter(_) => write!(f, "DetailCenter"),
            Message::DetailEnd(_) => write!(f, "DetailEnd"),
            Message::Cancel => write!(f, "Cancel"),
            Message::Search(_) => write!(f, "Search"),
            Message::AddApplet(_) => write!(f, "AddApplet"),
            Message::AddAppletDrawer => write!(f, "AddAppletDialogue"),
        }
    }
}

pub enum ContextDrawerVariant {
    AddApplet,
}

impl Page {
    pub fn save(&self) {
        let Some(config) = self.current_config.as_ref() else {
            error!("No panel config. Failed to save applets.");
            return;
        };
        let Some(helper) = self.config_helper.as_ref() else {
            error!("No panel config helper. Failed to save applets.");
            return;
        };
        if let Err(e) = config.write_entry(helper) {
            error!("Failed to save applets: {:?}", e);
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn add_applet_view<T: Fn(Message) -> crate::pages::Message + Copy + 'static>(
        &self,
        msg_map: T,
    ) -> Element<'_, crate::pages::Message> {
        let cosmic::cosmic_theme::Spacing {
            space_xxxs,
            space_xs,
            ..
        } = theme::spacing();
        let mut list_column = list_column();
        let mut has_some = false;
        for info in self
            .available_entries
            .iter()
            .filter(|a| a.matches(&self.search))
        {
            if let Some(config) = self.current_config.as_ref() {
                if let Some(center) = config.plugins_center.as_ref()
                    && center.iter().any(|a| a.as_str() == info.id.as_ref())
                {
                    continue;
                }

                if let Some(wings) = config.plugins_wings.as_ref()
                    && wings
                        .0
                        .iter()
                        .chain(wings.1.iter())
                        .any(|a| a.as_str() == info.id.as_ref())
                {
                    continue;
                }
            }
            has_some = true;
            list_column = list_column.add(
                row::with_children(vec![
                    icon::from_name(&*info.icon).size(32).icon().into(),
                    column::with_capacity(2)
                        .push(text::body(info.name.clone()))
                        .push_maybe(if info.description.is_empty() {
                            None
                        } else {
                            Some(text::caption(info.description.clone()))
                        })
                        .spacing(space_xxxs)
                        .width(Length::Fill)
                        .into(),
                    button::text(fl!("add"))
                        .on_press(msg_map(Message::AddApplet(info.clone())))
                        .into(),
                ])
                .padding([space_xxxs, 0])
                .spacing(space_xs)
                .align_y(Alignment::Center),
            );
        }
        if !has_some {
            list_column = list_column.add(
                text::body(fl!("no-applets-found"))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            );
        }

        list_column.into()
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        match message {
            Message::PanelConfig(c) => {
                self.current_config = Some(*c);
            }

            Message::ReorderStart(start_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let Some((list, _)) = config.plugins_wings.as_mut() else {
                    config.plugins_wings = Some((
                        start_list
                            .into_iter()
                            .map(|a: Applet| a.id.into())
                            .collect(),
                        Vec::new(),
                    ));
                    return Task::none();
                };
                *list = start_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::ReorderCenter(center_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let Some(list) = config.plugins_center.as_mut() else {
                    config.plugins_center = Some(
                        center_list
                            .into_iter()
                            .map(|a: Applet| a.id.into())
                            .collect(),
                    );
                    return Task::none();
                };
                *list = center_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::ReorderEnd(end_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let Some((_, list)) = config.plugins_wings.as_mut() else {
                    config.plugins_wings = Some((
                        Vec::new(),
                        end_list.into_iter().map(|a: Applet| a.id.into()).collect(),
                    ));
                    return Task::none();
                };
                *list = end_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::Applets(applets) => {
                self.available_entries = applets;
            }
            Message::StartDnd(applet) => {
                self.reorder_widget_state =
                    Some((applet, self.current_config.clone().unwrap_or_default()));
                return Task::none();
            }
            Message::Save => {
                self.reorder_widget_state = None;
                self.save();
            }
            Message::RemoveStart(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let Some((list, _)) = config.plugins_wings.as_mut() else {
                    return Task::none();
                };
                list.retain(|id| id != &to_remove);
                self.save();
            }
            Message::RemoveCenter(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let Some(list) = config.plugins_center.as_mut() else {
                    return Task::none();
                };
                list.retain(|id| id != &to_remove);
                self.save();
            }
            Message::RemoveEnd(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let Some((_, list)) = config.plugins_wings.as_mut() else {
                    return Task::none();
                };
                list.retain(|id| id != &to_remove);
                self.save();
            }
            Message::DetailStart(_) | Message::DetailCenter(_) | Message::DetailEnd(_) => {
                // TODO ask design team
            }
            Message::Cancel => {
                if let Some((_, config)) = self.reorder_widget_state.take() {
                    self.current_config = Some(config);
                }
            }
            Message::Search(text) => {
                self.search = text;
            }
            Message::AddApplet(applet) => {
                // TODO ask design team
                let Some(config) = self.current_config.as_mut() else {
                    return Task::none();
                };
                let list = if let Some((list, _)) = config.plugins_wings.as_mut() {
                    list
                } else {
                    config.plugins_wings = Some((Vec::new(), Vec::new()));
                    &mut config.plugins_wings.as_mut().unwrap().0
                };

                list.push(applet.id.to_string());
                self.save();
            }
            Message::AddAppletDrawer => {
                self.context = Some(ContextDrawerVariant::AddApplet);
                return cosmic::task::message(app::Message::OpenContextDrawer(self.entity));
            }
        };
        Task::none()
    }
}

#[allow(clippy::too_many_lines)]
pub fn lists<
    P: page::Page<crate::pages::Message> + AppletsPage,
    T: Fn(Message) -> crate::pages::Message + Copy + 'static,
>(
    msg_map: T,
) -> Section<crate::pages::Message> {
    Section::default().view::<P>(move |_binder, page, _section| {
        let cosmic::cosmic_theme::Spacing {
            space_xxs,
            space_xs,
            ..
        } = theme::spacing();
        let page = page.inner();
        let Some(config) = page.current_config.as_ref() else {
            return Element::from(text::body(fl!("unknown")));
        };

        column::with_children([
            column::with_children([
                text::body(fl!("start-segment")).into(),
                AppletReorderList::new(
                    config
                        .plugins_wings
                        .as_ref()
                        .map(|list| {
                            list.0
                                .iter()
                                .filter_map(|id| {
                                    page.available_entries
                                        .iter()
                                        .find(|e| e.id.as_ref() == id.as_str())
                                        .map(Applet::borrowed)
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    Message::StartDnd,
                    // |a| Message::DnDTask(Arc::new(a)),
                    Message::RemoveStart,
                    Message::DetailStart,
                    Message::ReorderStart,
                    Message::Save,
                    Message::Cancel,
                    page.reorder_widget_state.as_ref().map(|(a, _)| a.clone()),
                )
                .into(),
            ])
            .spacing(space_xxs)
            .into(),
            column::with_children([
                text::body(fl!("center-segment")).into(),
                AppletReorderList::new(
                    config
                        .plugins_center
                        .as_ref()
                        .map(|list| {
                            list.iter()
                                .filter_map(|id| {
                                    page.available_entries
                                        .iter()
                                        .find(|e| e.id.as_ref() == id.as_str())
                                        .map(Applet::borrowed)
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    Message::StartDnd,
                    // |a| Message::DnDTask(Arc::new(a)),
                    Message::RemoveCenter,
                    Message::DetailCenter,
                    Message::ReorderCenter,
                    Message::Save,
                    Message::Cancel,
                    page.reorder_widget_state.as_ref().map(|(a, _)| a.clone()),
                )
                .into(),
            ])
            .spacing(space_xxs)
            .into(),
            column::with_children([
                text::body(fl!("end-segment")).into(),
                AppletReorderList::new(
                    config
                        .plugins_wings
                        .as_ref()
                        .map(|list| {
                            list.1
                                .iter()
                                .filter_map(|id| {
                                    page.available_entries
                                        .iter()
                                        .find(|e| e.id.as_ref() == id.as_str())
                                        .map(Applet::borrowed)
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    Message::StartDnd,
                    // |a| Message::DnDTask(Arc::new(a)),
                    Message::RemoveEnd,
                    Message::DetailEnd,
                    Message::ReorderEnd,
                    Message::Save,
                    Message::Cancel,
                    page.reorder_widget_state.as_ref().map(|(a, _)| a.clone()),
                )
                .into(),
            ])
            .spacing(space_xxs)
            .into(),
        ])
        .spacing(space_xs)
        .apply(Element::from)
        .map(msg_map)
    })
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Applet<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub icon: Cow<'a, str>,
    pub path: Cow<'a, Path>,
}

impl Applet<'_> {
    #[must_use]
    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_ascii_lowercase();

        [
            self.name.as_ref(),
            self.description.as_ref(),
            self.id.as_ref(),
        ]
        .iter()
        .any(|field| field.to_ascii_lowercase().contains(&query))
    }
}

impl<'a> TryFrom<Cow<'a, Path>> for Applet<'static> {
    type Error = anyhow::Error;

    fn try_from(path: Cow<'a, Path>) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let languages = freedesktop_desktop_entry::get_languages_from_env();
        let entry = DesktopEntry::from_str(path.as_ref(), &content, Some(&languages))?;
        if entry.desktop_entry("X-CosmicApplet").is_none() {
            anyhow::bail!("Not an applet");
        }

        Ok(Self {
            id: Cow::from(entry.id().to_string()),
            name: Cow::from(entry.name(&languages).unwrap_or_default().to_string()),
            description: Cow::from(entry.comment(&languages).unwrap_or_default().to_string()),
            icon: Cow::from(entry.icon().unwrap_or_default().to_string()),
            path: Cow::from(path.into_owned()),
        })
    }
}

impl Applet<'static> {
    fn borrowed(&self) -> Applet<'_> {
        Applet {
            id: Cow::from(self.id.as_ref()),
            name: Cow::from(self.name.as_ref()),
            description: Cow::from(self.description.as_ref()),
            icon: Cow::from(self.icon.as_ref()),
            path: Cow::from(self.path.as_ref()),
        }
    }
}

impl Applet<'_> {
    fn into_owned(self) -> Applet<'static> {
        Applet {
            id: Cow::from(self.id.into_owned()),
            name: Cow::from(self.name.into_owned()),
            description: Cow::from(self.description.into_owned()),
            icon: Cow::from(self.icon.into_owned()),
            path: Cow::from(self.path.into_owned()),
        }
    }
}

// TODO A11y / keyboard support
#[allow(dead_code)]
pub struct AppletReorderList<'a, Message> {
    id: Id,
    info: Vec<Applet<'a>>,
    on_create_dnd_source: Box<dyn Fn(Applet<'static>) -> Message + 'a>,
    // on_dnd_task_produced: OnDndTask<'a, Message>,
    on_reorder: Box<dyn Fn(Vec<Applet<'static>>) -> Message + 'a>,
    on_finish: Option<Message>,
    on_cancel: Option<Message>,
    inner: Element<'a, Message>,
    active_applet_offer: Option<Applet<'a>>,
}

impl<'a, Message: 'static + Clone> AppletReorderList<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    /// new applet list which can be reordered and dragged
    pub fn new(
        info: Vec<Applet<'a>>,
        on_create_dnd_source: impl Fn(Applet<'static>) -> Message + 'a,
        on_remove: impl Fn(String) -> Message + 'a,
        _on_details: impl Fn(String) -> Message + 'a,
        on_reorder: impl Fn(Vec<Applet<'static>>) -> Message + 'a,
        on_apply_reorder: Message,
        on_cancel: Message,
        active_dnd: Option<Applet<'a>>,
    ) -> Self {
        let cosmic::cosmic_theme::Spacing {
            space_xxxs,
            space_xxs,
            space_xs,
            ..
        } = theme::spacing();
        let applet_buttons = info
            .clone()
            .into_iter()
            .map(|info| {
                let id_clone = info.id.to_string();
                let is_dragged = active_dnd.as_ref().is_some_and(|dnd| dnd.id == info.id);

                let content = if is_dragged {
                    row().height(Length::Fixed(32.0))
                } else {
                    row::with_children(vec![
                        icon::from_name("grip-lines-symbolic")
                            .symbolic(true)
                            .size(16)
                            .into(),
                        icon::from_name(info.icon).size(32).into(),
                        column::with_capacity(2)
                            .spacing(space_xxxs)
                            .width(Length::Fill)
                            .push(text::body(info.name))
                            .push_maybe(if info.description.is_empty() {
                                None
                            } else {
                                Some(text::caption(info.description))
                            })
                            .into(),
                        button::icon(icon::from_name("edit-delete-symbolic"))
                            .extra_small()
                            .on_press(on_remove(id_clone.clone()))
                            .into(),
                    ])
                    .spacing(space_xs)
                    .align_y(Alignment::Center)
                };

                container(content)
                    .padding(8)
                    .width(Length::Fill)
                    .class(theme::Container::Custom(Box::new(move |theme| {
                        let mut style =
                            container::Catalog::style(theme, &theme::Container::Primary);
                        style.border.radius = theme.cosmic().radius_s().into();
                        if is_dragged {
                            style.border.color = theme.cosmic().accent_color().into();
                            style.border.width = 2.0;
                            style.background = Some(
                                Color::from(theme.cosmic().accent_color().with_alpha(0.1)).into(),
                            );
                        } else {
                            style.background =
                                Some(Color::from(theme.cosmic().bg_component_color()).into());
                        }
                        style
                    })))
                    .into()
            })
            .collect::<Vec<_>>();

        Self {
            id: Id::unique(),
            info,
            on_create_dnd_source: Box::new(on_create_dnd_source),
            // on_dnd_task_produced: Box::new(on_dnd_task_produced),
            on_reorder: Box::new(on_reorder),
            on_finish: Some(on_apply_reorder),
            on_cancel: Some(on_cancel),
            inner: if applet_buttons.is_empty() {
                container(
                    text::body(fl!("place-here"))
                        .class(theme::Text::Color(
                            theme::active()
                                .cosmic()
                                .on_bg_component_color()
                                .with_alpha(0.75)
                                .into(),
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center(),
                )
                .width(Length::Fill)
                .height(Length::Fixed(48.0))
                .padding(8)
                .class(theme::Container::Custom(Box::new(|theme| {
                    container::Style {
                        border: Border {
                            radius: theme.cosmic().radius_s().into(),
                            color: theme.cosmic().bg_divider().with_alpha(0.5).into(),
                            width: 2.0,
                        },
                        ..Default::default()
                    }
                })))
                .into()
            } else {
                Column::with_children(applet_buttons)
                    .spacing(space_xxs)
                    .into()
            },
            active_applet_offer: active_dnd,
        }
    }

    #[must_use]
    /// reorders the list of applets given:
    /// - the bounds of the list
    /// - the current mouse position during a drag
    /// - the applet being offered to this list
    fn get_reordered(
        &self,
        layout: &layout::Layout,
        pos: Point,
        offered_applet: Applet<'a>,
    ) -> Vec<Applet<'a>> {
        let space_xxs = theme::spacing().space_xxs;
        let mut reordered: Vec<_> = self.info.clone();

        if !layout.bounds().contains(pos) {
            // applets shouldn't be in two lists at once
            reordered.retain(|a| a != &offered_applet);

            return reordered;
        }

        // special case
        if reordered.is_empty() {
            reordered.push(offered_applet);
            return reordered;
        }

        // special case
        if reordered.len() == 1 && reordered[0] == offered_applet {
            return reordered;
        }

        let height = (layout.bounds().height - space_xxs as f32 * (self.info.len() - 1) as f32)
            / self.info.len() as f32;

        let mut found = false;
        let mut y = layout.bounds().y;

        for i in 0..=reordered.len() {
            if i == 0 || i == reordered.len() {
                y += height / 2.0;
            } else {
                y += height + space_xxs as f32;
            }
            if pos.y <= y {
                reordered.insert(i, offered_applet.clone());
                let mut index = 0;
                reordered.retain(|a| {
                    let ret = a != &offered_applet || index == i;
                    index += 1;
                    ret
                });

                found = true;
                break;
            }
        }
        if !found {
            reordered.retain(|a| a != &offered_applet);
            reordered.push(offered_applet);
        }

        reordered
    }

    /// Returns the drag id of the destination.
    ///
    /// # Panics
    /// Panics if the destination has been assigned a Set id, which is invalid.
    #[must_use]
    pub fn get_drag_id(&self) -> u128 {
        match &self.id.0 {
            Internal::Unique(id) | Internal::Custom(id, _) => *id as u128,
            Internal::Set(_) => panic!("Invalid Id assigned to dnd destination."),
        }
    }
}

#[must_use]
/// mark this as a dnd icon
pub fn dnd_icon(info: Applet<'static>, layout: &layout::Layout) -> AppletReorderList<'static, ()> {
    AppletReorderList::<'static, ()> {
        id: Id::unique(),
        info: Vec::new(),
        on_create_dnd_source: Box::new(|_| unimplemented!()),
        // on_dnd_task_produced: Box::new(|_| unimplemented!()),
        on_reorder: Box::new(|_| unimplemented!()),
        on_finish: None,
        inner: container(
            row::with_children(vec![
                icon::from_name("grip-lines-symbolic")
                    .size(16)
                    .symbolic(true)
                    .into(),
                icon::from_name(info.icon.into_owned()).size(32).into(),
                column::with_capacity(2)
                    .spacing(4.0)
                    .width(Length::Fill)
                    .push(text::body(info.name))
                    .push_maybe(if info.description.is_empty() {
                        None
                    } else {
                        Some(text::caption(info.description))
                    })
                    .into(),
                button::icon(icon::from_name("edit-delete-symbolic"))
                    .extra_small()
                    .into(),
            ])
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .width(Length::Fixed(layout.bounds().width))
        .padding(8)
        .class(theme::Container::Custom(Box::new(move |theme| {
            let mut style = container::Catalog::style(theme, &theme::Container::Primary);
            style.background = Some(Color::from(theme.cosmic().bg_component_color()).into());
            style.border.radius = theme.cosmic().radius_s().into();
            style.border.color = theme.cosmic().bg_divider().into();
            style.border.width = 1.0;
            style
        })))
        .into(),
        on_cancel: None,
        active_applet_offer: None,
    }
}

impl<Message: 'static> Widget<Message, cosmic::Theme, cosmic::Renderer>
    for AppletReorderList<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ReorderWidgetState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ReorderWidgetState::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.inner)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut [&mut self.inner]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &cosmic::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let inner_layout = self
            .inner
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits);
        layout::Node::with_children(inner_layout.size(), vec![inner_layout])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &cosmic::Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let state = tree.state.downcast_mut::<ReorderWidgetState>();

        operation.custom(state, Some(&self.id));

        self.inner.as_widget().operate(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    #[allow(clippy::too_many_lines, clippy::needless_match)]
    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor_position: mouse::Cursor,
        renderer: &cosmic::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let space_xxs = theme::spacing().space_xxs;
        let mut ret = match self.inner.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            event::Status::Captured => return event::Status::Captured,
            event::Status::Ignored => event::Status::Ignored,
        };

        let height = (layout.bounds().height
            - space_xxs as f32 * (self.info.len().saturating_sub(1)) as f32)
            / self.info.len() as f32;
        let state = tree.state.downcast_mut::<ReorderWidgetState>();

        state.dragging_state = {
            match mem::take(&mut state.dragging_state) {
                DraggingState::Dragging(applet) => match &event {
                    event::Event::Dnd(DndEvent::Source(source_event)) => match source_event {
                        SourceEvent::Cancelled => {
                            ret = event::Status::Captured;
                            if let Some(on_cancel) = self.on_cancel.clone() {
                                shell.publish(on_cancel);
                            }
                            DraggingState::None
                        }
                        SourceEvent::Finished => {
                            ret = event::Status::Captured;

                            DraggingState::None
                        }
                        _ => DraggingState::Dragging(applet),
                    },
                    _ => DraggingState::Dragging(applet),
                },
                DraggingState::None => {
                    // if no dragging state, listen for press events
                    match &event {
                        event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                        | event::Event::Touch(touch::Event::FingerPressed { .. })
                            if cursor_position.is_over(layout.bounds()) =>
                        {
                            ret = event::Status::Captured;

                            DraggingState::Pressed(cursor_position.position().unwrap_or_default())
                        }
                        _ => DraggingState::None,
                    }
                }
                DraggingState::Pressed(start) => {
                    // if dragging state is pressed, listen for motion events or release events
                    match &event {
                        event::Event::Mouse(mouse::Event::CursorMoved { .. })
                        | event::Event::Touch(touch::Event::FingerMoved { .. }) => {
                            let pos = cursor_position.position().unwrap_or_default();
                            let d_y = pos.y - start.y;
                            let d_x = pos.x - start.x;
                            let distance_squared = d_y * d_y + d_x * d_x;

                            if distance_squared > DRAG_START_DISTANCE_SQUARED {
                                if let Some((_, applet)) =
                                    self.info.iter().enumerate().find(|(i, _)| {
                                        start.y
                                            < layout.bounds().y
                                                + (*i + 1) as f32 * (height + space_xxs as f32)
                                    })
                                {
                                    let applet = applet.clone().into_owned();
                                    state.dragging_state = DraggingState::Dragging(applet.clone());

                                    state.layout = Some(layout.bounds().size());
                                    shell.publish((self.on_create_dnd_source.as_ref())(
                                        applet.clone(),
                                    ));

                                    let p = applet.path.to_path_buf();
                                    iced_core::clipboard::start_dnd::<
                                        cosmic::Theme,
                                        cosmic::Renderer,
                                    >(
                                        clipboard,
                                        false,
                                        Some(iced_core::clipboard::DndSource::Widget(
                                            self.id.clone(),
                                        )),
                                        Some(IconSurface::new(
                                            dnd_icon(applet.clone(), &layout).into(),
                                            iced_core::widget::tree::State::new(state.clone()),
                                            iced_core::Vector::new(0.0, 0.0),
                                        )),
                                        Box::new(AppletString(p.clone())),
                                        DndAction::Move,
                                    );
                                    ret = event::Status::Captured;
                                    let reordered = self
                                        .info
                                        .iter()
                                        .filter(|a| {
                                            applet != **a
                                        })
                                        .cloned()
                                        .map(pages::desktop::panel::applets_inner::Applet::into_owned)
                                        .collect();
                                    shell.publish((self.on_reorder.as_ref())(reordered));
                                    DraggingState::Dragging(applet.clone().into_owned())
                                } else {
                                    DraggingState::Pressed(start)
                                }
                            } else {
                                DraggingState::Pressed(start)
                            }
                        }
                        event::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                        | event::Event::Touch(
                            touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. },
                        ) => {
                            ret = event::Status::Captured;
                            DraggingState::None
                        }
                        _ => DraggingState::Pressed(start),
                    }
                }
            }
        };

        state.dnd_offer = match mem::take(&mut state.dnd_offer) {
            DndOfferState::None => match &event {
                event::Event::Dnd(dnd_event) => match dnd_event {
                    DndEvent::Offer(rectangle, OfferEvent::Enter { x, y, .. })
                        if *rectangle == Some(self.get_drag_id()) =>
                    {
                        if let Some(data) = self.active_applet_offer.clone() {
                            let reordered_list = self.get_reordered(
                                &layout,
                                Point {
                                    x: *x as f32,
                                    y: *y as f32,
                                },
                                data,
                            );
                            if reordered_list != self.info {
                                shell.publish((self.on_reorder.as_ref())(
                                    reordered_list
                                        .into_iter()
                                        .map(pages::desktop::panel::applets_inner::Applet::into_owned)
                                        .collect(),
                                ));
                            }
                        }
                        DndOfferState::HandlingOffer
                    }
                    _ => DndOfferState::None,
                },
                _ => DndOfferState::None,
            },
            DndOfferState::HandlingOffer => match &event {
                event::Event::Dnd(dnd_event) => match dnd_event {
                    DndEvent::Offer(rectangle, OfferEvent::Motion { x, y })
                        if *rectangle == Some(self.get_drag_id()) =>
                    {
                        if let Some(data) = self.active_applet_offer.clone() {
                            let reordered_list = self.get_reordered(
                                &layout,
                                Point {
                                    x: *x as f32,
                                    y: *y as f32,
                                },
                                data,
                            );
                            if reordered_list != self.info {
                                shell.publish((self.on_reorder.as_ref())(
                                    reordered_list
                                        .into_iter()
                                        .map(pages::desktop::panel::applets_inner::Applet::into_owned)
                                        .collect(),
                                ));
                            }
                        }
                        DndOfferState::HandlingOffer
                    }
                    DndEvent::Offer(
                        rectangle,
                        OfferEvent::LeaveDestination | OfferEvent::Leave,
                    ) if *rectangle == Some(self.get_drag_id()) => {
                        let reordered = self
                            .info
                            .iter()
                            .filter(|a| {
                                !self
                                    .active_applet_offer
                                    .as_ref()
                                    .is_some_and(|offer| offer == *a)
                            })
                            .cloned()
                            .map(pages::desktop::panel::applets_inner::Applet::into_owned)
                            .collect();
                        shell.publish((self.on_reorder.as_ref())(reordered));

                        DndOfferState::None
                    }
                    DndEvent::Offer(rectangle, OfferEvent::Data { .. })
                        if *rectangle == Some(self.get_drag_id()) =>
                    {
                        if let Some(on_finish) = self.on_finish.clone() {
                            shell.publish(on_finish);
                        }

                        DndOfferState::None
                    }
                    _ => DndOfferState::HandlingOffer,
                },
                _ => DndOfferState::HandlingOffer,
            },
        };

        ret
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut cosmic::Renderer,
        theme: &cosmic::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.inner.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        renderer: &cosmic::Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, cosmic::Theme, cosmic::Renderer>> {
        self.inner.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: layout::Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &cosmic::Renderer,
    ) -> mouse::Interaction {
        match self.inner.as_widget().mouse_interaction(
            &state.children[0],
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        ) {
            mouse::Interaction::Idle => {
                let state = state.state.downcast_ref::<ReorderWidgetState>();
                if matches!(state.dragging_state, DraggingState::Dragging(_)) {
                    mouse::Interaction::Grabbing
                } else if cursor_position.is_over(layout.bounds()) {
                    mouse::Interaction::Grab
                } else {
                    mouse::Interaction::default()
                }
            }
            interaction => interaction,
        }
    }

    fn id(&self) -> Option<Id> {
        Some(self.id.clone())
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }

    fn drag_destinations(
        &self,
        _state: &Tree,
        layout: layout::Layout<'_>,
        _renderer: &cosmic::Renderer,
        dnd_rectangles: &mut cosmic::iced_core::clipboard::DndDestinationRectangles,
    ) {
        let Rectangle {
            x,
            y,
            width,
            height,
        } = layout.bounds();
        dnd_rectangles.push(DndDestinationRectangle {
            id: self.get_drag_id(),
            rectangle: cosmic::iced::clipboard::dnd::Rectangle {
                x: x as f64,
                y: y as f64,
                width: width as f64,
                height: height as f64,
            },
            mime_types: vec![Cow::Owned(MIME_TYPE.to_string())],
            actions: DndAction::Move,
            preferred: DndAction::Move,
        });
    }
}

impl AsMimeTypes for AppletString {
    fn available(&self) -> Cow<'static, [String]> {
        Cow::Owned(vec![MIME_TYPE.to_string()])
    }

    fn as_bytes(&self, mime_type: &str) -> Option<Cow<'static, [u8]>> {
        if mime_type == MIME_TYPE {
            Some(Cow::Owned(
                url::Url::from_file_path(self.0.clone())
                    .ok()?
                    .to_string()
                    .as_bytes()
                    .to_vec(),
            ))
        } else {
            None
        }
    }
}
/// A string which can be sent to the clipboard or drag-and-dropped.
#[derive(Debug, Clone)]
pub struct AppletString(PathBuf);

#[derive(Debug, Default, Clone)]
pub enum DraggingState {
    #[default]
    /// No ongoing drag or press
    None,
    /// A draggable item was being pressed at the recorded point
    Pressed(Point),
    /// An item is being dragged
    Dragging(Applet<'static>),
}

#[derive(Debug, Default, Clone)]
pub(crate) enum DndOfferState {
    #[default]
    None,
    HandlingOffer,
}

#[derive(Debug, Default, Clone)]
pub struct ReorderWidgetState {
    dragging_state: DraggingState,
    dnd_offer: DndOfferState,
    layout: Option<Size>,
}

impl ReorderWidgetState {
    pub(crate) fn new() -> ReorderWidgetState {
        ReorderWidgetState::default()
    }
}

impl<'a, Message: 'static + Clone> From<AppletReorderList<'a, Message>> for Element<'a, Message> {
    fn from(applet_reorder_list: AppletReorderList<'a, Message>) -> Self {
        Element::new(applet_reorder_list)
    }
}
#[derive(Debug, Clone)]
pub enum State {
    DndIcon(ReorderWidgetState),
}
