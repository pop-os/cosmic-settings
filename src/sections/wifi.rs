// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry, RT};
use cosmic_dbus_networkmanager::{device::SpecificDevice, nm::NetworkManager};
use futures::stream::{self, StreamExt};
use gtk4::{glib, prelude::*, Align, Button, Image, Label, Orientation, Switch};
use std::{cell::RefCell, rc::Rc, time::Duration};
use zbus::Connection;
pub struct WifiSection;

impl Section for WifiSection {
	const NAME: &'static str = "WiFi";
	const ICON: &'static str = "network-wireless-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Single(vec![
			AirplaneMode::boxed(),
			Wifi::boxed(),
			VisibleNetworks::boxed(),
			AdditionalNetworkSettings::boxed(),
		])
	}
}

#[derive(Default)]
struct AirplaneMode;

impl SettingsGroup for AirplaneMode {
	fn title(&self) -> &'static str {
		"Airplane Mode"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["airplane", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let checkbox = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Airplane Mode");
			..set_description("Disables Wi-Fi, Bluetooth, and mobile broadband");
			..set_child(&checkbox);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct Wifi;

impl SettingsGroup for Wifi {
	fn title(&self) -> &'static str {
		"Wi-Fi"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "wireless", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let checkbox = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Wi-Fi");
			..set_description("Disables all Wi-Fi functions");
			..set_child(&checkbox);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct VisibleNetworks {
	access_point_ids: Rc<RefCell<Vec<gtk4::Box>>>,
}

impl VisibleNetworks {}

enum NetworksEvent {
	IdList(Vec<String>),
}

// enum NetworksUiEvent {
// 	Reload,
// }

impl SettingsGroup for VisibleNetworks {
	fn title(&self) -> &'static str {
		"Visible Networks"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "connect", "ssid"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let (net_tx, net_rx) = glib::MainContext::channel::<NetworksEvent>(glib::PRIORITY_DEFAULT);
		// let (ui_tx, ui_rx) = glib::MainContext::channel::<NetworksUiEvent>(glib::PRIORITY_DEFAULT);

		let id_handles = &self.access_point_ids;
		net_rx.attach(
			None,
			glib::clone!(@weak target, @strong id_handles => @default-return glib::Continue(true), move |event| {
				match event {
					NetworksEvent::IdList(ids) => {
						let mut new_ids = vec![];
						for ssid in ids {
							dbg!(&ssid);
							let inner_box = gtk4::Box::new(Orientation::Horizontal, 16);
							let icon = Image::from_icon_name(Some("network-wireless-symbolic"));
							let label = Label::new(Some(&ssid));
							inner_box.append(&icon);
							inner_box.append(&label);
							let connect_button = Button::builder()
								.child(&inner_box)
								.css_classes(vec!["settings-button".into()])
								.hexpand(true)
								.build();
							let settings_button = Button::builder()
								.icon_name("emblem-system-symbolic")
								.css_classes(vec!["settings-button".into()])
								.build();
							let outer_box = gtk4::Box::builder()
								.orientation(Orientation::Horizontal)
								.margin_start(24)
								.margin_end(24)
								.margin_top(8)
								.margin_bottom(8)
								.build();
							outer_box.append(&connect_button);
							outer_box.append(&settings_button);
							target.append(&outer_box);
							new_ids.push(outer_box);
						}
						let old_ids = id_handles.replace(new_ids);
						for id in old_ids {
							id.unparent();
						}
					}
				};
				glib::Continue(true)
			}),
		);

		let handle = RT.get().unwrap().handle();
		handle.spawn(async move {
			let sys_conn = match Connection::system().await {
				Ok(conn) => conn,
				Err(_) => return, //TODO err msg
			};
			// let sys_conn_cell = once_cell::sync::OnceCell::new();
			// sys_conn_cell.set(sys_conn).unwrap();
			// let sys_conn = sys_conn_cell.get().unwrap();

			let nm = match NetworkManager::new(&sys_conn).await {
				Ok(p) => p,
				Err(_) => todo!(), //TODO err msg
			};
			let devices = match nm.devices().await {
				Ok(d) => d,
				Err(_) => todo!(),
			};
			for d in devices {
				if let Ok(Some(SpecificDevice::Wireless(w))) = d.downcast_to_device().await {
					if let Err(_) = w.request_scan(std::collections::HashMap::new()).await {
						eprintln!("Scan failed");
						continue;
					};
					let mut scan_changed = w.receive_last_scan_changed().await;
					while let Some(_t) = scan_changed.next().await {
						println!("scan changed");
						match w.get_access_points().await {
							Ok(aps) => {
								let ssids: Vec<String> = stream::iter(aps)
									.filter_map(|ap| async move {
										ap.ssid()
											.await
											.map(|v| String::from_utf8_lossy(&v).to_string())
											.ok()
									})
									.collect()
									.await;
								dbg!(&ssids);
								net_tx.send(NetworksEvent::IdList(ssids)).unwrap();
							}
							Err(_) => {
								println!("getting access points failed...");
								continue;
							}
						};
					}
				}
			}
		});
	}
}

#[derive(Default)]
struct AdditionalNetworkSettings;

impl AdditionalNetworkSettings {
	pub fn create_hidden_network_popup() -> gtk4::Box {
		let base = gtk4::Box::builder()
			.orientation(Orientation::Vertical)
			.build();
		let label = Label::new(Some("Hello World!"));
		base.append(&label);
		base
	}
}

impl SettingsGroup for AdditionalNetworkSettings {
	fn title(&self) -> &'static str {
		"Additional Network Settings"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"wifi", "wi-fi", "wireless", "hotspot", "hidden", "network", "tether", "hot-spot",
			"hot spot",
		]
	}

	fn layout(&self, target: &gtk4::Box, ui: Rc<SettingsGui>) {
		let button = Button::builder()
			.label("Wi-Fi Hotspot")
			.css_classes(vec!["settings-button".into()])
			.build();
		target.append(&button);
		let button = Button::builder()
			.label("Connect to Hidden Networks")
			.css_classes(vec!["settings-button".into()])
			.build();
		button.connect_clicked(glib::clone!(@strong ui => move |_| {
			ui.popup.pop_up("hidden-net");
		}));
		target.append(&button);
		ui.popup
			.add_overlay("hidden-net", Self::create_hidden_network_popup);
	}
}
