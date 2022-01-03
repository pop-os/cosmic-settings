// SPDX-License-Identifier: GPL-3.0-only

use gtk4::{
	glib::{self, clone},
	prelude::*,
	Button, ListBox, Orientation, Revealer, Stack,
};

/// A struct containing references to various elements of COSMIC setting's navigation GUI.
#[derive(Clone)]
pub struct SettingsNavGui {
	/// The revealer that handles the primary nav bar
	pub revealer: Revealer,
	/// The revealer that handles the subsection
	pub subsection_revealer: Revealer,
	/// The button that hides/shows the nav bar.
	pub button: Button,
	/// The box containing the primary nav bar.
	pub nav_box: gtk4::Box,
	/// The primary list containg nav entries
	pub list: ListBox,
	/// The stack containing the menus for subsections
	pub stack: Stack,
}

impl SettingsNavGui {
	pub fn new(header_box: &gtk4::Box) -> Self {
		let nav_box = Self::create_nav_box();
		let stack = Self::create_stack();
		let revealer = Self::create_revealer(&nav_box);
		let subsection_revealer = Self::create_subsection_revealer(&stack);
		let button = Self::create_button(header_box, &revealer);
		let list = gtk4::ListBox::new();
		nav_box.append(&list);
		nav_box.append(&subsection_revealer);
		revealer.set_child(Some(&nav_box));
		Self {
			revealer,
			subsection_revealer,
			button,
			nav_box,
			list,
			stack,
		}
	}

	fn create_nav_box() -> gtk4::Box {
		gtk4::Box::builder()
			.orientation(Orientation::Horizontal)
			.margin_top(20)
			.margin_bottom(20)
			.margin_start(12)
			.margin_end(12)
			.css_classes(vec!["nav".into()])
			.build()
	}

	fn create_stack() -> Stack {
		Stack::builder()
			.css_classes(vec!["nav-subsection".into()])
			.build()
	}

	fn create_revealer(nav_box: &gtk4::Box) -> Revealer {
		Revealer::builder()
			.child(nav_box)
			.transition_type(gtk4::RevealerTransitionType::SlideRight)
			.build()
	}

	fn create_subsection_revealer(stack: &Stack) -> Revealer {
		Revealer::builder()
			.child(stack)
			.transition_type(gtk4::RevealerTransitionType::SlideRight)
			.build()
	}

	fn create_button(header_box: &gtk4::Box, revealer: &Revealer) -> Button {
		let button_box = gtk4::Box::builder()
			.orientation(gtk4::Orientation::Horizontal)
			.valign(gtk4::Align::Center)
			.spacing(8)
			.margin_start(10)
			.margin_end(10)
			.build();
		let button_label = gtk4::Label::new(Some("Navigation"));
		button_box.append(&button_label);
		let button_sep = gtk4::Separator::new(Orientation::Vertical);
		button_box.append(&button_sep);
		let button_icon = gtk4::Image::from_icon_name(Some("go-next-symbolic"));
		button_box.append(&button_icon);
		let button = gtk4::Button::builder()
			.child(&button_box)
			.margin_top(10)
			.css_classes(vec!["nav-button".into()])
			.build();
		button.connect_clicked(clone!(@weak revealer, @weak button_icon => move |_| {
			let active = revealer.reveals_child();
			if active {
				button_icon.set_icon_name(Some("go-next-symbolic"));
				revealer.set_reveal_child(false);
			} else {
				button_icon.set_icon_name(Some("go-previous-symbolic"));
				revealer.set_reveal_child(true);
			}
		}));
		header_box.append(&button);
		button
	}
}
