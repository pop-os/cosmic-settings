// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::cell::RefCell;

use cosmic::iced::widget::canvas::{
    Canvas, Frame, Geometry, Path, Program, Stroke, Text,
    event::{self as canvas_event, Status},
};
use cosmic::iced::{Color, Point, Rectangle, Size, Vector, mouse};
use cosmic::iced_widget::core::Length;
use cosmic::widget::segmented_button::SingleSelectModel;
use cosmic::{Element, Renderer, Theme};
use cosmic_randr_shell as randr;

use super::{OutputKey, Page};

const CAMERA_FIT_PADDING: f32 = 1.2;
const HORIZONTAL_BIAS: f32 = 1.25;
const MIN_OVERLAP_PIXELS: f32 = 50.0;
const DISPLAY_CORNER_RADIUS: f32 = 4.0;
const DISPLAY_BORDER_WIDTH: f32 = 3.0;
const BADGE_WIDTH: f32 = 72.0;
const BADGE_HEIGHT: f32 = 46.0;
const BADGE_CORNER_RADIUS: f32 = 30.0;
const BADGE_FONT_SIZE: f32 = 24.0;
const SCALE_THRESHOLD: f32 = 0.05;
const CENTER_THRESHOLD: f32 = 50.0;
pub(super) const EDGE_TOLERANCE: f32 = 1.0;


#[derive(Debug, Clone)]
pub struct Camera2D {
    pub position: Point,
    pub scale: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            scale: 1.0,
        }
    }
}

impl Camera2D {
    fn screen_to_world(&self, screen_pos: Point, viewport_size: Size) -> Point {
        Point {
            x: (screen_pos.x - viewport_size.width / 2.0) / self.scale + self.position.x,
            y: (screen_pos.y - viewport_size.height / 2.0) / self.scale + self.position.y,
        }
    }

    fn world_to_screen(&self, world_pos: Point, viewport_size: Size) -> Point {
        Point {
            x: (world_pos.x - self.position.x) * self.scale + viewport_size.width / 2.0,
            y: (world_pos.y - self.position.y) * self.scale + viewport_size.height / 2.0,
        }
    }

    fn pan(&mut self, screen_delta: Vector) {
        self.position.x -= screen_delta.x / self.scale;
        self.position.y -= screen_delta.y / self.scale;
    }
}

#[derive(Debug, Clone)]
pub struct DisplayRect {
    pub output_key: OutputKey,
    pub position: Point,
    pub size: Size,
    pub name: String,
}

impl DisplayRect {
    fn bounds(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.size.width,
            height: self.size.height,
        }
    }

    fn contains(&self, point: Point) -> bool {
        self.bounds().contains(point)
    }

    fn edge_center(&self, edge: Edge) -> Point {
        let bounds = self.bounds();
        match edge {
            Edge::Left => Point::new(bounds.x, bounds.y + bounds.height / 2.0),
            Edge::Right => Point::new(bounds.x + bounds.width, bounds.y + bounds.height / 2.0),
            Edge::Top => Point::new(bounds.x + bounds.width / 2.0, bounds.y),
            Edge::Bottom => Point::new(bounds.x + bounds.width / 2.0, bounds.y + bounds.height),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Edge {
    const ALL: [Edge; 4] = [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom];

    fn distance_bias(&self) -> f32 {
        match self {
            Edge::Left | Edge::Right => HORIZONTAL_BIAS,
            Edge::Top | Edge::Bottom => 1.0,
        }
    }

    fn snap_to(&self, target_bounds: Rectangle, dragged_pos: Point, dragged_size: Size) -> Point {
        match self {
            Edge::Left => Point::new(target_bounds.x - dragged_size.width, dragged_pos.y),
            Edge::Right => Point::new(target_bounds.x + target_bounds.width, dragged_pos.y),
            Edge::Top => Point::new(dragged_pos.x, target_bounds.y - dragged_size.height),
            Edge::Bottom => Point::new(dragged_pos.x, target_bounds.y + target_bounds.height),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Interaction {
    None,
    Dragging {
        output_key: OutputKey,
        offset: Vector,
    },
    Panning {
        last_pos: Point,
    },
}

#[inline]
fn distance(a: Point, b: Point) -> f32 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

#[derive(Debug)]
pub struct ArrangementState {
    pub camera: RefCell<Camera2D>,
    pub interaction: Interaction,
    pub displays: RefCell<Vec<DisplayRect>>,
    pub needs_fit: RefCell<bool>,
}

impl Default for ArrangementState {
    fn default() -> Self {
        Self {
            camera: RefCell::new(Camera2D::default()),
            interaction: Interaction::None,
            displays: RefCell::new(Vec::new()),
            needs_fit: RefCell::new(true),
        }
    }
}

impl ArrangementState {
    fn find_display_at(&self, world_pos: Point) -> Option<usize> {
        self.displays
            .borrow()
            .iter()
            .position(|d| d.contains(world_pos))
    }

    fn needs_sync(&self, list: &randr::List, tab_model: &SingleSelectModel) -> bool {
        let enabled_count = tab_model
            .iter()
            .filter_map(|id| tab_model.data::<OutputKey>(id))
            .filter(|&&key| {
                list.outputs
                    .get(key)
                    .map(|output| output.enabled)
                    .unwrap_or(false)
            })
            .count();

        let displays = self.displays.borrow();

        if displays.len() != enabled_count {
            return true;
        }

        displays.iter().any(|disp| {
            list.outputs
                .get(disp.output_key)
                .map(|output| {
                    let randr_pos = Point::new(
                        output.position.0 as f32,
                        output.position.1 as f32,
                    );
                    randr_pos != disp.position
                })
                .unwrap_or(false)
        })
    }

    fn calculate_bounding_box(&self) -> Option<Rectangle> {
        let displays = self.displays.borrow();
        if displays.is_empty() {
            return None;
        }

        let (mut min_x, mut min_y) = (f32::MAX, f32::MAX);
        let (mut max_x, mut max_y) = (f32::MIN, f32::MIN);

        for disp in displays.iter() {
            min_x = min_x.min(disp.position.x);
            min_y = min_y.min(disp.position.y);
            max_x = max_x.max(disp.position.x + disp.size.width);
            max_y = max_y.max(disp.position.y + disp.size.height);
        }

        Some(Rectangle {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }

    fn calculate_edge_overlap(
        edge: Edge,
        rect_pos: Point,
        rect_size: Size,
        target: Rectangle,
    ) -> f32 {
        let (overlap_start, overlap_end) = match edge {
            Edge::Left | Edge::Right => (
                rect_pos.y.max(target.y),
                (rect_pos.y + rect_size.height).min(target.y + target.height),
            ),
            Edge::Top | Edge::Bottom => (
                rect_pos.x.max(target.x),
                (rect_pos.x + rect_size.width).min(target.x + target.width),
            ),
        };
        (overlap_end - overlap_start).max(0.0)
    }

    fn rectangles_overlap(a_pos: Point, a_size: Size, b: Rectangle) -> bool {
        a_pos.x < b.x + b.width
            && a_pos.x + a_size.width > b.x
            && a_pos.y < b.y + b.height
            && a_pos.y + a_size.height > b.y
    }

    fn is_connected_to_any(&self, dragged_key: OutputKey, position: Point, size: Size) -> bool {
        self.displays.borrow().iter().any(|other| {
            if other.output_key == dragged_key {
                return false;
            }

            Self::are_rectangles_adjacent(position, size, &other.bounds())
        })
    }

    fn are_rectangles_adjacent(pos: Point, size: Size, other: &Rectangle) -> bool {
        let right = pos.x + size.width;
        let bottom = pos.y + size.height;
        let other_right = other.x + other.width;
        let other_bottom = other.y + other.height;

        // Vertical adjacency (left-right touching)
        let left_touches_right = (right - other.x).abs() <= EDGE_TOLERANCE;
        let right_touches_left = (other_right - pos.x).abs() <= EDGE_TOLERANCE;
        let has_vertical_overlap = bottom > other.y && pos.y < other_bottom;

        if (left_touches_right || right_touches_left) && has_vertical_overlap {
            return true;
        }

        // Horizontal adjacency (top-bottom touching)
        let bottom_touches_top = (bottom - other.y).abs() <= EDGE_TOLERANCE;
        let top_touches_bottom = (other_bottom - pos.y).abs() <= EDGE_TOLERANCE;
        let has_horizontal_overlap = right > other.x && pos.x < other_right;

        (bottom_touches_top || top_touches_bottom) && has_horizontal_overlap
    }

    fn would_overlap_any(&self, dragged_key: OutputKey, position: Point, size: Size) -> bool {
        self.displays.borrow().iter().any(|other| {
            other.output_key != dragged_key && Self::rectangles_overlap(position, size, other.bounds())
        })
    }

    fn fit_camera(&self, viewport_size: Size) {
        let Some(bbox) = self.calculate_bounding_box() else {
            return;
        };

        let center = Point::new(bbox.x + bbox.width / 2.0, bbox.y + bbox.height / 2.0);

        let scale_x = viewport_size.width / (bbox.width * CAMERA_FIT_PADDING);
        let scale_y = viewport_size.height / (bbox.height * CAMERA_FIT_PADDING);
        let scale = scale_x.min(scale_y).min(1.0);

        let mut camera = self.camera.borrow_mut();
        camera.position = center;
        camera.scale = scale;

        *self.needs_fit.borrow_mut() = false;
    }

    fn find_best_snap_target(
        &self,
        dragged_key: OutputKey,
        position: Point,
        size: Size,
    ) -> Option<(Edge, Rectangle, OutputKey)> {
        let center = Point::new(
            position.x + size.width / 2.0,
            position.y + size.height / 2.0,
        );

        let mut best_distance = f32::MAX;
        let mut best_edge = None;
        let mut best_target = Rectangle::default();
        let mut best_target_key = None;

        for other in self.displays.borrow().iter() {
            if other.output_key == dragged_key {
                continue;
            }

            let other_bounds = other.bounds();

            for edge in Edge::ALL {
                let edge_center = other.edge_center(edge);
                let dist = distance(edge_center, center) * edge.distance_bias();
                let test_snap_pos = edge.snap_to(other_bounds, position, size);
                let overlap = Self::calculate_edge_overlap(edge, test_snap_pos, size, other_bounds);

                if overlap >= MIN_OVERLAP_PIXELS && dist < best_distance {
                    best_distance = dist;
                    best_edge = Some(edge);
                    best_target = other_bounds;
                    best_target_key = Some(other.output_key);
                }
            }
        }

        best_edge.map(|edge| (edge, best_target, best_target_key.unwrap()))
    }

    fn apply_snapping(
        &self,
        dragged_key: OutputKey,
        position: Point,
        size: Size,
        current_position: Point,
    ) -> Point {
        // No snapping if this is the only display
        let has_other_displays = self
            .displays
            .borrow()
            .iter()
            .any(|d| d.output_key != dragged_key);

        if !has_other_displays {
            return position;
        }

        let Some((edge, target, _target_key)) =
            self.find_best_snap_target(dragged_key, position, size)
        else {
            return current_position;
        };

        let snapped_pos = edge.snap_to(target, position, size);

        // Validate the snapped position
        let overlap = Self::calculate_edge_overlap(edge, snapped_pos, size, target);
        let is_valid = overlap >= MIN_OVERLAP_PIXELS
            && self.is_connected_to_any(dragged_key, snapped_pos, size)
            && !self.would_overlap_any(dragged_key, snapped_pos, size);

        if is_valid {
            snapped_pos
        } else {
            current_position
        }
    }

    fn sync_displays(&self, list: &randr::List, tab_model: &SingleSelectModel) {
        let mut displays = self.displays.borrow_mut();
        let old_count = displays.len();
        let old_positions: std::collections::HashMap<_, _> = displays
            .iter()
            .map(|d| (d.output_key, d.position))
            .collect();

        displays.clear();

        for id in tab_model.iter() {
            let Some(&key) = tab_model.data::<OutputKey>(id) else {
                continue;
            };

            let Some(output) = list.outputs.get(key) else {
                continue;
            };

            if !output.enabled {
                continue;
            }

            let Some(mode_key) = output.current else {
                continue;
            };

            let Some(mode) = list.modes.get(mode_key) else {
                continue;
            };

            let (width, height) = if output.transform.is_none_or(Page::is_landscape) {
                (
                    mode.size.0 as f32 / output.scale as f32,
                    mode.size.1 as f32 / output.scale as f32,
                )
            } else {
                (
                    mode.size.1 as f32 / output.scale as f32,
                    mode.size.0 as f32 / output.scale as f32,
                )
            };

            displays.push(DisplayRect {
                output_key: key,
                position: Point::new(output.position.0 as f32, output.position.1 as f32),
                size: Size::new(width, height),
                name: output.name.clone(),
            });
        }

        let positions_changed = displays.iter().any(|d| {
            old_positions
                .get(&d.output_key)
                .map(|&old_pos| old_pos != d.position)
                .unwrap_or(true)
        });

        if displays.len() != old_count || positions_changed {
            *self.needs_fit.borrow_mut() = true;
        }
    }

    fn handle_drag_move(
        &self,
        output_key: OutputKey,
        offset: Vector,
        world_pos: Point,
        bounds_size: Size,
    ) {
        let raw_pos = Point::new(world_pos.x - offset.x, world_pos.y - offset.y);

        let (display_size, current_pos) = self
            .displays
            .borrow()
            .iter()
            .find(|d| d.output_key == output_key)
            .map(|d| (d.size, d.position))
            .unwrap_or_default();

        let snapped_pos = self.apply_snapping(output_key, raw_pos, display_size, current_pos);

        if let Some(disp) = self
            .displays
            .borrow_mut()
            .iter_mut()
            .find(|d| d.output_key == output_key)
        {
            disp.position = snapped_pos;
        }

        self.check_refit_needed(bounds_size);
    }

    fn check_refit_needed(&self, bounds_size: Size) {
        let Some(bbox) = self.calculate_bounding_box() else {
            return;
        };

        let camera = self.camera.borrow();

        let required_scale_x = bounds_size.width / (bbox.width * CAMERA_FIT_PADDING);
        let required_scale_y = bounds_size.height / (bbox.height * CAMERA_FIT_PADDING);
        let required_scale = required_scale_x.min(required_scale_y).min(1.0);

        let required_center_x = bbox.x + bbox.width / 2.0;
        let required_center_y = bbox.y + bbox.height / 2.0;

        let scale_diff = (camera.scale - required_scale).abs() / camera.scale;
        let center_diff = ((camera.position.x - required_center_x).powi(2)
            + (camera.position.y - required_center_y).powi(2))
        .sqrt();

        if scale_diff > SCALE_THRESHOLD || center_diff > CENTER_THRESHOLD {
            *self.needs_fit.borrow_mut() = true;
        }
    }
}

pub struct ArrangementCanvas<'a, Message> {
    list: &'a randr::List,
    tab_model: &'a SingleSelectModel,
    on_placement: Option<Box<dyn Fn(OutputKey, i32, i32) -> Message + 'a>>,
    on_select: Option<Box<dyn Fn(OutputKey) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> ArrangementCanvas<'a, Message> {
    pub fn new(list: &'a randr::List, tab_model: &'a SingleSelectModel) -> Self {
        Self {
            list,
            tab_model,
            on_placement: None,
            on_select: None,
        }
    }

    pub fn on_placement(mut self, f: impl Fn(OutputKey, i32, i32) -> Message + 'a) -> Self {
        self.on_placement = Some(Box::new(f));
        self
    }

    pub fn on_select(mut self, f: impl Fn(OutputKey) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    pub fn view(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fixed(400.0))
            .into()
    }
}

impl<'a, Message: Clone> Program<Message, Theme, Renderer> for ArrangementCanvas<'a, Message> {
    type State = ArrangementState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        if !matches!(state.interaction, Interaction::Dragging { .. })
            && state.needs_sync(self.list, self.tab_model)
        {
            state.sync_displays(self.list, self.tab_model);
        }

        let selected_output = self
            .tab_model
            .data::<OutputKey>(self.tab_model.active())
            .copied();

        if !matches!(state.interaction, Interaction::Dragging { .. })
            && !state.displays.borrow().is_empty()
        {
            state.check_refit_needed(bounds.size());
        }

        if *state.needs_fit.borrow() && !state.displays.borrow().is_empty() {
            state.fit_camera(bounds.size());
        }

        let mut frame = Frame::new(renderer, bounds.size());
        let cosmic_theme = theme.cosmic();

        frame.fill(
            &Path::rectangle(Point::ORIGIN, bounds.size()),
            Color::from(cosmic_theme.background.component.base),
        );

        let camera = state.camera.borrow();
        let border_color = cosmic_theme.palette.neutral_7;

        for (index, disp) in state.displays.borrow().iter().enumerate() {
            let world_rect = disp.bounds();
            let screen_pos = camera.world_to_screen(world_rect.position(), bounds.size());
            let screen_size = Size::new(
                world_rect.width * camera.scale,
                world_rect.height * camera.scale,
            );

            if screen_pos.x + screen_size.width < 0.0
                || screen_pos.y + screen_size.height < 0.0
                || screen_pos.x > bounds.width
                || screen_pos.y > bounds.height
            {
                continue;
            }

            let is_selected = selected_output == Some(disp.output_key);

            let bg_color = if is_selected {
                cosmic_theme.accent_color()
            } else {
                cosmic_theme.palette.neutral_4
            };

            frame.fill(
                &Path::rounded_rectangle(screen_pos, screen_size, DISPLAY_CORNER_RADIUS.into()),
                Color::from(bg_color),
            );

            frame.stroke(
                &Path::rounded_rectangle(screen_pos, screen_size, DISPLAY_CORNER_RADIUS.into()),
                Stroke::default()
                    .with_width(DISPLAY_BORDER_WIDTH)
                    .with_color(Color::from(border_color)),
            );

            let badge_pos = Point::new(
                screen_pos.x + (screen_size.width - BADGE_WIDTH) / 2.0,
                screen_pos.y + (screen_size.height - BADGE_HEIGHT) / 2.0,
            );
            let badge_size = Size::new(BADGE_WIDTH, BADGE_HEIGHT);

            frame.fill(
                &Path::rounded_rectangle(badge_pos, badge_size, BADGE_CORNER_RADIUS.into()),
                Color::from(cosmic_theme.palette.neutral_1),
            );

            frame.fill_text(Text {
                content: (index + 1).to_string(),
                position: Point::new(
                    badge_pos.x + BADGE_WIDTH / 2.0,
                    badge_pos.y + BADGE_HEIGHT / 2.0,
                ),
                color: Color::from(cosmic_theme.palette.neutral_10),
                size: BADGE_FONT_SIZE.into(),
                font: cosmic::font::bold(),
                horizontal_alignment: cosmic::iced::alignment::Horizontal::Center,
                vertical_alignment: cosmic::iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas_event::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (Status, Option<Message>) {
        let has_active_interaction = !matches!(state.interaction, Interaction::None);

        let cursor_pos = if has_active_interaction {
            cursor
                .position()
                .map(|pos| Point::new(pos.x - bounds.x, pos.y - bounds.y))
        } else {
            cursor.position_in(bounds)
        };

        let Some(cursor_pos) = cursor_pos else {
            return (Status::Ignored, None);
        };

        match event {
            canvas_event::Event::Mouse(mouse::Event::ButtonPressed(button)) => match button {
                mouse::Button::Left => {
                    let world_pos = state
                        .camera
                        .borrow()
                        .screen_to_world(cursor_pos, bounds.size());
                    if let Some(idx) = state.find_display_at(world_pos) {
                        let displays = state.displays.borrow();
                        let display = &displays[idx];
                        let display_key = display.output_key;
                        let display_position = display.position;
                        drop(displays);

                        state.interaction = Interaction::Dragging {
                            output_key: display_key,
                            offset: Vector::new(
                                world_pos.x - display_position.x,
                                world_pos.y - display_position.y,
                            ),
                        };

                        if let Some(ref on_select) = self.on_select {
                            return (Status::Captured, Some(on_select(display_key)));
                        }
                        return (Status::Captured, None);
                    }
                }
                mouse::Button::Middle | mouse::Button::Right => {
                    state.interaction = Interaction::Panning {
                        last_pos: cursor_pos,
                    };
                    return (Status::Captured, None);
                }
                _ => {}
            },

            // Handle movement
            canvas_event::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                match &state.interaction {
                    Interaction::Dragging { output_key, offset } => {
                        let world_pos = state
                            .camera
                            .borrow()
                            .screen_to_world(cursor_pos, bounds.size());
                        state.handle_drag_move(*output_key, *offset, world_pos, bounds.size());
                        return (Status::Captured, None);
                    }
                    Interaction::Panning { last_pos } => {
                        let delta =
                            Vector::new(cursor_pos.x - last_pos.x, cursor_pos.y - last_pos.y);
                        state.camera.borrow_mut().pan(delta);
                        state.interaction = Interaction::Panning {
                            last_pos: cursor_pos,
                        };
                        return (Status::Captured, None);
                    }
                    Interaction::None => {}
                }
            }

            canvas_event::Event::Mouse(mouse::Event::ButtonReleased(button)) => {
                if matches!(
                    button,
                    mouse::Button::Left | mouse::Button::Middle | mouse::Button::Right
                ) {
                    if let Interaction::Dragging { output_key, .. } = state.interaction {
                        state.interaction = Interaction::None;

                        let displays = state.displays.borrow();
                        if let Some(display) = displays.iter().find(|d| d.output_key == output_key)
                        {
                            let new_pos = (
                                display.position.x as i32,
                                display.position.y as i32,
                            );

                            let old_pos = self
                                .list
                                .outputs
                                .get(output_key)
                                .map(|output| output.position)
                                .unwrap_or((0, 0));

                            if old_pos != new_pos {
                                *state.needs_fit.borrow_mut() = true;

                                if let Some(ref on_placement) = self.on_placement {
                                    return (
                                        Status::Captured,
                                        Some(on_placement(output_key, new_pos.0, new_pos.1)),
                                    );
                                }
                            }
                        }
                        return (Status::Captured, None);
                    }

                    state.interaction = Interaction::None;
                    return (Status::Captured, None);
                }
            }

            _ => {}
        }

        (Status::Ignored, None)
    }
}

pub type Arrangement<'a, Message> = ArrangementCanvas<'a, Message>;
