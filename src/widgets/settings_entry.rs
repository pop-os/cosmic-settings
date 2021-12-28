use gtk4::{
	glib::{self, Object},
	prelude::*,
	subclass::prelude::*,
};
use std::cell::RefCell;

glib::wrapper! {
	pub struct SettingsEntry(ObjectSubclass<SettingsEntryImp>)
		@extends gtk4::Widget,
		@implements gtk4::Accessible;
}

impl SettingsEntry {
	pub fn new() -> Self {
		Object::new(&[]).expect("Failed to create `SettingsEntry`.")
	}

	pub fn set_child<'a, Widget, IntoWidget>(&self, child: IntoWidget)
	where
		Widget: IsA<gtk4::Widget>,
		IntoWidget: Into<Option<&'a Widget>>,
	{
		let imp = self.inner();
		let child = child.into().map(|x| x.as_ref());
		let child_box_ref = imp.child_box.borrow();
		let child_box: &gtk4::Box = child_box_ref.as_ref().expect("child_box not created??");
		if let Some(new_child) = child {
			new_child.set_halign(gtk4::Align::End);
			child_box.append(new_child);
		}
		if let Some(old_child) = imp.child.replace(child.cloned()) {
			child_box.remove(&old_child);
		}
	}

	pub fn set_title(&self, title: &str) {
		let inner = self.inner();
		if let Some(label) = inner.title.borrow().as_ref() {
			label.set_label(title);
		}
	}

	pub fn set_description(&self, description: &str) {
		let inner = self.inner();
		if let Some(label) = inner.desc.borrow().as_ref() {
			label.set_label(description);
		}
	}

	fn inner(&self) -> &SettingsEntryImp {
		SettingsEntryImp::from_instance(self)
	}
}

#[derive(Debug, Default)]
pub struct SettingsEntryImp {
	title: RefCell<Option<gtk4::Label>>,
	desc: RefCell<Option<gtk4::Label>>,
	child_box: RefCell<Option<gtk4::Box>>,
	child: RefCell<Option<gtk4::Widget>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SettingsEntryImp {
	const NAME: &'static str = "SettingsEntry";
	type Type = SettingsEntry;
	type ParentType = gtk4::Widget;
	type Interfaces = ();
	type Instance = glib::subclass::basic::InstanceStruct<Self>;
	type Class = glib::subclass::basic::ClassStruct<Self>;

	fn class_init(klass: &mut Self::Class) {
		// The layout manager determines how child widgets are laid out.
		klass.set_layout_manager_type::<gtk4::BinLayout>();
	}
}

impl ObjectImpl for SettingsEntryImp {
	fn constructed(&self, obj: &Self::Type) {
		self.parent_constructed(obj);
		let child = gtk4::Box::builder()
			.css_classes(vec!["settings-entry".into()])
			.orientation(gtk4::Orientation::Horizontal)
			.hexpand(true)
			.margin_start(24)
			.margin_end(8)
			.margin_top(8)
			.margin_bottom(8)
			.build();

		let title_and_desc = gtk4::Box::builder()
			.css_classes(vec!["settings-entry-info".into()])
			.orientation(gtk4::Orientation::Vertical)
			.spacing(4)
			.hexpand(true)
			.build();
		let title = gtk4::Label::builder()
			.css_classes(vec!["settings-entry-title".into()])
			.halign(gtk4::Align::Start)
			.build();
		let desc = gtk4::Label::builder()
			.css_classes(vec!["settings-entry-desc".into()])
			.halign(gtk4::Align::Start)
			.build();
		title_and_desc.append(&title);
		title_and_desc.append(&desc);
		*self.title.borrow_mut() = Some(title);
		*self.desc.borrow_mut() = Some(desc);
		child.append(&title_and_desc);
		if let Some(entry_child) = self.child.borrow().as_ref() {
			child.append(entry_child);
		}
		child.set_parent(obj);
		*self.child_box.borrow_mut() = Some(child);
	}

	fn dispose(&self, _obj: &Self::Type) {
		if let Some(child) = self.child.borrow_mut().take() {
			child.unparent();
		}
		if let Some(child_box) = self.child_box.borrow_mut().take() {
			child_box.unparent();
		}
	}
}

impl WidgetImpl for SettingsEntryImp {}
