// SPDX-License-Identifier: GPL-3.0-only

mod desktop;
mod keyboard;
mod wifi;

pub use self::{desktop::DesktopSection, keyboard::KeyboardSection, wifi::WifiSection};
use crate::ui::SettingsGui;
use std::rc::Rc;

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
	fn new() -> Box<dyn SettingsGroup>
	where
		Self: 'static + Default,
	{
		Box::new(Self::default()) as _
	}
	/// Returns the title of this group of settings.
	fn title(&self) -> &'static str;
	/// Returns the keywords that this group will show up in search with.
	fn keywords(&self) -> &[&'static str];
	/// Lays out the widgets from this setting group in the given Box.
	fn layout(&self, target: &gtk4::Box, ui: Rc<SettingsGui>);
}
