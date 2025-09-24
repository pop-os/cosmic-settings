// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use cosmic::Renderer;
use cosmic::iced_core::renderer::Quad;
use cosmic::iced_core::widget::{Tree, tree};
use cosmic::iced_core::{
    self as core, Border, Clipboard, Element, Layout, Length, Rectangle, Renderer as IcedRenderer,
    Shell, Size, Widget,
};
use cosmic::iced_core::{Point, layout, mouse, renderer, touch};
use cosmic::iced_core::{alignment, event, text};
use cosmic::widget::segmented_button::{self, SingleSelectModel};
use cosmic_randr_shell::{self as randr, OutputKey};
use randr::Transform;

const UNIT_PIXELS: f32 = 12.0;
const VERTICAL_OVERHEAD: f32 = 1.5;
const VERTICAL_DISPLAY_OVERHEAD: f32 = 4.0;

pub type OnPlacementFunc<Message> = Box<dyn Fn(OutputKey, i32, i32) -> Message>;
pub type OnSelectFunc<Message> = Box<dyn Fn(segmented_button::Entity) -> Message>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pan {
    Left,
    Right,
}

#[must_use]
#[derive(derive_setters::Setters)]
pub struct Arrangement<'a, Message> {
    #[setters(skip)]
    list: &'a randr::List,
    #[setters(skip)]
    tab_model: &'a SingleSelectModel,
    #[setters(skip)]
    on_pan: Option<Box<dyn Fn(Pan) -> Message>>,
    #[setters(skip)]
    on_placement: Option<OnPlacementFunc<Message>>,
    #[setters(skip)]
    on_select: Option<OnSelectFunc<Message>>,
    width: Length,
    height: Length,
}

impl<'a, Message> Arrangement<'a, Message> {
    pub fn new(list: &'a randr::List, tab_model: &'a SingleSelectModel) -> Self {
        Self {
            list,
            tab_model,
            on_pan: None,
            on_placement: None,
            on_select: None,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    pub fn on_pan(mut self, on_pan: impl Fn(Pan) -> Message + 'static) -> Self {
        self.on_pan = Some(Box::new(on_pan));
        self
    }

    pub fn on_placement(
        mut self,
        on_placement: impl Fn(OutputKey, i32, i32) -> Message + 'static,
    ) -> Self {
        self.on_placement = Some(Box::new(on_placement));
        self
    }

    pub fn on_select(
        mut self,
        on_select: impl Fn(segmented_button::Entity) -> Message + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }
}

impl<'a, Message: Clone> Widget<Message, cosmic::Theme, Renderer> for Arrangement<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Determine the max display dimensions, and the total display area utilized.
        let mut total_height = 0;
        let mut max_dimensions = (0, 0);
        let mut display_area = (0, 0);

        for output in self.list.outputs.values() {
            if !output.enabled {
                continue;
            }

            let Some(mode_key) = output.current else {
                continue;
            };

            let Some(mode) = self.list.modes.get(mode_key) else {
                continue;
            };

            let (mut width, mut height) = if output.transform.map_or(true, is_landscape) {
                (mode.size.0, mode.size.1)
            } else {
                (mode.size.1, mode.size.0)
            };

            // Scale dimensions of the display with the output scale.
            width = (width as f64 / output.scale) as u32;
            height = (height as f64 / output.scale) as u32;

            max_dimensions.0 = max_dimensions.0.max(width);
            max_dimensions.1 = max_dimensions.1.max(height);

            display_area.0 = display_area.0.max(width as i32 + output.position.0);
            display_area.1 = display_area.1.max(height as i32 + output.position.1);

            total_height = total_height.max(height as i32 + output.position.1);
        }

        let state = tree.state.downcast_mut::<State>();

        state.max_dimensions = (
            max_dimensions.0 as f32 / UNIT_PIXELS,
            total_height as f32 / UNIT_PIXELS,
        );

        let width = ((max_dimensions.0 as f32 * 2.0) as i32 + display_area.0) as f32 / UNIT_PIXELS;
        let height = total_height as f32 * VERTICAL_OVERHEAD / UNIT_PIXELS;

        let limits = limits
            .width(Length::Fixed(width))
            .height(Length::Fixed(height));

        let size = limits.resolve(width, height, Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: cosmic::iced_core::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let bounds = layout.bounds();

        match event {
            core::Event::Mouse(mouse::Event::CursorMoved { position, .. })
            | core::Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                let state = tree.state.downcast_mut::<State>();

                if let Some((output_key, region)) = state.dragging.as_mut() {
                    if let Some(ref mut on_pan) = self.on_pan {
                        if bounds.x + viewport.width - 150.0 < position.x {
                            shell.publish(on_pan(Pan::Right));
                        } else if bounds.x + 150.0 > position.x {
                            shell.publish(on_pan(Pan::Left));
                        }
                    }

                    if let Some(inner_position) = cursor.position() {
                        update_dragged_region(
                            self.tab_model,
                            self.list,
                            &bounds,
                            *output_key,
                            region,
                            state.max_dimensions,
                            (
                                inner_position.x - state.offset.0,
                                inner_position.y - state.offset.1,
                            ),
                        );

                        return event::Status::Captured;
                    }
                }
            }

            core::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | core::Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(position) = cursor.position() {
                    let state = tree.state.downcast_mut::<State>();
                    if let Some((output_key, output_region)) = display_region_hovers(
                        self.tab_model,
                        self.list,
                        &bounds,
                        state.max_dimensions,
                        position,
                    ) {
                        state.drag_from = position;
                        state.offset = (position.x - output_region.x, position.y - output_region.y);
                        state.dragging = Some((output_key, output_region));
                        return event::Status::Captured;
                    }
                }
            }

            core::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | core::Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                if let Some((output_key, region)) = state.dragging.take() {
                    if let Some(position) = cursor.position() {
                        if position.distance(state.drag_from) < 4.0 {
                            if let Some(ref on_select) = self.on_select {
                                for id in self.tab_model.iter() {
                                    if let Some(&key) = self.tab_model.data::<OutputKey>(id) {
                                        if key == output_key {
                                            shell.publish(on_select(id));
                                        }
                                    }
                                }
                            }

                            return event::Status::Captured;
                        }
                    }

                    if let Some(ref on_placement) = self.on_placement {
                        shell.publish(on_placement(
                            output_key,
                            ((region.x - state.max_dimensions.0 - bounds.x) * UNIT_PIXELS) as i32,
                            ((region.y
                                - (state.max_dimensions.1 / VERTICAL_DISPLAY_OVERHEAD)
                                - bounds.y)
                                * UNIT_PIXELS) as i32,
                        ));
                    }

                    return event::Status::Captured;
                }
            }

            _ => (),
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        for (_output_key, region) in
            display_regions(self.tab_model, self.list, &bounds, state.max_dimensions)
        {
            if cursor.is_over(region) {
                return mouse::Interaction::Grab;
            }
        }

        mouse::Interaction::Idle
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &cosmic::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        let bounds = layout.bounds();
        let theme = cosmic::theme::active();
        let cosmic_theme = theme.cosmic();

        let border_color = cosmic_theme.palette.neutral_7;

        let active_key = self.tab_model.active_data::<OutputKey>();

        for (id, (output_key, mut region)) in
            display_regions(self.tab_model, self.list, &bounds, state.max_dimensions).enumerate()
        {
            // If the output is being dragged, show its dragged position instead.
            if let Some((dragged_key, dragged_region)) = state.dragging {
                if dragged_key == output_key {
                    region = dragged_region;
                }
            }

            let (background, border_color) = if Some(&output_key) == active_key {
                let mut border_color = border_color;
                border_color.alpha = 0.4;

                (cosmic_theme.accent_color(), border_color)
            } else {
                (cosmic_theme.palette.neutral_4, border_color)
            };

            renderer.fill_quad(
                Quad {
                    bounds: region,
                    border: Border {
                        color: border_color.into(),
                        radius: 4.0.into(),
                        width: 3.0,
                    },
                    shadow: Default::default(),
                },
                core::Background::Color(background.into()),
            );

            let id_bounds = Rectangle {
                x: region.x + (region.width / 2.0 - 36.0),
                y: region.y + (region.height / 2.0 - 23.0),
                width: 72.0,
                height: 46.0,
            };

            renderer.fill_quad(
                Quad {
                    bounds: id_bounds,
                    border: Border {
                        radius: 30.0.into(),
                        ..Default::default()
                    },
                    shadow: Default::default(),
                },
                core::Background::Color(cosmic_theme.palette.neutral_1.into()),
            );

            core::text::Renderer::fill_text(
                renderer,
                core::Text {
                    content: itoa::Buffer::new().format(id + 1).to_string(),
                    size: core::Pixels(24.0),
                    line_height: core::text::LineHeight::Relative(1.2),
                    font: cosmic::font::bold(),
                    bounds: id_bounds.size(),
                    horizontal_alignment: alignment::Horizontal::Center,
                    vertical_alignment: alignment::Vertical::Center,
                    shaping: text::Shaping::Basic,
                    wrapping: text::Wrapping::Word,
                },
                core::Point {
                    x: id_bounds.center_x(),
                    y: id_bounds.center_y(),
                },
                cosmic_theme.palette.neutral_10.into(),
                *viewport,
            );
        }
    }
}

impl<'a, Message: 'static + Clone> From<Arrangement<'a, Message>> for cosmic::Element<'a, Message> {
    fn from(display_positioner: Arrangement<'a, Message>) -> Self {
        Element::new(display_positioner)
    }
}

#[derive(Default)]
struct State {
    drag_from: Point,
    dragging: Option<(OutputKey, Rectangle)>,
    offset: (f32, f32),
    max_dimensions: (f32, f32),
}

/// Iteratively calculate display regions for each display output in the list.
fn display_regions<'a>(
    model: &'a SingleSelectModel,
    list: &'a randr::List,
    bounds: &'a Rectangle,
    max_dimensions: (f32, f32),
) -> impl Iterator<Item = (OutputKey, Rectangle)> + 'a {
    model
        .iter()
        .filter_map(move |id| model.data::<OutputKey>(id))
        .filter_map(move |&key| {
            let output = list.outputs.get(key)?;

            if !output.enabled {
                return None;
            }

            let mode_key = output.current?;

            let mode = list.modes.get(mode_key)?;

            let (mut width, mut height) = (
                (mode.size.0 as f32 / output.scale as f32) / UNIT_PIXELS,
                (mode.size.1 as f32 / output.scale as f32) / UNIT_PIXELS,
            );

            (width, height) = if output.transform.map_or(true, is_landscape) {
                (width, height)
            } else {
                (height, width)
            };

            Some((
                key,
                Rectangle {
                    width,
                    height,
                    x: max_dimensions.0 + bounds.x + (output.position.0 as f32) / UNIT_PIXELS,
                    y: (max_dimensions.1 / VERTICAL_DISPLAY_OVERHEAD)
                        + bounds.y
                        + (output.position.1 as f32) / UNIT_PIXELS,
                },
            ))
        })
}

fn display_region_hovers(
    model: &SingleSelectModel,
    list: &randr::List,
    bounds: &Rectangle,
    max_dimensions: (f32, f32),
    point: Point,
) -> Option<(OutputKey, Rectangle)> {
    for (output_key, region) in display_regions(model, list, bounds, max_dimensions) {
        if region.contains(point) {
            return Some((output_key, region));
        }
    }

    None
}

/// Updates a display's region, preventing coordinates from overlapping with existing displays.
fn update_dragged_region(
    model: &SingleSelectModel,
    list: &randr::List,
    bounds: &Rectangle,
    output: OutputKey,
    region: &mut Rectangle,
    max_dimensions: (f32, f32),
    (x, y): (f32, f32),
) {
    let mut dragged_region = Rectangle { x, y, ..*region };

    let mut nearest = f32::MAX;
    let mut nearest_region = Rectangle::default();
    let mut nearest_side = NearestSide::East;

    // Find the nearest adjacent display to the dragged display.
    for (other_output, other_region) in display_regions(model, list, bounds, max_dimensions) {
        if other_output == output {
            continue;
        }

        let center = dragged_region.center();

        let eastward = distance(east_point(&other_region), center) * 1.25;
        let westward = distance(west_point(&other_region), center) * 1.25;
        let northward = distance(north_point(&other_region), center);
        let southward = distance(south_point(&other_region), center);

        let mut nearer = false;

        if nearest > eastward {
            (nearest, nearest_side, nearer) = (eastward, NearestSide::East, true);
        }

        if nearest > westward {
            (nearest, nearest_side, nearer) = (westward, NearestSide::West, true);
        }

        if nearest > northward {
            (nearest, nearest_side, nearer) = (northward, NearestSide::North, true);
        }

        if nearest > southward {
            (nearest, nearest_side, nearer) = (southward, NearestSide::South, true);
        }

        if nearer {
            nearest_region = other_region;
        }
    }

    // Attach dragged display to nearest adjacent display.
    match nearest_side {
        NearestSide::East => {
            dragged_region.x = nearest_region.x - dragged_region.width;
            dragged_region.y = dragged_region
                .y
                .max(nearest_region.y - dragged_region.height + 8.0)
                .min(nearest_region.y + nearest_region.height - 8.0);
        }

        NearestSide::North => {
            dragged_region.y = nearest_region.y - dragged_region.height;
            dragged_region.x = dragged_region
                .x
                .max(nearest_region.x - dragged_region.width + 8.0)
                .min(nearest_region.x + nearest_region.width - 8.0);
        }

        NearestSide::West => {
            dragged_region.x = nearest_region.x + nearest_region.width;
            dragged_region.y = dragged_region
                .y
                .max(nearest_region.y - dragged_region.height + 8.0)
                .min(nearest_region.y + nearest_region.height - 8.0);
        }

        NearestSide::South => {
            dragged_region.y = nearest_region.y + nearest_region.height;
            dragged_region.x = dragged_region
                .x
                .max(nearest_region.x - dragged_region.width + 8.0)
                .min(nearest_region.x + nearest_region.width - 8.0);
        }
    }

    // Snap-align on x-axis when alignment is near.
    if (dragged_region.x - nearest_region.x).abs() <= 8.0 {
        dragged_region.x = nearest_region.x;
    }

    // Snap-align on x-axis when alignment is near bottom edge.
    if ((dragged_region.x + dragged_region.width) - (nearest_region.x + nearest_region.width)).abs()
        <= 8.0
    {
        dragged_region.x = nearest_region.x + nearest_region.width - dragged_region.width;
    }

    // Snap-align on y-axis when alignment is near.
    if (dragged_region.y - nearest_region.y).abs() <= 8.0 {
        dragged_region.y = nearest_region.y;
    }

    // Snap-align on y-axis when alignment is near bottom edge.
    if ((dragged_region.y + dragged_region.height) - (nearest_region.y + nearest_region.height))
        .abs()
        <= 8.0
    {
        dragged_region.y = nearest_region.y + nearest_region.height - dragged_region.height;
    }

    // Prevent display from overlapping with other displays.
    for (other_output, other_region) in display_regions(model, list, bounds, max_dimensions) {
        if other_output == output {
            continue;
        }

        if other_region.intersects(&dragged_region) {
            return;
        }
    }

    *region = dragged_region;
}

fn is_landscape(transform: Transform) -> bool {
    matches!(
        transform,
        Transform::Normal | Transform::Rotate180 | Transform::Flipped | Transform::Flipped180
    )
}

#[derive(Debug)]
enum NearestSide {
    East,
    North,
    South,
    West,
}

fn distance(a: Point, b: Point) -> f32 {
    ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)).sqrt()
}

fn east_point(r: &Rectangle) -> Point {
    Point {
        x: r.x,
        y: r.center_y(),
    }
}

fn north_point(r: &Rectangle) -> Point {
    Point {
        x: r.center_x(),
        y: r.y,
    }
}

fn west_point(r: &Rectangle) -> Point {
    Point {
        x: r.x + r.width,
        y: r.center_y(),
    }
}

fn south_point(r: &Rectangle) -> Point {
    Point {
        x: r.center_x(),
        y: r.y + r.height,
    }
}
