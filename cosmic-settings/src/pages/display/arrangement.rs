// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::cell::RefCell;

use cosmic::iced::widget::canvas::{Action, Canvas, Frame, Geometry, Path, Program, Stroke, Text};
use cosmic::iced::{
    Color, Length, Point, Rectangle, Size, Vector, event as canvas_event, mouse, touch, window,
};
use cosmic::widget::segmented_button::SingleSelectModel;
use cosmic::{Element, Renderer, Theme};
use cosmic_randr_shell as randr;

use super::OutputKey;

const CAMERA_FIT_PADDING: f32 = 1.2;
const HORIZONTAL_BIAS: f32 = 1.25;
const MIN_OVERLAP_PIXELS: f32 = 50.0;
const SNAP_HYSTERESIS: f32 = 8.0;
const DISPLAY_CORNER_RADIUS: f32 = 4.0;
const DISPLAY_BORDER_WIDTH: f32 = 3.0;
const BADGE_WIDTH: f32 = 72.0;
const BADGE_HEIGHT: f32 = 46.0;
const BADGE_CORNER_RADIUS: f32 = 30.0;
const BADGE_FONT_SIZE: f32 = 24.0;

#[derive(Debug, Clone)]
struct Camera2D {
    position: Point,
    scale: f32,
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

#[derive(Debug, Clone, PartialEq)]
struct DisplayRect {
    output_key: OutputKey,
    entity: cosmic::widget::segmented_button::Entity,
    position: Point,
    size: Size,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SnapTarget {
    output_key: OutputKey,
    edge: Edge,
}

#[derive(Debug, Clone)]
enum Interaction {
    None,
    Dragging {
        output_key: OutputKey,
        offset: Vector,
        snap_target: Option<SnapTarget>,
        finger: Option<touch::Finger>,
    },
    Panning {
        last_pos: Point,
    },
}

#[derive(Debug)]
pub struct ArrangementState {
    camera: RefCell<Camera2D>,
    interaction: Interaction,
    displays: RefCell<Vec<DisplayRect>>,
    needs_fit: RefCell<bool>,
    viewport_size: RefCell<Size>,
}

impl Default for ArrangementState {
    fn default() -> Self {
        Self {
            camera: RefCell::new(Camera2D::default()),
            interaction: Interaction::None,
            displays: RefCell::new(Vec::new()),
            needs_fit: RefCell::new(true),
            viewport_size: RefCell::new(Size::ZERO),
        }
    }
}

impl ArrangementState {
    fn display_rect(
        list: &randr::List,
        tab_model: &SingleSelectModel,
        entity: cosmic::widget::segmented_button::Entity,
    ) -> Option<DisplayRect> {
        let key = *tab_model.data::<OutputKey>(entity)?;
        let output = list.outputs.get(key)?;
        let mode = list.modes.get(output.current?)?;

        if !output.enabled {
            return None;
        }

        let is_landscape = output.transform.is_none_or(|transform| {
            matches!(
                transform,
                randr::Transform::Normal
                    | randr::Transform::Rotate180
                    | randr::Transform::Flipped
                    | randr::Transform::Flipped180
            )
        });
        let (width, height) = if is_landscape {
            (mode.size.0, mode.size.1)
        } else {
            (mode.size.1, mode.size.0)
        };

        Some(DisplayRect {
            output_key: key,
            entity,
            position: Point::new(output.position.0 as f32, output.position.1 as f32),
            size: Size::new(
                width as f32 / output.scale as f32,
                height as f32 / output.scale as f32,
            ),
        })
    }

    fn display_snapshot(list: &randr::List, tab_model: &SingleSelectModel) -> Vec<DisplayRect> {
        tab_model
            .iter()
            .filter_map(|entity| Self::display_rect(list, tab_model, entity))
            .collect()
    }

    fn find_display_at(&self, world_pos: Point) -> Option<usize> {
        self.displays
            .borrow()
            .iter()
            .position(|d| d.contains(world_pos))
    }

    fn needs_sync(&self, list: &randr::List, tab_model: &SingleSelectModel) -> bool {
        let displays = self.displays.borrow();
        let mut expected = tab_model
            .iter()
            .filter_map(|entity| Self::display_rect(list, tab_model, entity));

        !displays
            .iter()
            .all(|display| expected.next().as_ref() == Some(display))
            || expected.next().is_some()
    }

    fn calculate_bounding_box(&self) -> Option<Rectangle> {
        self.displays
            .borrow()
            .iter()
            .map(DisplayRect::bounds)
            .reduce(|bounds, display| bounds.union(&display))
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
        Rectangle::new(a_pos, a_size).intersects(&b)
    }

    fn overlap(a_start: f32, a_length: f32, b_start: f32, b_length: f32) -> f32 {
        (a_start + a_length).min(b_start + b_length) - a_start.max(b_start)
    }

    fn are_adjacent(a: &DisplayRect, b: &DisplayRect) -> bool {
        let horizontal_contact = (a.position.x + a.size.width - b.position.x).abs() <= 1.0
            || (b.position.x + b.size.width - a.position.x).abs() <= 1.0;
        let vertical_contact = (a.position.y + a.size.height - b.position.y).abs() <= 1.0
            || (b.position.y + b.size.height - a.position.y).abs() <= 1.0;

        (horizontal_contact
            && Self::overlap(a.position.y, a.size.height, b.position.y, b.size.height) > 0.0)
            || (vertical_contact
                && Self::overlap(a.position.x, a.size.width, b.position.x, b.size.width) > 0.0)
    }

    fn connected_component(displays: &[DisplayRect], start: usize) -> Vec<usize> {
        let mut component = vec![start];
        let mut cursor = 0;
        while cursor < component.len() {
            let current = component[cursor];
            for candidate in 0..displays.len() {
                if !component.contains(&candidate)
                    && Self::are_adjacent(&displays[current], &displays[candidate])
                {
                    component.push(candidate);
                }
            }
            cursor += 1;
        }
        component
    }

    fn is_connected(displays: &[DisplayRect]) -> bool {
        displays.is_empty() || Self::connected_component(displays, 0).len() == displays.len()
    }

    fn has_overlaps(displays: &[DisplayRect]) -> bool {
        displays.iter().enumerate().any(|(index, display)| {
            displays.iter().skip(index + 1).any(|other| {
                Self::rectangles_overlap(display.position, display.size, other.bounds())
            })
        })
    }

    fn unconnected_component(
        displays: &[DisplayRect],
        start: usize,
        connected: &[bool],
    ) -> Vec<usize> {
        let mut component = vec![start];
        let mut cursor = 0;
        while cursor < component.len() {
            let current = component[cursor];
            for candidate in 0..displays.len() {
                if !connected[candidate]
                    && !component.contains(&candidate)
                    && Self::are_adjacent(&displays[current], &displays[candidate])
                {
                    component.push(candidate);
                }
            }
            cursor += 1;
        }
        component
    }

    fn component_translation(
        displays: &[DisplayRect],
        moving: &[usize],
        connected: &[bool],
    ) -> Option<Vector> {
        let mut best: Option<(Vector, f32)> = None;

        for &moving_index in moving {
            let moving_display = &displays[moving_index];
            for (target_index, target) in displays.iter().enumerate() {
                if !connected[target_index] {
                    continue;
                }

                for edge in Edge::ALL {
                    let required_overlap = match edge {
                        Edge::Left | Edge::Right => MIN_OVERLAP_PIXELS
                            .min(moving_display.size.height)
                            .min(target.size.height),
                        Edge::Top | Edge::Bottom => MIN_OVERLAP_PIXELS
                            .min(moving_display.size.width)
                            .min(target.size.width),
                    };
                    let target_bounds = target.bounds();
                    let base =
                        edge.snap_to(target_bounds, moving_display.position, moving_display.size);
                    let (minimum, maximum, current) = match edge {
                        Edge::Left | Edge::Right => (
                            target_bounds.y + required_overlap - moving_display.size.height,
                            target_bounds.y + target_bounds.height - required_overlap,
                            moving_display.position.y,
                        ),
                        Edge::Top | Edge::Bottom => (
                            target_bounds.x + required_overlap - moving_display.size.width,
                            target_bounds.x + target_bounds.width - required_overlap,
                            moving_display.position.x,
                        ),
                    };
                    let mut alignments = vec![current.clamp(minimum, maximum), minimum, maximum];
                    for &component_index in moving {
                        let component_display = &displays[component_index];
                        for (obstacle_index, obstacle) in displays.iter().enumerate() {
                            if moving.contains(&obstacle_index) {
                                continue;
                            }

                            let obstacle_bounds = obstacle.bounds();
                            let before = match edge {
                                Edge::Left | Edge::Right => {
                                    obstacle_bounds.y
                                        - component_display.position.y
                                        - component_display.size.height
                                        + moving_display.position.y
                                }
                                Edge::Top | Edge::Bottom => {
                                    obstacle_bounds.x
                                        - component_display.position.x
                                        - component_display.size.width
                                        + moving_display.position.x
                                }
                            };
                            let after = match edge {
                                Edge::Left | Edge::Right => {
                                    obstacle_bounds.y + obstacle_bounds.height
                                        - component_display.position.y
                                        + moving_display.position.y
                                }
                                Edge::Top | Edge::Bottom => {
                                    obstacle_bounds.x + obstacle_bounds.width
                                        - component_display.position.x
                                        + moving_display.position.x
                                }
                            };
                            alignments.push(before.clamp(minimum, maximum));
                            alignments.push(after.clamp(minimum, maximum));
                        }
                    }

                    for alignment in alignments {
                        let mut snapped = base;
                        match edge {
                            Edge::Left | Edge::Right => snapped.y = alignment,
                            Edge::Top | Edge::Bottom => snapped.x = alignment,
                        }

                        match edge {
                            Edge::Left => snapped.x = snapped.x.floor(),
                            Edge::Right => snapped.x = snapped.x.ceil(),
                            Edge::Top => snapped.y = snapped.y.floor(),
                            Edge::Bottom => snapped.y = snapped.y.ceil(),
                        }
                        if matches!(edge, Edge::Left | Edge::Right) {
                            snapped.y = snapped.y.round();
                        } else {
                            snapped.x = snapped.x.round();
                        }

                        let translation = snapped - moving_display.position;
                        let valid = moving.iter().all(|&index| {
                            let translated = displays[index].position + translation;
                            translated.x >= 0.0
                                && translated.y >= 0.0
                                && displays.iter().enumerate().all(|(other_index, other)| {
                                    moving.contains(&other_index)
                                        || !Self::rectangles_overlap(
                                            translated,
                                            displays[index].size,
                                            other.bounds(),
                                        )
                                })
                        });
                        if !valid {
                            continue;
                        }

                        let score = translation
                            .x
                            .mul_add(translation.x, translation.y * translation.y);
                        if best.is_none_or(|(_, best_score)| score < best_score) {
                            best = Some((translation, score));
                        }
                    }
                }
            }
        }

        best.map(|(translation, _)| translation)
    }

    fn close_gaps(&mut self, anchor: OutputKey) -> bool {
        let mut displays = self.displays.borrow_mut();
        let Some(anchor_index) = displays
            .iter()
            .position(|display| display.output_key == anchor)
        else {
            return false;
        };

        let mut connected = vec![false; displays.len()];
        for index in Self::connected_component(&displays, anchor_index) {
            connected[index] = true;
        }

        while connected.iter().any(|is_connected| !is_connected) {
            let start = connected
                .iter()
                .position(|is_connected| !is_connected)
                .expect("a disconnected display exists");
            let component = Self::connected_component(&displays, start);
            let Some(translation) = Self::component_translation(&displays, &component, &connected)
            else {
                return false;
            };

            for &index in &component {
                displays[index].position += translation;
                connected[index] = true;
            }
        }

        debug_assert!(Self::is_connected(&displays));
        true
    }

    fn reflow_around(&mut self, anchor: OutputKey) -> bool {
        let mut displays = self.displays.borrow_mut();
        let Some(anchor_index) = displays
            .iter()
            .position(|display| display.output_key == anchor)
        else {
            return false;
        };

        let mut connected = vec![false; displays.len()];
        connected[anchor_index] = true;

        while connected.iter().any(|is_connected| !is_connected) {
            let mut visited = connected.clone();
            let mut absorbed = false;
            for start in 0..displays.len() {
                if visited[start] {
                    continue;
                }
                let component = Self::unconnected_component(&displays, start, &connected);
                for &index in &component {
                    visited[index] = true;
                }

                let touches_connected = component.iter().any(|&index| {
                    displays.iter().enumerate().any(|(other_index, other)| {
                        connected[other_index] && Self::are_adjacent(&displays[index], other)
                    })
                });
                let overlaps_connected = component.iter().any(|&index| {
                    displays.iter().enumerate().any(|(other_index, other)| {
                        connected[other_index]
                            && Self::rectangles_overlap(
                                displays[index].position,
                                displays[index].size,
                                other.bounds(),
                            )
                    })
                });
                if touches_connected && !overlaps_connected {
                    for &index in &component {
                        connected[index] = true;
                    }
                    absorbed = true;
                }
            }
            if absorbed {
                continue;
            }

            let start = connected
                .iter()
                .position(|is_connected| !is_connected)
                .expect("a disconnected display exists");
            let component = Self::unconnected_component(&displays, start, &connected);
            let Some(translation) = Self::component_translation(&displays, &component, &connected)
            else {
                return false;
            };
            for &index in &component {
                displays[index].position += translation;
                connected[index] = true;
            }
        }

        debug_assert!(Self::is_connected(&displays));
        debug_assert!(!Self::has_overlaps(&displays));
        true
    }

    fn would_overlap_any(&self, dragged_key: OutputKey, position: Point, size: Size) -> bool {
        self.displays.borrow().iter().any(|other| {
            other.output_key != dragged_key
                && Self::rectangles_overlap(position, size, other.bounds())
        })
    }

    fn fit_camera(&self, viewport_size: Size) {
        let Some(bbox) = self.calculate_bounding_box() else {
            return;
        };

        let center = bbox.center();

        let scale_x = viewport_size.width / (bbox.width * CAMERA_FIT_PADDING);
        let scale_y = viewport_size.height / (bbox.height * CAMERA_FIT_PADDING);
        let scale = scale_x.min(scale_y).min(1.0);

        let mut camera = self.camera.borrow_mut();
        camera.position = center;
        camera.scale = scale;

        *self.viewport_size.borrow_mut() = viewport_size;
        *self.needs_fit.borrow_mut() = false;
    }

    fn find_best_snap_target(
        &self,
        dragged_key: OutputKey,
        position: Point,
        size: Size,
        current_position: Point,
        previous: Option<SnapTarget>,
    ) -> Option<(SnapTarget, Rectangle, f32)> {
        let center = Point::new(
            position.x + size.width / 2.0,
            position.y + size.height / 2.0,
        );
        let current_distance = position.distance_slow(current_position);
        let displays = self.displays.borrow();
        let mut best: Option<(SnapTarget, Rectangle, f32)> = None;

        let candidate = |other: &DisplayRect, edge: Edge| {
            let target = other.bounds();
            let snapped_pos = edge.snap_to(target, position, size);
            let overlap = Self::calculate_edge_overlap(edge, snapped_pos, size, target);
            let valid = overlap >= MIN_OVERLAP_PIXELS
                && position.distance_slow(snapped_pos) <= current_distance
                && !self.would_overlap_any(dragged_key, snapped_pos, size);

            valid.then(|| {
                let score = center.distance_slow(other.edge_center(edge)) * edge.distance_bias();
                (
                    SnapTarget {
                        output_key: other.output_key,
                        edge,
                    },
                    target,
                    score,
                )
            })
        };

        for other in displays
            .iter()
            .filter(|display| display.output_key != dragged_key)
        {
            for edge in Edge::ALL {
                let Some(current) = candidate(other, edge) else {
                    continue;
                };
                if best.is_none_or(|best| current.2 < best.2) {
                    best = Some(current);
                }
            }
        }

        let best = best?;
        let previous = previous
            .and_then(|previous| {
                displays
                    .iter()
                    .find(|display| display.output_key == previous.output_key)
                    .and_then(|display| candidate(display, previous.edge))
            })
            .filter(|previous| previous.2 <= best.2 + SNAP_HYSTERESIS);

        previous.or(Some(best))
    }

    fn apply_snapping_with_target(
        &self,
        dragged_key: OutputKey,
        position: Point,
        size: Size,
        current_position: Point,
        previous: Option<SnapTarget>,
    ) -> (Point, Option<SnapTarget>) {
        if !self
            .displays
            .borrow()
            .iter()
            .any(|display| display.output_key != dragged_key)
        {
            return (position, None);
        }

        let Some((snap_target, target, _score)) =
            self.find_best_snap_target(dragged_key, position, size, current_position, previous)
        else {
            return (current_position, None);
        };

        (
            snap_target.edge.snap_to(target, position, size),
            Some(snap_target),
        )
    }

    fn sync_displays(&self, list: &randr::List, tab_model: &SingleSelectModel) {
        *self.displays.borrow_mut() = Self::display_snapshot(list, tab_model);
        *self.needs_fit.borrow_mut() = true;
    }

    fn handle_drag_move(
        &mut self,
        output_key: OutputKey,
        offset: Vector,
        world_pos: Point,
        previous: Option<SnapTarget>,
    ) {
        let raw_pos = Point::new(world_pos.x - offset.x, world_pos.y - offset.y);

        let (display_size, current_pos) = self
            .displays
            .borrow()
            .iter()
            .find(|display| display.output_key == output_key)
            .map(|display| (display.size, display.position))
            .unwrap_or_default();

        let (snapped_pos, snap_target) = self.apply_snapping_with_target(
            output_key,
            raw_pos,
            display_size,
            current_pos,
            previous,
        );

        if let Some(display) = self
            .displays
            .borrow_mut()
            .iter_mut()
            .find(|display| display.output_key == output_key)
        {
            display.position = snapped_pos;
        }

        if let Interaction::Dragging {
            snap_target: active_target,
            ..
        } = &mut self.interaction
        {
            *active_target = snap_target;
        }
    }

    fn check_refit_needed(&self, bounds_size: Size) {
        if *self.viewport_size.borrow() != bounds_size {
            *self.needs_fit.borrow_mut() = true;
        }
    }
}

fn display_snapshot(list: &randr::List) -> Vec<DisplayRect> {
    let mut tab_model = SingleSelectModel::default();
    for output_key in list.outputs.keys() {
        tab_model.insert().data::<OutputKey>(output_key);
    }
    ArrangementState::display_snapshot(list, &tab_model)
}

pub(super) fn repair_after_logical_size_change(
    previous: &randr::List,
    current: &randr::List,
) -> Vec<(OutputKey, i32, i32)> {
    let previous_displays = display_snapshot(previous);
    let current_displays = display_snapshot(current);

    if previous_displays.len() != current_displays.len() {
        return Vec::new();
    }

    let mut changed_output_key = None;
    for previous_display in &previous_displays {
        let name = previous.outputs[previous_display.output_key].name.as_str();
        let Some(current_display) = current_displays
            .iter()
            .find(|display| current.outputs[display.output_key].name == name)
        else {
            return Vec::new();
        };

        if previous_display.size != current_display.size
            && changed_output_key
                .replace(current_display.output_key)
                .is_some()
        {
            return Vec::new();
        }
    }

    let Some(changed_output_key) = changed_output_key else {
        return Vec::new();
    };
    if ArrangementState::is_connected(&current_displays)
        && !ArrangementState::has_overlaps(&current_displays)
    {
        return Vec::new();
    }

    let mut state = ArrangementState::default();
    *state.displays.get_mut() = current_displays;
    if !state.reflow_around(changed_output_key) {
        return Vec::new();
    }

    state
        .displays
        .into_inner()
        .into_iter()
        .filter_map(|display| {
            let new_position = (
                display.position.x.round() as i32,
                display.position.y.round() as i32,
            );
            (current.outputs[display.output_key].position != new_position).then_some((
                display.output_key,
                new_position.0,
                new_position.1,
            ))
        })
        .collect()
}

type PlacementHandler<'a, Message> = dyn Fn(Vec<(OutputKey, i32, i32)>) -> Message + 'a;

pub struct ArrangementCanvas<'a, Message> {
    list: &'a randr::List,
    tab_model: &'a SingleSelectModel,
    on_placement: Option<Box<PlacementHandler<'a, Message>>>,
    on_select: Option<Box<dyn Fn(cosmic::widget::segmented_button::Entity) -> Message + 'a>>,
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

    pub fn on_placement(mut self, f: impl Fn(Vec<(OutputKey, i32, i32)>) -> Message + 'a) -> Self {
        self.on_placement = Some(Box::new(f));
        self
    }

    pub fn on_select(
        mut self,
        f: impl Fn(cosmic::widget::segmented_button::Entity) -> Message + 'a,
    ) -> Self {
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
                align_x: cosmic::iced::alignment::Horizontal::Center.into(),
                align_y: cosmic::iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas_event::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let interaction_cancelled = match (event, &state.interaction) {
            (canvas_event::Event::Window(window::Event::Unfocused), interaction) => {
                !matches!(interaction, Interaction::None)
            }
            (
                canvas_event::Event::Mouse(mouse::Event::CursorLeft),
                Interaction::Dragging { finger: None, .. } | Interaction::Panning { .. },
            ) => true,
            (
                canvas_event::Event::Touch(touch::Event::FingerLost { id, .. }),
                Interaction::Dragging {
                    finger: Some(active_finger),
                    ..
                },
            ) => id == active_finger,
            _ => false,
        };
        if interaction_cancelled {
            state.interaction = Interaction::None;
            return Some(Action::request_redraw().and_capture());
        }

        let released_finger = match event {
            canvas_event::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                Some(None)
            }
            canvas_event::Event::Touch(touch::Event::FingerLifted { id, .. }) => Some(Some(*id)),
            _ => None,
        };
        let released_output = match (released_finger, &state.interaction) {
            (
                Some(released_finger),
                Interaction::Dragging {
                    output_key, finger, ..
                },
            ) if *finger == released_finger => Some(*output_key),
            _ => None,
        };
        if let Some(output_key) = released_output {
            state.interaction = Interaction::None;

            let dragged_position_changed = state
                .displays
                .borrow()
                .iter()
                .find(|display| display.output_key == output_key)
                .and_then(|display| {
                    let original = self.list.outputs.get(output_key)?.position;
                    let current = (
                        display.position.x.round() as i32,
                        display.position.y.round() as i32,
                    );
                    Some(original != current)
                })
                .unwrap_or(false);
            if !dragged_position_changed {
                return Some(Action::request_redraw().and_capture());
            }

            if !state.close_gaps(output_key) {
                state.sync_displays(self.list, self.tab_model);
                return Some(Action::request_redraw().and_capture());
            }

            let placements = state
                .displays
                .borrow()
                .iter()
                .filter_map(|display| {
                    let new_pos = (
                        display.position.x.round() as i32,
                        display.position.y.round() as i32,
                    );
                    let old_pos = self
                        .list
                        .outputs
                        .get(display.output_key)
                        .map(|output| output.position)?;
                    (old_pos != new_pos).then_some((display.output_key, new_pos.0, new_pos.1))
                })
                .collect::<Vec<_>>();

            if !placements.is_empty() {
                *state.needs_fit.borrow_mut() = true;

                if let Some(ref on_placement) = self.on_placement {
                    return Some(Action::publish(on_placement(placements)).and_capture());
                }
            }
            return Some(Action::request_redraw().and_capture());
        }

        if let canvas_event::Event::Mouse(mouse::Event::ButtonReleased(button)) = event
            && matches!(button, mouse::Button::Middle | mouse::Button::Right)
            && matches!(state.interaction, Interaction::Panning { .. })
        {
            state.interaction = Interaction::None;
            return Some(Action::request_redraw().and_capture());
        }

        let has_active_interaction = !matches!(state.interaction, Interaction::None);
        let cursor_pos = match event {
            canvas_event::Event::Touch(
                touch::Event::FingerPressed { position, .. }
                | touch::Event::FingerMoved { position, .. }
                | touch::Event::FingerLifted { position, .. }
                | touch::Event::FingerLost { position, .. },
            ) => Some(Point::new(position.x - bounds.x, position.y - bounds.y)),
            _ if has_active_interaction => cursor
                .position()
                .map(|pos| Point::new(pos.x - bounds.x, pos.y - bounds.y)),
            _ => cursor.position_in(bounds),
        };

        let cursor_pos = cursor_pos?;

        match event {
            canvas_event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | canvas_event::Event::Touch(touch::Event::FingerPressed { .. }) => {
                if !matches!(state.interaction, Interaction::None) {
                    return None;
                }
                let finger = match event {
                    canvas_event::Event::Touch(touch::Event::FingerPressed { id, .. }) => Some(*id),
                    _ => None,
                };
                let world_pos = state
                    .camera
                    .borrow()
                    .screen_to_world(cursor_pos, bounds.size());
                if let Some(idx) = state.find_display_at(world_pos) {
                    let displays = state.displays.borrow();
                    let display = &displays[idx];
                    let display_key = display.output_key;
                    let display_entity = display.entity;
                    let display_position = display.position;
                    drop(displays);

                    state.interaction = Interaction::Dragging {
                        output_key: display_key,
                        offset: Vector::new(
                            world_pos.x - display_position.x,
                            world_pos.y - display_position.y,
                        ),
                        snap_target: None,
                        finger,
                    };

                    return Some(match &self.on_select {
                        Some(on_select) => Action::publish(on_select(display_entity)).and_capture(),
                        None => Action::capture(),
                    });
                }
            }
            canvas_event::Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Middle | mouse::Button::Right,
            )) => {
                if !matches!(state.interaction, Interaction::None) {
                    return None;
                }
                state.interaction = Interaction::Panning {
                    last_pos: cursor_pos,
                };
                return Some(Action::capture());
            }
            canvas_event::Event::Mouse(mouse::Event::CursorMoved { .. })
            | canvas_event::Event::Touch(touch::Event::FingerMoved { .. }) => {
                match &state.interaction {
                    Interaction::Dragging {
                        output_key,
                        offset,
                        snap_target,
                        finger,
                    } => {
                        let moving_finger = match event {
                            canvas_event::Event::Touch(touch::Event::FingerMoved {
                                id, ..
                            }) => Some(*id),
                            _ => None,
                        };
                        if *finger != moving_finger {
                            return None;
                        }

                        let (output_key, offset, snap_target) =
                            (*output_key, *offset, *snap_target);
                        let world_pos = state
                            .camera
                            .borrow()
                            .screen_to_world(cursor_pos, bounds.size());
                        state.handle_drag_move(output_key, offset, world_pos, snap_target);
                        return Some(Action::request_redraw().and_capture());
                    }
                    Interaction::Panning { last_pos }
                        if matches!(event, canvas_event::Event::Mouse(_)) =>
                    {
                        let delta =
                            Vector::new(cursor_pos.x - last_pos.x, cursor_pos.y - last_pos.y);
                        state.camera.borrow_mut().pan(delta);
                        state.interaction = Interaction::Panning {
                            last_pos: cursor_pos,
                        };
                        return Some(Action::request_redraw().and_capture());
                    }
                    Interaction::Panning { .. } | Interaction::None => {}
                }
            }
            _ => {}
        }

        None
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if !matches!(state.interaction, Interaction::None) {
            return mouse::Interaction::Grabbing;
        }

        let Some(cursor_pos) = cursor.position_in(bounds) else {
            return mouse::Interaction::Idle;
        };
        let world_pos = state
            .camera
            .borrow()
            .screen_to_world(cursor_pos, bounds.size());

        if state.find_display_at(world_pos).is_some() {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::Idle
        }
    }
}

pub type Arrangement<'a, Message> = ArrangementCanvas<'a, Message>;
