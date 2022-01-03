// SPDX-License-Identifier: GPL-3.0-only

use gtk4::{prelude::*, Align, Overlay, Revealer, RevealerTransitionType, Stack, Widget};

#[derive(Clone)]
pub struct PopupGui {
	pub overlay: Overlay,
	pub revealer: Revealer,
	pub stack: Stack,
}

impl PopupGui {
	pub fn new(child: &gtk4::Box) -> Self {
		let stack = Stack::builder()
			.margin_top(24)
			.margin_bottom(24)
			.margin_start(24)
			.margin_end(24)
			.build();
		let revealer = Revealer::builder()
			.child(&stack)
			.halign(Align::End)
			.valign(Align::Center)
			.transition_type(RevealerTransitionType::SlideLeft)
			.css_classes(vec!["settings-popup".into()])
			.margin_end(24)
			.build();
		let overlay = Overlay::builder().child(child).build();
		overlay.add_overlay(&revealer);
		Self {
			stack,
			revealer,
			overlay,
		}
	}

	pub fn pop_up(&self, name: &str) {
		self.stack.set_visible_child_name(name);
		self.revealer.set_reveal_child(true);
	}

	pub fn dismiss(&self) {
		self.revealer.set_reveal_child(false);
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
