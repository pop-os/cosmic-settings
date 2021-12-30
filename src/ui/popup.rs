use gtk4::{prelude::*, Align, Overlay, Revealer, RevealerTransitionType, Stack, Widget};

#[derive(Clone)]
pub struct PopupGui {
	pub overlay: Overlay,
	pub revealer: Revealer,
	pub stack: Stack,
}

impl PopupGui {
	pub fn new(child: &gtk4::Box) -> Self {
		let stack = Stack::new();
		let revealer = Revealer::builder()
			.child(&stack)
			//.halign(Align::End)
			.transition_type(RevealerTransitionType::SlideLeft)
			.build();
		let overlay = Overlay::builder().child(child).build();
		overlay.add_overlay(&revealer);
		revealer.hide();
		Self {
			stack,
			revealer,
			overlay,
		}
	}

	pub fn pop_up(&self, name: &str) {
		self.revealer.show();
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
