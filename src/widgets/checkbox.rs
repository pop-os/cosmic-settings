use gtk4::{glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

glib::wrapper! {
	pub struct SettingsCheckbox(ObjectSubclass<SettingsCheckboxImp>)
		@extends gtk4::Widget;
}

#[derive(Debug, Default)]
pub struct SettingsCheckboxImp {
	child: RefCell<Option<gtk4::Box>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SettingsCheckboxImp {
	const NAME: &'static str = "SettingsCheckbox";
	type Type = SettingsCheckbox;
	type ParentType = gtk4::Widget;
	type Interfaces = ();
	type Instance = glib::subclass::basic::InstanceStruct<Self>;
	type Class = glib::subclass::basic::ClassStruct<Self>;

	fn class_init(klass: &mut Self::Class) {
		// The layout manager determines how child widgets are laid out.
		klass.set_layout_manager_type::<gtk4::BinLayout>();
	}
}

impl ObjectImpl for SettingsCheckboxImp {
	fn constructed(&self, obj: &Self::Type) {
		self.parent_constructed(obj);
		let child = gtk4::Box::builder()
			.css_classes(vec!["settings-checkbox".into()])
			.orientation(gtk4::Orientation::Horizontal)
			.build();
		child.set_parent(obj);
	}

	fn dispose(&self, _obj: &Self::Type) {
		if let Some(child) = self.child.borrow_mut().take() {
			child.unparent();
		}
	}
}

impl WidgetImpl for SettingsCheckboxImp {}
