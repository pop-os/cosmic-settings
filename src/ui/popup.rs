// SPDX-License-Identifier: GPL-3.0-only

use gtk4::{
	glib::{self, clone},
	prelude::*,
	Align, Button, Label, Orientation, Overlay, Revealer, RevealerTransitionType, Stack, Widget,
};

#[derive(Clone)]
pub struct PopupGui {
	pub overlay: Overlay,
	pub revealer: Revealer,
	pub stack: Stack,
	pub label: Label,
}

impl PopupGui {
	pub fn new(child: &gtk4::Box) -> Self {
		let stack = Self::create_stack();
		let revealer = Self::create_revealer();
		let button = Self::create_button(&revealer);
		let overlay = Overlay::builder().child(child).build();
		let label = Label::builder()
			.halign(Align::Center)
			.valign(Align::Center)
			.build();
		let top_box = gtk4::Box::new(Orientation::Horizontal, 60);
		top_box.append(&label);
		top_box.append(&button);
		let internal_box = gtk4::Box::builder()
			.orientation(Orientation::Vertical)
			.css_classes(vec!["settings-popup".into()])
			.build();
		internal_box.append(&top_box);
		internal_box.append(&stack);
		revealer.set_child(Some(&internal_box));
		overlay.add_overlay(&revealer);
		Self {
			stack,
			revealer,
			overlay,
			label,
		}
	}

	fn create_stack() -> Stack {
		Stack::builder()
			.margin_top(24)
			.margin_bottom(24)
			.margin_start(24)
			.margin_end(24)
			.build()
	}

	fn create_revealer() -> Revealer {
		Revealer::builder()
			.halign(Align::End)
			.valign(Align::Start)
			.transition_type(RevealerTransitionType::SlideLeft)
			.margin_end(24)
			.build()
	}

	fn create_button(revealer: &Revealer) -> Button {
		let button = Button::builder()
			.label("Close")
			.halign(Align::End)
			.valign(Align::Center)
			.css_classes(vec!["settings-popup-close".into()])
			.build();
		button.connect_clicked(clone!(@weak revealer => move |_| {
			revealer.set_reveal_child(false);
		}));
		button
	}

	pub fn pop_up(&self, name: &str) {
		self.label.set_text(name);
		self.stack.set_visible_child_name(name);
		self.revealer.set_reveal_child(true);
	}

	pub fn add_overlay<W, F>(&self, name: &str, create_overlay: F)
	where
		W: IsA<Widget>,
		F: FnOnce() -> W,
	{
		let new_overlay = create_overlay();
		self.stack.add_named(&new_overlay, Some(name));
	}
}
