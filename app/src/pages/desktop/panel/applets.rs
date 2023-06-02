use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    fmt::Debug,
    mem,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

use apply::Apply;
use cosmic::{
    cosmic_config::{self, Config, CosmicConfigEntry},
    iced::{
        event::{
            self,
            wayland::{self, Event},
            PlatformSpecific,
        },
        mouse, overlay, touch,
        wayland::actions::data_device::{ActionInner, DataFromMimeType, DndIcon},
        wayland::data_device::action as data_device_action,
        window, Alignment, Length, Point, Rectangle, Size,
    },
    iced_runtime::{
        command::platform_specific,
        core::{self, id::Id},
        Command,
    },
    iced_style::container::StyleSheet,
    iced_widget::{
        column, container,
        core::{
            layout, renderer,
            widget::{tree, Operation, OperationOutputWrapper, Tree},
            Clipboard, Shell, Widget,
        },
        graphics::image::image_rs::EncodableLayout,
        row, text, Column,
    },
    sctk::reexports::client::protocol::wl_data_device_manager::DndAction,
    theme,
    widget::{button, icon},
    Element,
};

use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section, Section};
use freedesktop_desktop_entry::DesktopEntry;
use slotmap::SlotMap;

use crate::{app, pages};

pub struct Page {
    available_entries: Vec<Applet<'static>>,
    panel_config_helper: Option<Config>,
    current_config: Option<CosmicPanelConfig>,
    drag_state: State,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            available_entries: Vec::new(),
            panel_config_helper: None,
            current_config: None,
            drag_state: State::default(),
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(lists())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("panel_applets", "preferences-pop-desktop-dock-symbolic")
            .title(fl!("applets"))
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
    StartDnd(State),
    DnDCommand(Arc<Box<dyn Send + Sync + Fn() -> ActionInner>>),
    ApplyReorder,
    Ignore,
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
            Message::Ignore => write!(f, "Ignore"),
            Message::ApplyReorder => write!(f, "ApplyReorder"),
            Message::RemoveStart(_) => write!(f, "RemoveStart"),
            Message::RemoveCenter(_) => write!(f, "RemoveCenter"),
            Message::RemoveEnd(_) => write!(f, "RemoveEnd"),
            Message::DetailStart(_) => write!(f, "DetailStart"),
            Message::DetailCenter(_) => write!(f, "DetailCenter"),
            Message::DetailEnd(_) => write!(f, "DetailEnd"),
        }
    }
}

impl Page {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::PanelConfig(c) => {
                self.current_config = Some(c);
            }

            Message::ReorderStart(start_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((list, _)) = config.plugins_wings.as_mut() else {
                    return Command::none();
                };
                *list = start_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::ReorderCenter(center_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some(list) = config.plugins_center.as_mut() else {
                    return Command::none();
                };
                *list = center_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::ReorderEnd(end_list) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((_, list)) = config.plugins_wings.as_mut() else {
                    return Command::none();
                };
                *list = end_list.into_iter().map(|a| a.id.into()).collect();
            }
            Message::Applets(applets) => {
                self.available_entries = applets;
            }
            Message::StartDnd(state) => {
                self.drag_state = state;
            }
            Message::DnDCommand(action) => {
                return data_device_action(action());
            }
            Message::Ignore => {}
            Message::ApplyReorder => todo!(),
            Message::RemoveStart(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((list, _)) = config.plugins_wings.as_mut() else {
                    return Command::none();
                };
                list.retain(|id| id != &to_remove);
            }
            Message::RemoveCenter(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some(list) = config.plugins_center.as_mut() else {
                    return Command::none();
                };
                list.retain(|id| id != &to_remove);
            }
            Message::RemoveEnd(to_remove) => {
                let Some(config) = self.current_config.as_mut() else {
                    return Command::none();
                };
                let Some((_, list)) = config.plugins_wings.as_mut() else {
                    return Command::none();
                };
                list.retain(|id| id != &to_remove);
            }
            Message::DetailStart(_) => todo!(),
            Message::DetailCenter(_) => todo!(),
            Message::DetailEnd(_) => todo!(),
        };
        Command::none()
    }
}

pub fn lists() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(|_binder, page, _section| {
        let Some(config) = page.current_config.as_ref() else {
            return Element::from(
                text(fl!("unknown"))
            );
        };
        column![
            column![
                text(fl!("start-segment")),
                AppletReorderList::new(
                    page.available_entries
                        .iter()
                        .filter_map(|a| {
                            config.plugins_wings.as_ref().and_then(|list| {
                                list.0
                                    .iter()
                                    .any(|saved| (saved.as_str() == a.id.as_ref()))
                                    .then(|| a.borrowed())
                            })
                        })
                        .collect(),
                    Some((window::Id(0), APPLET_DND_ICON_ID)),
                    Message::StartDnd,
                    |a| Message::DnDCommand(Arc::new(a)),
                    Message::RemoveStart,
                    Message::DetailStart,
                    Message::ReorderStart,
                    Message::ApplyReorder,
                )
            ]
            .spacing(8.0),
            column![
                text(fl!("center-segment")),
                AppletReorderList::new(
                    page.available_entries
                        .iter()
                        .filter_map(|a| {
                            config.plugins_center.as_ref().and_then(|list| {
                                list.iter()
                                    .any(|saved| (saved.as_str() == a.id.as_ref()))
                                    .then(|| a.borrowed())
                            })
                        })
                        .collect(),
                    Some((window::Id(0), APPLET_DND_ICON_ID)),
                    Message::StartDnd,
                    |a| Message::DnDCommand(Arc::new(a)),
                    Message::RemoveCenter,
                    Message::DetailCenter,
                    Message::ReorderCenter,
                    Message::ApplyReorder,
                )
            ]
            .spacing(8.0),
            column![
                text(fl!("end-segment")),
                AppletReorderList::new(
                    page.available_entries
                        .iter()
                        .filter_map(|a| {
                            config.plugins_wings.as_ref().and_then(|list| {
                                list.1
                                    .iter()
                                    .any(|saved| (saved.as_str() == a.id.as_ref()))
                                    .then(|| a.borrowed())
                            })
                        })
                        .collect(),
                    Some((window::Id(0), APPLET_DND_ICON_ID)),
                    Message::StartDnd,
                    |a| Message::DnDCommand(Arc::new(a)),
                    Message::RemoveEnd,
                    Message::DetailEnd,
                    Message::ReorderEnd,
                    Message::ApplyReorder,
                )
            ]
            .spacing(8.0),
        ]
        .spacing(12.0)
        .apply(Element::from)
        .map(pages::Message::Applet)
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

static MIME_TYPE: &str = "text/uri-list";

pub type OnDndCommand<'a, Message> = Box<
    dyn Fn(
            Box<dyn Send + Sync + Fn() -> platform_specific::wayland::data_device::ActionInner>,
        ) -> Message
        + 'a,
>;

const SPACING: f32 = 8.0;

const DRAG_START_DISTANCE_SQUARED: f32 = 64.0;

pub const APPLET_DND_ICON_ID: window::Id = window::Id(1000);
// TODO A11y

pub struct AppletReorderList<'a, Message> {
    id: Id,
    info: Vec<Applet<'a>>,
    on_create_dnd_source: Box<dyn Fn(State) -> Message + 'a>,
    on_dnd_command_produced: OnDndCommand<'a, Message>,
    on_reorder: Box<dyn Fn(Vec<Applet<'static>>) -> Message + 'a>,
    on_finish: Option<Message>,
    surface_ids: Option<(window::Id, window::Id)>,
    dnd_icon: bool,
    inner: Element<'a, Message>,
}

impl<'a, Message: 'static + Clone> AppletReorderList<'a, Message> {
    /// new applet list which can be reordered and dragged
    pub fn new(
        info: Vec<Applet<'a>>,
        surface_ids: Option<(window::Id, window::Id)>,
        on_create_dnd_source: impl Fn(State) -> Message + 'a,
        on_dnd_command_produced: impl Fn(
                Box<dyn Send + Sync + Fn() -> platform_specific::wayland::data_device::ActionInner>,
            ) -> Message
            + 'a,
        on_remove: impl Fn(String) -> Message + 'a,
        on_details: impl Fn(String) -> Message + 'a,
        on_reorder: impl Fn(Vec<Applet<'static>>) -> Message + 'a,
        on_apply_reorder: Message,
        // state: Option<&State>,
    ) -> Self {
        // let dragged_path = state.and_then(|state| state.dragged_applet());
        let applet_buttons = info
            .clone()
            .into_iter()
            .map(|info| {
                let id_clone = info.id.to_string();
                container(
                    row![
                        icon("open-menu-symbolic", 16).style(theme::Svg::Symbolic),
                        icon(info.icon, 32).style(theme::Svg::Symbolic),
                        column![text(info.name), text(info.description).size(10)]
                            .spacing(4.0)
                            .width(Length::Fill),
                        button(theme::Button::Text)
                            .icon(theme::Svg::Symbolic, "edit-delete-symbolic", 16)
                            .on_press(on_remove(id_clone.clone())),
                        button(theme::Button::Text)
                            .icon(theme::Svg::Symbolic, "open-menu-symbolic", 16)
                            .on_press(on_details(id_clone)),
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .padding(8)
                .style(theme::Container::Custom(Box::new(move |theme| {
                    let mut style = theme.appearance(&theme::Container::Primary);
                    style.border_radius = 8.0.into();
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
            surface_ids,
            dnd_icon: false,
            inner: Column::with_children(applet_buttons)
                .spacing(SPACING)
                .into(),
        }
    }

    /// mark this as a dnd icon
    pub fn dnd_icon(state: State) -> Self {
        Self {
            id: Id::unique(),
            info: Vec::new(),
            on_create_dnd_source: Box::new(|_| unimplemented!()),
            on_dnd_command_produced: Box::new(|_| unimplemented!()),
            on_reorder: Box::new(|_| unimplemented!()),
            on_finish: None,
            surface_ids: None,
            dnd_icon: true,
            inner: if let Some(info) = state.dragged_applet() {
                container(
                    row![
                        icon("open-menu-symbolic", 16).style(theme::Svg::Symbolic),
                        icon(info.icon.to_owned(), 32).style(theme::Svg::Symbolic),
                        column![text(info.name), text(info.description).size(10)]
                            .spacing(4.0)
                            .width(Length::Fill),
                        button(theme::Button::Text).icon(
                            theme::Svg::Symbolic,
                            "edit-delete-symbolic",
                            16
                        ),
                        button(theme::Button::Text).icon(
                            theme::Svg::Symbolic,
                            "open-menu-symbolic",
                            16
                        ),
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
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
        }
    }

    /// reorders the list of applets given:
    /// - the bounds of the list
    /// - the current mouse position during a drag
    /// - the applet being dragged
    fn get_reordered(
        &self,
        layout: &layout::Layout,
        pos: Point,
        path: Applet<'a>,
    ) -> Vec<Applet<'a>> {
        let mut reordered: Vec<_> = self.info.clone();
        let mut to_remove = reordered.iter().position(|p| p == &path);

        if !layout.bounds().contains(pos) {
            if let Some(pos) = reordered.iter().position(|p| p == &path) {
                reordered.remove(pos);
            }
            return reordered;
        }

        let height = (layout.bounds().height - SPACING * (self.info.len() - 1) as f32)
            / self.info.len() as f32;

        let rectangles = (0..reordered.len())
            .map(|i| {
                if i == 0 {
                    let y = layout.bounds().y;
                    Rectangle::new(
                        Point::new(layout.bounds().x, y),
                        Size::new(layout.bounds().width, height / 2.0),
                    )
                } else if i == reordered.len() - 1 {
                    let i_offset = i - 1;
                    let y = layout.bounds().y
                        + height / 2.0
                        + height * i_offset as f32
                        + SPACING * i_offset as f32;
                    Rectangle::new(
                        Point::new(layout.bounds().x, y),
                        Size::new(layout.bounds().width, height / 2.0),
                    )
                } else {
                    let i_offset = i - 1;
                    let y = layout.bounds().y
                        + height / 2.0
                        + height * i_offset as f32
                        + SPACING * i_offset as f32;
                    Rectangle::new(
                        Point::new(layout.bounds().x, y),
                        Size::new(layout.bounds().width, height),
                    )
                }
            })
            .collect::<Vec<_>>();
        if let Some(cursor_index) = rectangles
            .iter()
            .enumerate()
            .find(|(_, r)| r.contains(pos))
            .map(|(i, _)| i)
        {
            reordered.insert(cursor_index, path);
            if let Some(to_remove) = to_remove.as_mut() {
                if *to_remove < cursor_index {
                    *to_remove += 1;
                } else if *to_remove > cursor_index {
                    *to_remove -= 1;
                }
            }
        }

        if let Some(to_remove) = to_remove {
            reordered.remove(to_remove);
        }

        reordered
    }
}

impl<'a, Message: 'static> Widget<Message, cosmic::Renderer> for AppletReorderList<'a, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
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

    #[allow(clippy::too_many_lines)]
    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        renderer: &cosmic::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let mut ret = match self.inner.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        ) {
            event::Status::Captured => return event::Status::Captured,
            event::Status::Ignored => event::Status::Ignored,
        };
        // todo do some arithmetic to find out which applet from the list is being dragged
        let height = (layout.bounds().height - SPACING * (self.info.len() - 1) as f32)
            / self.info.len() as f32;
        let state = tree.state.downcast_mut::<State>();

        state.dragging_state = match mem::take(&mut state.dragging_state) {
            DraggingState::None => {
                // if no dragging state, listen for press events
                match &event {
                    event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | event::Event::Touch(touch::Event::FingerPressed { .. }) => {
                        ret = event::Status::Captured;

                        DraggingState::Pressed(cursor_position)
                    }
                    _ => DraggingState::None,
                }
            }
            DraggingState::Pressed(start) => {
                // if dragging state is pressed, listen for motion events or release events
                match &event {
                    event::Event::Mouse(mouse::Event::CursorMoved { .. })
                    | event::Event::Touch(touch::Event::FingerMoved { .. }) => {
                        let d_y = cursor_position.y - start.y;
                        let d_x = cursor_position.x - start.x;
                        let distance_squared = d_y * d_y + d_x * d_x;
                        if distance_squared > DRAG_START_DISTANCE_SQUARED {
                            if let Some((_, applet)) =
                                self.info.iter().enumerate().find(|(i, _)| {
                                    let bounds = Rectangle::new(
                                        Point::new(
                                            layout.bounds().x,
                                            layout.bounds().y + *i as f32 * height,
                                        ),
                                        cosmic::iced::Size::new(layout.bounds().width, height),
                                    );
                                    bounds.contains(cursor_position)
                                })
                            {
                                let (window_id, icon_id) = self.surface_ids.unwrap();
                                let mut state_clone = state.clone();
                                state_clone.dragging_state =
                                    DraggingState::Dragging(applet.clone().into_owned());

                                // TODO emit a dnd command

                                shell.publish((self.on_create_dnd_source.as_ref())(state_clone));
                                let reordered = self
                                    .info
                                    .clone()
                                    .into_iter()
                                    .filter(|a| a != applet)
                                    .map(Applet::into_owned)
                                    .collect::<Vec<_>>();
                                shell.publish((self.on_reorder.as_ref())(reordered));

                                let state = state.clone();
                                let p = applet.path.to_path_buf();
                                let state_clone = state.clone();
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
            // if dragging listen for drag events
            DraggingState::Dragging(applet) => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DataSource(wayland::DataSourceEvent::DndFinished),
                ))
                | event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DataSource(wayland::DataSourceEvent::Cancelled),
                )) => {
                    ret = event::Status::Captured;
                    if let Some(on_finish) = self.on_finish.clone() {
                        shell.publish(on_finish);
                    }
                    DraggingState::None
                }
                // event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                //     wayland::Event::DataSource(wayland::DataSourceEvent::DndActionAccepted(_)),
                // )) => {
                //     // TODO support just copying
                //     todo!()
                // }
                _ => DraggingState::Dragging(applet),
            },
        };
        state.dnd_offer = match mem::take(&mut state.dnd_offer) {
            DndOfferState::None => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Enter { x, y, mime_types }),
                )) => {
                    if mime_types.iter().any(|m| m.as_str() == MIME_TYPE) {
                        let point = Point::new(*x as f32, *y as f32);
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
                                    preferred: DndAction::Move,
                                    accepted: DndAction::Move.union(DndAction::Copy),
                                }
                            },
                        )));
                        shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                            move || {
                                platform_specific::wayland::data_device::ActionInner::RequestDndData(MIME_TYPE.to_string())
                            },
                        )));
                        if layout.bounds().contains(point) {
                            // let reordered_list = self.get_reordered(layout, cursor_position, path)
                            DndOfferState::HandlingOffer(
                                mime_types.clone(),
                                DndAction::empty(),
                                None,
                            )
                        } else {
                            DndOfferState::OutsideWidget(
                                mime_types.clone(),
                                DndAction::empty(),
                                None,
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
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Motion { x, y }),
                )) => {
                    let point = Point::new(*x as f32, *y as f32);
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || {
                            platform_specific::wayland::data_device::ActionInner::Accept(Some(
                                MIME_TYPE.to_string(),
                            ))
                        },
                    )));
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::SetActions {
                            preferred: DndAction::Move,
                            accepted: DndAction::Move.union(DndAction::Copy),
                        },
                    )));
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || {
                            platform_specific::wayland::data_device::ActionInner::RequestDndData(
                                MIME_TYPE.to_string(),
                            )
                        },
                    )));
                    if layout.bounds().contains(point) {
                        if let Some(applet) = data.clone() {
                            let reordered_list: Vec<_> =
                                self.get_reordered(&layout, cursor_position, applet);
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
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Leave),
                )) => DndOfferState::None,
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::SourceActions(actions)),
                )) => {
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::SetActions {
                            preferred: DndAction::Move,
                            accepted: DndAction::Move.union(DndAction::Copy),
                        },
                    )));
                    DndOfferState::OutsideWidget(mime_types, *actions, data)
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DndData {
                        data: new_data,
                        mime_type,
                    }),
                )) => {
                    if mime_type.as_str() == MIME_TYPE {
                        DndOfferState::OutsideWidget(
                            mime_types.clone(),
                            action,
                            std::str::from_utf8(new_data.as_bytes())
                                .ok()
                                .and_then(|s| url::Url::from_str(s).ok())
                                .and_then(|url| url.to_file_path().ok())
                                .and_then(|p| Applet::try_from(Cow::from(p)).ok()),
                        )
                    } else {
                        DndOfferState::OutsideWidget(mime_types, action, data)
                    }
                }
                _ => DndOfferState::OutsideWidget(mime_types, action, data),
            },
            DndOfferState::HandlingOffer(mime_types, action, data) => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::Motion { x, y }),
                )) => {
                    let point = Point::new(*x as f32, *y as f32);
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || {
                            platform_specific::wayland::data_device::ActionInner::Accept(Some(
                                MIME_TYPE.to_string(),
                            ))
                        },
                    )));
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::SetActions {
                            preferred: DndAction::Move,
                            accepted: DndAction::Move.union(DndAction::Copy),
                        },
                    )));
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || {
                            platform_specific::wayland::data_device::ActionInner::RequestDndData(
                                MIME_TYPE.to_string(),
                            )
                        },
                    )));
                    if layout.bounds().contains(point) {
                        if let Some(data) = data.clone() {
                            let reordered_list = self.get_reordered(&layout, cursor_position, data);
                            if reordered_list != self.info {
                                shell.publish((self.on_reorder.as_ref())(
                                    reordered_list.into_iter().map(|p| p.into_owned()).collect(),
                                ));
                            }
                        }
                        DndOfferState::HandlingOffer(mime_types, DndAction::empty(), data)
                    } else {
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
                            accepted: DndAction::Move.union(DndAction::Copy),
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
                        DndOfferState::HandlingOffer(
                            mime_types.clone(),
                            action,
                            std::str::from_utf8(new_data.as_bytes())
                                .ok()
                                .and_then(|s| url::Url::from_str(s).ok())
                                .and_then(|url| url.to_file_path().ok())
                                .and_then(|p| Applet::try_from(Cow::from(p)).ok()),
                        )
                    } else {
                        DndOfferState::HandlingOffer(mime_types, action, data)
                    }
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DropPerformed),
                )) => DndOfferState::Dropped,
                _ => DndOfferState::HandlingOffer(mime_types, action, data),
            },
            DndOfferState::Dropped => match &event {
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DataSource(wayland::DataSourceEvent::DndFinished)
                    | wayland::Event::DataSource(wayland::DataSourceEvent::Cancelled),
                )) => {
                    if let Some(on_finish) = self.on_finish.clone() {
                        shell.publish(on_finish);
                    }
                    ret = event::Status::Captured;
                    DndOfferState::None
                }
                event::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::DndOffer(wayland::DndOfferEvent::DndData { .. }),
                )) => {
                    // already applied the data, so we can just finish
                    shell.publish((self.on_dnd_command_produced.as_ref())(Box::new(
                        move || platform_specific::wayland::data_device::ActionInner::DndFinished,
                    )));
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
        cursor_position: Point,
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
        cursor_position: Point,
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
                let state = state.state.downcast_ref::<State>();
                if matches!(state.dragging_state, DraggingState::Dragging(_)) {
                    mouse::Interaction::Grabbing
                } else if layout.bounds().contains(cursor_position) {
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
            Some(
                url::Url::from_file_path(self.0.clone())
                    .ok()?
                    .to_string()
                    .as_bytes()
                    .to_vec(),
            )
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

/// The state of a [`TextInput`].
#[derive(Debug, Default, Clone)]
pub struct State {
    dragging_state: DraggingState,
    dnd_offer: DndOfferState,
}

impl State {
    pub(crate) fn new() -> State {
        State::default()
    }

    pub(crate) fn dragged_applet(self) -> Option<Applet<'static>> {
        match self.dragging_state {
            DraggingState::Dragging(applet) => Some(applet),
            _ => None,
        }
    }
}

impl<'a, Message: 'static + Clone> From<AppletReorderList<'a, Message>> for Element<'a, Message> {
    fn from(applet_reorder_list: AppletReorderList<'a, Message>) -> Self {
        Element::new(applet_reorder_list)
    }
}
