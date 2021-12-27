use gtk4::prelude::*;

fn main() {
	let application =
		gtk4::Application::new(Some("com.system76.cosmic.settings"), Default::default());
	application.connect_activate(build_ui);
	application.run();
}

fn build_ui(application: &gtk4::Application) {
	let window = gtk4::ApplicationWindow::new(application);

	window.set_title(Some("Settings"));
	window.set_default_size(842, 632);

	window.show();
}