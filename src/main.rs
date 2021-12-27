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

	let base_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
		.margin_top(16)
		.margin_bottom(0)
		.margin_start(16)
		.margin_end(32)
		.build();
	let nav = gtk4::ListBox::builder(). margin_top(20).margin_bottom(20).margin_start(12).margin_end(12).build();

	// TODO: clean this up! Perhaps make the menu items based on a trait and collect them using inventory?
	let (wifi_icon, wifi_label) = ( gtk4::Image::from_icon_name(Some("network-wireless")), gtk4::Label::new(Some("WiFi")) );
	let wifi_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(8).build();
	wifi_box.append(&wifi_icon);
	wifi_box.append(&wifi_label);

	let wifi_button = gtk4::Button::builder().child(&wifi_box).build();
	nav.append(&wifi_button);
	base_box.append(&nav);

	window.set_child(Some(&base_box));

	window.show();
}