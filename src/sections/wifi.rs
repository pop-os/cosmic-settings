// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry, RT};
use cosmic_dbus_networkmanager::{
	device::SpecificDevice, interface::enums::ApSecurityFlags, nm::NetworkManager,
};
use futures::stream::{self, StreamExt};
use gtk4::glib::Sender;
use gtk4::{glib, prelude::*, Align, Button, Image, Label, Orientation, Spinner, Switch};
use std::{cell::RefCell, rc::Rc};
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

struct VisibleNetworks {
	spinner: Spinner,
	access_point_ids: Rc<RefCell<Vec<gtk4::Box>>>,
}

impl Default for VisibleNetworks {
	fn default() -> Self {
		view! {
			spinner = Spinner {
				set_margin_top: 8,
				set_margin_bottom: 8,
				set_margin_start: 8,
				set_margin_end: 8,
				set_halign: Align::Center,
				set_spinning: true
			}
		}
		let access_point_ids = Rc::default();
		Self {
			spinner,
			access_point_ids,
		}
	}
}

impl VisibleNetworks {
	fn handle_access_point(
		target: gtk4::Box,
		aps: Vec<AccessPoint>,
		id_handles: Rc<RefCell<Vec<gtk4::Box>>>,
	) {
		let mut new_ids = vec![];
		for ap in aps {
			dbg!(&ap);
			view! {
				outer_box = gtk4::Box {
					set_orientation: Orientation::Horizontal,
					set_margin_start: 24,
					set_margin_end: 24,
					set_margin_top: 8,
					set_margin_bottom: 8,
					append: connect_button = &Button {
						add_css_class: "settings-button",
						set_hexpand: true,
						set_child: inner_box = Some(&gtk4::Box) {
							set_orientation: Orientation::Horizontal,
							set_spacing: 16,
							append: icon = &Image::from_icon_name(Some("network-wireless-symbolic")) {},
							append: label = &Label::new(Some(&ap.ssid)) {}
						}
					},
					append: settings_button = &Button {
						add_css_class: "settings-button",
						set_icon_name: "emblem-system-symbolic",
					}
				}
			}
			target.prepend(&outer_box);
			new_ids.push(outer_box);
		}
		let old_ids = id_handles.replace(new_ids);
		for id in old_ids {
			id.unparent();
		}
	}

	async fn scan_for_devices(tx: Sender<NetworksEvent>) {
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
				if w.request_scan(std::collections::HashMap::new())
					.await
					.is_err()
				{
					eprintln!("Scan failed");
					continue;
				};
				let mut scan_changed = w.receive_last_scan_changed().await;
				while let Some(_t) = scan_changed.next().await {
					println!("scan changed");
					match w.get_access_points().await {
						Ok(aps) => {
							tx.send(NetworksEvent::ApList(AccessPoint::from_list(aps).await))
								.unwrap();
						}
						Err(_) => {
							println!("getting access points failed...");
							continue;
						}
					};
				}
			}
		}
	}
}

enum NetworksEvent {
	ApList(Vec<AccessPoint>),
}

#[derive(Debug)]
struct AccessPoint {
	ssid: String,
	hw_address: String,
	strength: u8,
	wpa_flags: ApSecurityFlags,
}

impl AccessPoint {
	pub async fn new(
		ap: cosmic_dbus_networkmanager::access_point::AccessPoint<'_>,
	) -> Option<Self> {
		Some(Self {
			ssid: ap
				.ssid()
				.await
				.map(|x| String::from_utf8_lossy(&x).into_owned())
				.ok()?,
			hw_address: ap.hw_address().await.ok()?,
			strength: ap.strength().await.ok()?,
			wpa_flags: ap.wpa_flags().await.ok()?,
		})
	}

	pub async fn from_list(
		aps: Vec<cosmic_dbus_networkmanager::access_point::AccessPoint<'_>>,
	) -> Vec<Self> {
		let mut out = Vec::<Self>::with_capacity(aps.len());
		for ap in aps {
			if let Some(ap) = Self::new(ap).await {
				out.push(ap);
			}
		}
		out.sort_by(|a, b| a.strength.cmp(&b.strength));
		out
	}
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
		net_rx.attach(
			None,
			glib::clone!(@weak target, @strong self.access_point_ids as id_handles => @default-return glib::Continue(true), move |event| {
				match event {
					NetworksEvent::ApList(aps) => {
						Self::handle_access_point(target, aps, id_handles.clone());
					}
				};
				glib::Continue(true)
			}),
		);

		target.append(&self.spinner);

		let handle = RT.get().unwrap().handle();
		handle.spawn(Self::scan_for_devices(net_tx));
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
