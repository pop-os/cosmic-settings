mod desktop;
mod wifi;

pub use desktop::DesktopSection;
pub use wifi::WifiSection;

pub trait SectionInfo {
	/// The name this section appears as.
	const NAME: &'static str;
	/// The icon that will appear next to this setting in the nav.
	const ICON: &'static str;
}

/// A section of the COSMIC settings app.
pub trait Section: SectionInfo + Sized + Sync + Send + 'static {
	/// Creates the widget that will be displayed when this menu object is selected.
	fn widget(boxed: &gtk4::Box);
}
