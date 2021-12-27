mod desktop;
mod wifi;

pub use desktop::DesktopSection;
pub use wifi::WifiSection;

/// A section of the COSMIC settings app.
pub trait Section: Sized {
	/// The name this section appears as.
	const NAME: &'static str;
	/// The icon that will appear next to this setting in the nav.
	const ICON: &'static str;
	/// Returns all the settings groups from this section.
	fn settings() -> Vec<Box<dyn SettingsGroup>>;
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
	fn layout(&self, target: &gtk4::Box);
}
