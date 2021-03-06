// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod desktop;
mod keyboard;
mod wifi;

pub use self::{
	about::AboutSection, desktop::DesktopSection, keyboard::KeyboardSection, wifi::WifiSection,
};
use crate::ui::SettingsGui;
use std::{cell::RefCell, rc::Rc};

/// A section of the COSMIC settings app.
pub trait Section: Sized {
	/// The name this section appears as.
	const NAME: &'static str;
	/// The icon that will appear next to this setting in the nav.
	const ICON: &'static str;
	/// Returns all the settings groups from this section.
	fn layout() -> SectionLayout;
}

/// The layout of the groups in a [`Section`].
pub enum SectionLayout {
	/// The section has a single page of settings groups.
	Single(Vec<Box<dyn SettingsGroup>>),
	/// The section has multiple pages of settings groups.
	///
	/// This is a [`Vec`] and not a [`HashMap`] or [`BTreeMap`] due to the fact that they may
	/// be arbitraily sorted in a specific way.
	Multiple(Vec<(&'static str, Vec<Box<dyn SettingsGroup>>)>),
}

/// A group of settings in the COSMIC settings app.
pub trait SettingsGroup {
	/// Returns a new instance of this settings group in a box.
	fn boxed() -> Box<dyn SettingsGroup>
	where
		Self: 'static + Default,
	{
		Box::new(Self::default()) as _
	}
	/// Returns the title of this group of settings.
	/// No title will be displayed if an empty string is returned.
	fn title(&self) -> &'static str {
		""
	}
	/// Returns the keywords that this group will show up in search with.
	fn keywords(&self) -> &'static [&'static str];
	/// Lays out the widgets from this setting group in the given Box.
	fn layout(&self, target: &gtk4::Box, ui: Rc<SettingsGui>);
}

/// Where all the settings groups are stored. Used for search.
pub type SettingsGroupStore = Rc<RefCell<Vec<Box<dyn SettingsGroup>>>>;
