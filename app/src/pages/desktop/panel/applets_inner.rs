use button::StyleSheet as ButtonStyleSheet;
use cosmic::iced_style::container::StyleSheet;

use cosmic::widget::{
    button, column, container, header_bar, icon, list_column, row, scrollable, text, text_input,
    Column,
};

use cosmic::{
    cosmic_config::{Config, CosmicConfigEntry},
    iced::{
        alignment::{Horizontal, Vertical},
        event::{
            self,
            wayland::{self},
            PlatformSpecific,
        },
        mouse, overlay, touch,
        wayland::actions::{
            data_device::{ActionInner, DataFromMimeType, DndIcon},
            window::SctkWindowSettings,
        },
        wayland::data_device::action as data_device_action,
        window, Alignment, Color, Length, Point, Rectangle, Size,
    },
    iced_runtime::{command::platform_specific, core::id::Id, Command},
    iced_sctk::commands,
    iced_widget::{
        core::{
            layout, renderer,
            widget::{tree, Operation, OperationOutputWrapper, Tree},
            Clipboard, Shell, Widget,
        },
        graphics::image::image_rs::EncodableLayout,
    },
    sctk::reexports::client::protocol::wl_data_device_manager::DndAction,
    theme, Apply, Element,
};

use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    mem,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use crate::{app, pages};
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section, Section};
use freedesktop_desktop_entry::DesktopEntry;
use slotmap::SlotMap;
use tracing::error;

const MIME_TYPE: &str = "text/uri-list";

pub type OnDndCommand<'a, Message> = Box<
    dyn Fn(
            Box<dyn Send + Sync + Fn() -> platform_specific::wayland::data_device::ActionInner>,
        ) -> Message
        + 'a,
>;

const SPACING: f32 = 8.0;

// radius is 8.0
const DRAG_START_DISTANCE_SQUARED: f32 = 64.0;

pub const APPLET_DND_ICON_ID: window::Id = window::Id(1000);
pub const ADD_PANEL_APPLET_DIALOGUE_ID: window::Id = window::Id(1001);

pub struct Page {
    pub(crate) available_entries: Vec<Applet<'static>>,
    pub(crate) config_helper: Option<Config>,
    pub(crate) current_config: Option<CosmicPanelConfig>,
    pub(crate) reorder_widget_state: ReorderWidgetState,
    pub(crate) search: String,
    pub(crate) has_dialogue: bool,
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
            available_entries: freedesktop_desktop_entry::Iter::new(
                freedesktop_desktop_entry::default_paths(),
            )
            .filter_map(|p| Applet::try_from(Cow::from(p)).ok())
            .collect(),
            config_helper,
            current_config,
            reorder_widget_state: ReorderWidgetState::default(),
            search: String::new(),
            has_dialogue: false,
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
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(lists::<Page, _>(pages::Message::PanelApplet))
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("panel_applets", "preferences-pop-desktop-dock-symbolic")
        // .title(fl!("applets"))
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
    PanelConfig(CosmicPanelConfig),
    StartDnd(ReorderWidgetState),
    DnDCommand(Arc<Box<dyn Send + Sync + Fn() -> ActionInner>>),
    Search(String),
    AddApplet(Applet<'static>),
    AddAppletDialogue,
    CloseAppletDialogue,
    ClosedAppletDialogue,
    DragAppletDialogue,
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
            Message::DnDCommand(_) => write!(f, "DnDCommand"),
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
            Message::AddAppletDialogue => write!(f, "AddAppletDialogue"),
            Message::CloseAppletDialogue => write!(f, "CloseAppletDialogue"),
            Message::DragAppletDialogue => write!(f, "DragAppletDialogue"),
            Message::ClosedAppletDialogue => write!(f, "ClosedAppletDialogue"),
        }
    }
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
    pub fn dnd_icon(&self) -> Element<app::Message> {
        Element::from(AppletReorderList::dnd_icon(&self.reorder_widget_state))
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn add_applet_view<T: Fn(Message) -> crate::pages::Message + Copy + 'static>(
        &self,
        msg_map: T,
    ) -> Element<app::Message> {
        let mut list_column = list_column();
        let mut has_some = false;
        for info in self
            .available_entries
            .iter()
            .filter(|a| a.matches(&self.search))
        {
            if let Some(config) = self.current_config.as_ref() {
                if let Some(center) = config.plugins_center.as_ref() {
                    if center.iter().any(|a| a.as_str() == info.id.as_ref()) {
                        continue;
                    }
                }

                if let Some(wings) = config.plugins_wings.as_ref() {
                    if wings
                        .0
                        .iter()
                        .chain(wings.1.iter())
                        .any(|a| a.as_str() == info.id.as_ref())
                    {
                        continue;
                    }
                }
            }
            has_some = true;
            list_column = list_column.add(
                row::with_children(vec![
                    icon::from_name(&*info.icon)
                        .size(32)
                        .symbolic(true)
                        .icon()
                        .into(),
                    column::with_capacity(2)
                        .push(text(info.name.clone()))
                        .push(text(info.description.clone()).size(10))
                        .spacing(4.0)
                        .width(Length::Fill)
                        .into(),
                    button(text(fl!("add")))
                        .style(button::Style::Custom {
                            active: Box::new(|focused, theme| {
                                let mut style = theme.active(focused, &button::Style::Text);
                                style.text_color = Some(theme.cosmic().accent_color().into());
                                style
                            }),
                            disabled: Box::new(|theme| {
                                let mut style = theme.disabled(&button::Style::Text);
                                let mut text_color: Color = theme.cosmic().accent_color().into();
                                text_color.a *= 0.5;
                                style.text_color = Some(text_color);
                                style
                            }),
                            hovered: Box::new(|focused, theme| {
                                let mut style = theme.hovered(focused, &theme::Button::Text);
                                style.text_color = Some(theme.cosmic().accent_color().into());
                                style
                            }),
                            pressed: Box::new(|focused, theme| {
                                let mut style = theme.pressed(focused, &theme::Button::Text);
                                style.text_color = Some(theme.cosmic().accent_color().into());
                                style
                            }),
                        })
                        .padding(8.0)
                        .on_press(app::Message::PageMessage(msg_map(Message::AddApplet(
                            info.clone(),
                        ))))
                        .into(),
                ])
                .padding([0, 32, 0, 32])
                .spacing(12)
                .align_items(Alignment::Center),
            );
        }
        if !has_some {
            list_column = list_column.add(
                text(fl!("no-applets-found"))
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            );
        }
        column::with_children(vec![
            header_bar()
                .title(fl!("add-applet"))
                .on_close(app::Message::PageMessage(msg_map(
                    Message::CloseAppletDialogue,
                )))
                .on_drag(app::Message::PageMessage(msg_map(
                    Message::DragAppletDialogue,
                )))
                .into(),
            container(
                scrollable(
                    column::with_children(vec![
                        text(fl!("add-applet")).size(24).width(Length::Fill).into(),
                        text_input::search_input(&fl!("search-applets"), &self.search)
                            .on_input(move |s| {
                                app::Message::PageMessage(msg_map(Message::Search(s)))
                            })
                            .on_paste(move |s| {
                                app::Message::PageMessage(msg_map(Message::Search(s)))
                            })
                            .width(Length::Fixed(312.0))
                            .into(),
                        list_column.into(),
                    ])
                    .padding([0, 64, 32, 64])
                    .align_items(Alignment::Center)
                    .spacing(8.0),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .style(theme::Container::Background)
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        ])
        .into()
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message, window_id: window::Id) -> Command<app::Message> {
        match message {
            Message::PanelConfig(c) => {
                self.current_config = Some(c);
            }

            Message::ReorderStart(start_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((list, _)) = config.plugins_wings.as_mut() else {
                    config.plugins_wings = Some((start_list.into_iter().map(|a: Applet| a.id.into()).collect(), Vec::new()));
                    return Command::none();
                };
                *list = start_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::ReorderCenter(center_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some(list) = config.plugins_center.as_mut() else {
                    config.plugins_center = Some(center_list.into_iter().map(|a: Applet| a.id.into()).collect());
                    return Command::none();
                };
                *list = center_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::ReorderEnd(end_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((_, list)) = config.plugins_wings.as_mut() else {
                    config.plugins_wings = Some((Vec::new(), end_list.into_iter().map(|a: Applet| a.id.into()).collect()));
                    return Command::none();
                };
                *list = end_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::Applets(applets) => {
                self.available_entries = applets;
            }
            Message::StartDnd(state) => {
                self.reorder_widget_state = state;
                return Command::none();
            }
            Message::DnDCommand(action) => {
                return data_device_action(action());
            }
            Message::Save => {
                self.reorder_widget_state = ReorderWidgetState::default();
                self.save();
            }
            Message::RemoveStart(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((list, _)) = config.plugins_wings.as_mut() else {
                    return Command::none();
                };
                list.retain(|id| id != &to_remove);
                self.save();
            }
            Message::RemoveCenter(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some(list) = config.plugins_center.as_mut() else {
                    return Command::none();
                };
                list.retain(|id| id != &to_remove);
                self.save();
            }
            Message::RemoveEnd(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((_, list)) = config.plugins_wings.as_mut() else {
                    return Command::none();
                };
                list.retain(|id| id != &to_remove);
                self.save();
            }
            Message::DetailStart(_) => {
                // TODO ask design team
            }
            Message::DetailCenter(_) => {
                // TODO ask design team
            }
            Message::DetailEnd(_) => {
                // TODO ask design team
            }
            Message::Cancel => {
                self.reorder_widget_state = ReorderWidgetState::default();
                let current_config = self.config_helper.as_ref().and_then(|config_helper| {
                    // TODO error handling...
                    let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
                    // If the config is not present, it will be created with the default values and the name will not match
                    (panel_config.name == "Panel").then_some(panel_config)
                });
                self.current_config = current_config;
            }
            Message::Search(text) => {
                self.search = text;
            }
            Message::AddApplet(applet) => {
                // TODO ask design team
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let list = if let Some((list, _)) = config.plugins_wings.as_mut() {
                    list
                } else {
                    config.plugins_wings = Some((Vec::new(), Vec::new()));
                    &mut config.plugins_wings.as_mut().unwrap().0
                };

                list.push(applet.id.to_string());
                self.save();
                return commands::window::close_window(window_id);
            }
            Message::AddAppletDialogue => {
                self.has_dialogue = true;
                let window_settings = SctkWindowSettings {
                    window_id,
                    app_id: Some("com.system76.CosmicSettings".to_string()),
                    title: Some(fl!("add-applet")),
                    parent: Some(window::Id(0)),
                    autosize: false,
                    size_limits: layout::Limits::NONE
                        .min_width(300.0)
                        .max_width(800.0)
                        .min_height(200.0)
                        .max_height(1080.0),
                    size: (512, 420),
                    resizable: None,
                    client_decorations: true,
                    transparent: true,
                    ..Default::default()
                };
                return commands::window::get_window(window_settings);
            }
            Message::ClosedAppletDialogue => {
                self.has_dialogue = false;
            }
            Message::CloseAppletDialogue => {
                self.has_dialogue = false;
                return commands::window::close_window(window_id);
            }
            Message::DragAppletDialogue => {
                return commands::window::start_drag_window(window_id);
            }
        };
        Command::none()
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
        let page = page.inner();
        let Some(config) = page.current_config.as_ref() else {
            return Element::from(
                text(fl!("unknown"))
            );
        };

        let button = button::standard(fl!("add-applet"));

        column::with_children(vec![
            column::with_children(vec![
                row::with_children(vec![
                    text(fl!("applets")).width(Length::Fill).size(24).into(),
                    (if page.has_dialogue {
                        button
                    } else {
                        button.on_press(Message::AddAppletDialogue)
                    })
                    .into(),
                ])
                .into(),
                text(fl!("start-segment")).into(),
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
                    Some((window::Id(0), APPLET_DND_ICON_ID)),
                    Message::StartDnd,
                    |a| Message::DnDCommand(Arc::new(a)),
                    Message::RemoveStart,
                    Message::DetailStart,
                    Message::ReorderStart,
                    Message::Save,
                    Message::Cancel,
                    page.reorder_widget_state.dragged_applet().as_ref(),
                )
                .into(),
            ])
            .spacing(8.0)
            .into(),
            column::with_children(vec![
                text(fl!("center-segment")).into(),
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
                    Some((window::Id(0), APPLET_DND_ICON_ID)),
                    Message::StartDnd,
                    |a| Message::DnDCommand(Arc::new(a)),
                    Message::RemoveCenter,
                    Message::DetailCenter,
                    Message::ReorderCenter,
                    Message::Save,
                    Message::Cancel,
                    page.reorder_widget_state.dragged_applet().as_ref(),
                )
                .into(),
            ])
            .spacing(8.0)
            .into(),
            column::with_children(vec![
                text(fl!("end-segment")).into(),
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
                    Some((window::Id(0), APPLET_DND_ICON_ID)),
                    Message::StartDnd,
                    |a| Message::DnDCommand(Arc::new(a)),
                    Message::RemoveEnd,
                    Message::DetailEnd,
                    Message::ReorderEnd,
                    Message::Save,
                    Message::Cancel,
                    page.reorder_widget_state.dragged_applet().as_ref(),
                )
                .into(),
            ])
            .spacing(8.0)
            .into(),
        ])
        .padding([0, 16, 0, 16])
        .spacing(12.0)
        .apply(Element::from)
        .map(msg_map)
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        self.name.contains(query) || self.description.contains(query) || self.id.contains(query)
    }
}

impl<'a> TryFrom<Cow<'a, Path>> for Applet<'static> {
    type Error = anyhow::Error;

    fn try_from(path: Cow<'a, Path>) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let entry = DesktopEntry::decode(path.as_ref(), &content)?;
        if entry.desktop_entry("X-CosmicApplet").is_none() {
            anyhow::bail!("Not an applet");
        }

        Ok(Self {
            id: Cow::from(entry.id().to_string()),
            name: Cow::from(entry.name(None).unwrap_or_default().to_string()),
            description: Cow::from(entry.comment(None).unwrap_or_default().to_string()),
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

impl<'a> Applet<'a> {
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
    on_create_dnd_source: Box<dyn Fn(ReorderWidgetState) -> Message + 'a>,
    on_dnd_command_produced: OnDndCommand<'a, Message>,
    on_reorder: Box<dyn Fn(Vec<Applet<'static>>) -> Message + 'a>,
    on_finish: Option<Message>,
    on_cancel: Option<Message>,
    surface_ids: Option<(window::Id, window::Id)>,
    inner: Element<'a, Message>,
}

impl<'a, Message: 'static + Clone> AppletReorderList<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    /// new applet list which can be reordered and dragged
    pub fn new(
        info: Vec<Applet<'a>>,
        surface_ids: Option<(window::Id, window::Id)>,
        on_create_dnd_source: impl Fn(ReorderWidgetState) -> Message + 'a,
        on_dnd_command_produced: impl Fn(
                Box<dyn Send + Sync + Fn() -> platform_specific::wayland::data_device::ActionInner>,
            ) -> Message
            + 'a,
        on_remove: impl Fn(String) -> Message + 'a,
        on_details: impl Fn(String) -> Message + 'a,
        on_reorder: impl Fn(Vec<Applet<'static>>) -> Message + 'a,
        on_apply_reorder: Message,
        on_cancel: Message,
        active_dnd: Option<&Applet<'a>>,
    ) -> Self {
        let applet_buttons = info
            .clone()
            .into_iter()
            .map(|info| {
                let id_clone = info.id.to_string();
                let is_dragged = active_dnd.as_ref().map_or(false, |dnd| dnd.id == info.id);
                container(
                    row::with_children(vec![
                        icon::from_name("open-menu-symbolic")
                            .symbolic(true)
                            .size(16)
                            .into(),
                        icon::from_name(info.icon).size(32).symbolic(true).into(),
                        column::with_capacity(2)
                            .spacing(4.0)
                            .width(Length::Fill)
                            .push(text(info.name))
                            .push(text::caption(info.description))
                            .into(),
                        button::icon(icon::from_name("edit-delete-symbolic"))
                            .extra_small()
                            .on_press(on_remove(id_clone.clone()))
                            .into(),
                        button::icon(icon::from_name("open-menu-symbolic"))
                            .extra_small()
                            .on_press(on_details(id_clone))
                            .into(),
                    ])
                    .spacing(12)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .padding(8)
                .style(theme::Container::Custom(Box::new(move |theme| {
                    let mut style = theme.appearance(&theme::Container::Primary);
                    style.border_radius = 8.0.into();
                    if is_dragged {
                        style.border_color = theme.cosmic().accent_color().into();
                        style.border_width = 2.0;
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
            on_dnd_command_produced: Box::new(on_dnd_command_produced),
            on_reorder: Box::new(on_reorder),
            on_finish: Some(on_apply_reorder),
            on_cancel: Some(on_cancel),
            surface_ids,
            inner: if active_dnd.is_some() && applet_buttons.is_empty() {
                container(
                    text(fl!("drop-here"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center),
                )
                .width(Length::Fill)
                .height(Length::Fixed(48.0))
                .padding(8)
                .style(theme::Container::Custom(Box::new(move |theme| {
                    let mut style = theme.appearance(&theme::Container::Primary);
                    style.border_radius = 8.0.into();
                    style.border_color = theme.cosmic().bg_divider().into();
                    style.border_width = 2.0;
                    style.background = Some(Color::TRANSPARENT.into());
                    style
                })))
                .into()
            } else {
                Column::with_children(applet_buttons)
                    .spacing(SPACING)
                    .into()
            },
        }
    }

    #[must_use]
    /// mark this as a dnd icon
    pub fn dnd_icon(state: &'a ReorderWidgetState) -> Self {
        Self {
            id: Id::unique(),
            info: Vec::new(),
            on_create_dnd_source: Box::new(|_| unimplemented!()),
            on_dnd_command_produced: Box::new(|_| unimplemented!()),
            on_reorder: Box::new(|_| unimplemented!()),
            on_finish: None,
            surface_ids: None,
            inner: if let Some(info) = state.dragged_applet() {
                container(
                    row::with_children(vec![
                        icon::from_name("open-menu-symbolic")
                            .size(16)
                            .symbolic(true)
                            .into(),
                        icon::from_name(info.icon.into_owned())
                            .size(32)
                            .symbolic(true)
                            .into(),
                        column::with_capacity(2)
                            .spacing(4.0)
                            .width(Length::Fill)
                            .push(text(info.name))
                            .push(text::caption(info.description))
                            .into(),
                        button::icon(icon::from_name("edit-delete-symbolic"))
                            .extra_small()
                            .into(),
                        button::icon(icon::from_name("open-menu-symbolic"))
                            .extra_small()
                            .into(),
                    ])
                    .spacing(12)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fixed(state.layout.map_or(400.0, |l| l.width)))
                .padding(8)
                .style(theme::Container::Custom(Box::new(move |theme| {
                    let mut style = theme.appearance(&theme::Container::Primary);
                    style.border_radius = 8.0.into();
                    style
                })))
                .into()
            } else {
                text("unknown").into()
            },
            on_cancel: None,
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

        let height = (layout.bounds().height - SPACING * (self.info.len() - 1) as f32)
            / self.info.len() as f32;

        let mut found = false;
        let mut y = layout.bounds().y;

        for i in 0..=reordered.len() {
            if i == 0 || i == reordered.len() {
                y += height / 2.0;
            } else {
                y += height + SPACING;
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
}

impl<'a, Message: 'static> Widget<Message, cosmic::Renderer> for AppletReorderList<'a, Message>
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
        tree.diff_children(std::slice::from_mut(&mut self.inner));
    }

    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &cosmic::Renderer, limits: &layout::Limits) -> layout::Node {
        let inner_layout = self.inner.as_widget().layout(renderer, limits);
        layout::Node::with_children(inner_layout.size(), vec![inner_layout])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &cosmic::Renderer,
        operation: &mut dyn Operation<OperationOutputWrapper<Message>>,
    ) {
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

        let height = (layout.bounds().height - SPACING * (self.info.len() - 1) as f32)
            / self.info.len() as f32;
        let state = tree.state.downcast_mut::<ReorderWidgetState>();

        state.dragging_state = match mem::take(&mut state.dragging_state) {
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
            DraggingState::Dragging(applet) => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DataSource(wayland::DataSourceEvent::DndFinished),
                )) => {
                    ret = event::Status::Captured;
                    DraggingState::None
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DataSource(wayland::DataSourceEvent::Cancelled),
                )) => {
                    ret = event::Status::Captured;
                    if let Some(on_cancel) = self.on_cancel.clone() {
                        shell.publish(on_cancel);
                    }
                    DraggingState::None
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DataSource(wayland::DataSourceEvent::DndDropPerformed),
                )) => {
                    ret = event::Status::Captured;

                    DraggingState::None
                }
                _ => DraggingState::Dragging(applet),
            },
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
                                        < layout.bounds().y + (*i + 1) as f32 * (height + SPACING)
                                })
                            {
                                let (window_id, icon_id) = self.surface_ids.unwrap();
                                state.dragging_state =
                                    DraggingState::Dragging(applet.clone().into_owned());

                                // TODO emit a dnd command
                                state.layout = Some(layout.bounds().size());
                                let state_clone = state.clone();
                                shell.publish((self.on_create_dnd_source.as_ref())(
                                    state_clone.clone(),
                                ));

                                let p = applet.path.to_path_buf();
                                shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(move || {
                                    platform_specific::wayland::data_device::ActionInner::StartDnd {
                                        mime_types: vec![MIME_TYPE.to_string()],
                                        actions: DndAction::Move,
                                        origin_id: window_id,
                                        icon_id: Some(DndIcon::Widget(
                                            icon_id,
                                            Box::new(state_clone.clone()),
                                        )),
                                        data: Box::new(AppletString(p.clone())),
                                    }
                                })));
                                ret = event::Status::Captured;
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
        };
        state.dnd_offer = match mem::take(&mut state.dnd_offer) {
            DndOfferState::None => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::SourceActions(actions)),
                )) => DndOfferState::OutsideWidget(Vec::new(), *actions, None),
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Enter { x, y, mime_types }),
                )) => {
                    if mime_types.iter().any(|m| m.as_str() == MIME_TYPE) {
                        let point = Point::new(*x as f32, *y as f32);

                        if layout.bounds().contains(point) {
                            shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                                move || {
                                    platform_specific::wayland::data_device::ActionInner::SetActions {
                                        preferred: DndAction::Move,
                                        accepted: DndAction::Move,
                                    }
                                },
                            )));
                            shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                                move || {
                                    platform_specific::wayland::data_device::ActionInner::Accept(
                                        Some(MIME_TYPE.to_string()),
                                    )
                                },
                            )));
                            let data = match &state.dragging_state {
                                DraggingState::Dragging(a) => Some(a.clone()),
                                _ => {
                                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                                    move || {
                                        platform_specific::wayland::data_device::ActionInner::RequestDndData(
                                            MIME_TYPE.to_string(),
                                        )
                                    },
                                )));
                                    None
                                }
                            };
                            DndOfferState::HandlingOffer(
                                mime_types.clone(),
                                DndAction::empty(),
                                data,
                            )
                        } else {
                            let data = match &state.dragging_state {
                                DraggingState::Dragging(data) => {
                                    let filtered: Vec<_> = self
                                        .info
                                        .clone()
                                        .into_iter()
                                        .filter(|a| a != data)
                                        .collect();
                                    if filtered != self.info {
                                        shell.publish((self.on_reorder.as_ref())(
                                            filtered
                                                .into_iter()
                                                .map(pages::desktop::panel::applets_inner::Applet::into_owned)
                                                .collect(),
                                        ));
                                    }
                                    Some(data.clone())
                                }
                                _ => None,
                            };
                            DndOfferState::OutsideWidget(
                                mime_types.clone(),
                                DndAction::empty(),
                                data,
                            )
                        }
                    } else {
                        DndOfferState::None
                    }
                }
                _ => DndOfferState::None,
            },
            DndOfferState::OutsideWidget(mime_types, action, data) => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::SourceActions(actions)),
                )) => DndOfferState::OutsideWidget(mime_types, *actions, data),
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Motion { x, y }),
                )) => {
                    let point = Point::new(*x as f32, *y as f32);

                    if layout.bounds().contains(point) {
                        shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                            move || {
                                platform_specific::wayland::data_device::ActionInner::SetActions {
                                    preferred: DndAction::Move,
                                    accepted: DndAction::Move,
                                }
                            },
                        )));
                        shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                            move || {
                                platform_specific::wayland::data_device::ActionInner::Accept(Some(
                                    MIME_TYPE.to_string(),
                                ))
                            },
                        )));
                        shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                            move || {
                                platform_specific::wayland::data_device::ActionInner::SetActions {
                                    preferred: action.intersection(DndAction::Move),
                                    accepted: action
                                        .intersection(DndAction::Move.union(DndAction::Copy)),
                                }
                            },
                        )));
                        // TODO maybe keep track of data and request here if we don't have it
                        // also maybe just refactor DND Targets to allow easier handling...

                        if data.is_none() {
                            shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                                move || {
                                    platform_specific::wayland::data_device::ActionInner::RequestDndData(
                                        MIME_TYPE.to_string(),
                                    )
                                },
                            )));
                        }
                        if let Some(applet) = data.clone() {
                            let reordered_list: Vec<_> = self.get_reordered(
                                &layout,
                                Point {
                                    x: *x as f32,
                                    y: *y as f32,
                                },
                                applet,
                            );
                            if reordered_list != self.info {
                                shell.publish((self.on_reorder.as_ref())(
                                    reordered_list.into_iter().map(Applet::into_owned).collect(),
                                ));
                            }
                        }

                        DndOfferState::HandlingOffer(mime_types, DndAction::empty(), data)
                    } else {
                        DndOfferState::OutsideWidget(mime_types, DndAction::empty(), data)
                    }
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DndData {
                        data: new_data,
                        mime_type,
                    }),
                )) => {
                    if mime_type.as_str() == MIME_TYPE {
                        let data = std::str::from_utf8(new_data.as_bytes())
                            .ok()
                            .and_then(|s| url::Url::from_str(s).ok())
                            .and_then(|url| url.to_file_path().ok())
                            .and_then(|p| Applet::try_from(Cow::from(p)).ok());
                        DndOfferState::OutsideWidget(mime_types, action, data)
                    } else {
                        DndOfferState::OutsideWidget(mime_types, action, data)
                    }
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(
                        wayland::DndOfferEvent::DropPerformed | wayland::DndOfferEvent::Leave,
                    ),
                )) => DndOfferState::None,
                _ => DndOfferState::OutsideWidget(mime_types, action, data),
            },
            DndOfferState::HandlingOffer(mime_types, action, data) => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Motion { x, y }),
                )) => {
                    let point = Point::new(*x as f32, *y as f32);
                    if layout.bounds().contains(point) {
                        shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                            move || {
                                platform_specific::wayland::data_device::ActionInner::SetActions {
                                    preferred: DndAction::Move,
                                    accepted: DndAction::Move,
                                }
                            },
                        )));
                        if let Some(data) = data.clone() {
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
                        DndOfferState::HandlingOffer(mime_types, DndAction::empty(), data)
                    } else {
                        if let Some(applet) = data.clone() {
                            let reordered_list: Vec<_> = self.get_reordered(
                                &layout,
                                Point {
                                    x: *x as f32,
                                    y: *y as f32,
                                },
                                applet,
                            );
                            if reordered_list != self.info {
                                shell.publish((self.on_reorder.as_ref())(
                                    reordered_list.into_iter().map(Applet::into_owned).collect(),
                                ));
                            }
                        }
                        shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                            move || {
                                platform_specific::wayland::data_device::ActionInner::Accept(None)
                            },
                        )));
                        DndOfferState::OutsideWidget(mime_types, DndAction::empty(), data)
                    }
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Leave),
                )) => DndOfferState::None,
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::SourceActions(actions)),
                )) => {
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::SetActions {
                            preferred: DndAction::Move,
                            accepted: DndAction::Move,
                        },
                    )));
                    DndOfferState::HandlingOffer(mime_types, *actions, data)
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DndData {
                        data: new_data,
                        mime_type,
                    }),
                )) => {
                    if mime_type.as_str() == MIME_TYPE {
                        let data = std::str::from_utf8(new_data.as_bytes())
                            .ok()
                            .and_then(|s| url::Url::from_str(s).ok())
                            .and_then(|url| url.to_file_path().ok())
                            .and_then(|p| Applet::try_from(Cow::from(p)).ok());
                        if let Some(data) = data.borrow() {
                            let filtered: Vec<_> = self
                                .info
                                .clone()
                                .into_iter()
                                .filter(|a| a != data)
                                .collect();
                            if filtered != self.info {
                                shell.publish((self.on_reorder.as_ref())(
                                    filtered
                                        .into_iter()
                                        .map(pages::desktop::panel::applets_inner::Applet::into_owned)
                                        .collect(),
                                ));
                            }
                        }

                        DndOfferState::HandlingOffer(mime_types, action, data)
                    } else {
                        DndOfferState::HandlingOffer(mime_types, action, data)
                    }
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DropPerformed),
                )) => {
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::SetActions {
                            preferred: DndAction::Move,
                            accepted: DndAction::Move,
                        },
                    )));
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || {
                            platform_specific::wayland::data_device::ActionInner::Accept(Some(
                                MIME_TYPE.to_string(),
                            ))
                        },
                    )));
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || {
                            platform_specific::wayland::data_device::ActionInner::RequestDndData(
                                MIME_TYPE.to_string(),
                            )
                        },
                    )));
                    DndOfferState::Dropped
                }
                _ => DndOfferState::HandlingOffer(mime_types, action, data),
            },
            DndOfferState::Dropped => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DndData { .. }),
                )) => {
                    if let Some(on_finish) = self.on_finish.clone() {
                        shell.publish(on_finish);
                    }
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::DndFinished,
                    )));

                    DndOfferState::None
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Leave),
                )) => {
                    // already applied the offer, so we can just finish
                    if let Some(on_cancel) = self.on_cancel.clone() {
                        shell.publish(on_cancel);
                    }

                    DndOfferState::None
                }
                _ => DndOfferState::Dropped,
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
    ) -> Option<overlay::Element<'b, Message, cosmic::Renderer>> {
        self.inner.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
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
}

/// A string which can be sent to the clipboard or drag-and-dropped.
#[derive(Debug, Clone)]
pub struct AppletString(PathBuf);

impl DataFromMimeType for AppletString {
    fn from_mime_type(&self, mime_type: &str) -> Option<Vec<u8>> {
        if mime_type == MIME_TYPE {
            let data = Some(
                url::Url::from_file_path(self.0.clone())
                    .ok()?
                    .to_string()
                    .as_bytes()
                    .to_vec(),
            );
            data
        } else {
            None
        }
    }
}

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
    OutsideWidget(Vec<String>, DndAction, Option<Applet<'static>>),
    HandlingOffer(Vec<String>, DndAction, Option<Applet<'static>>),
    Dropped,
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

    pub(crate) fn dragged_applet(&self) -> Option<Applet<'_>> {
        match &self.dragging_state {
            DraggingState::Dragging(applet) => Some(applet.borrowed()),
            _ => None,
        }
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
