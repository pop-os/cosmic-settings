// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry, RT};
use cosmic_dbus_networkmanager::{
	device::{wireless::WirelessDevice, SpecificDevice},
	interface::settings::{connection::ConnectionSettingsProxy, SettingsProxy},
	nm::NetworkManager,
};
use futures::stream::{self, StreamExt};
use gtk4::{glib, prelude::*, Align, Button, Image, Label, Orientation, Switch};
use once_cell::sync::OnceCell;
use std::{iter::IntoIterator, rc::Rc};
use zbus::{Connection, Error, Result};
use zvariant::Str;
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
struct VisibleNetworks;

impl VisibleNetworks {
	async fn network_connections_as_id<I>(
		connections: I,
		sys_conn_cell: &OnceCell<Connection>,
	) -> Vec<String>
	where
		I: IntoIterator<Item = zbus::zvariant::OwnedObjectPath>,
	{
		stream::iter(connections)
			.then(
				glib::clone!(@strong sys_conn_cell => @default-return Ok(String::new()), move |conn_path|
					{
						let sys_conn_cell = sys_conn_cell.clone();
						async move {
							let sys_conn = sys_conn_cell.get().unwrap();
							let conn_proxy = ConnectionSettingsProxy::builder(&sys_conn)
								.path(conn_path)?
								.build()
								.await?;
							let settings = conn_proxy.get_settings().await?;
							let id: String = String::from(
								settings
									.get("connection")
									.ok_or(Error::InvalidReply)?
									.get("id")
									.ok_or(Error::InvalidReply)?
									.downcast_ref::<Str>()
									.ok_or(Error::InvalidReply)?
									.as_str(),
							);
							Ok(id)
						}
					}
				),
			)
			.collect::<Vec<Result<String>>>()
			.await
			.into_iter()
			.filter_map(|x| x.ok())
			.collect::<Vec<String>>()
	}
}

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

		net_rx.attach(
			None,
			glib::clone!(@weak target => @default-return glib::Continue(true), move |event| {
				match event {
					NetworksEvent::IdList(ids) => {
						for network in ids {
							let inner_box = gtk4::Box::new(Orientation::Horizontal, 16);
							let icon = Image::from_icon_name(Some("network-wireless-symbolic"));
							let label = Label::new(Some(&network));
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
						}

					}
				};
				glib::Continue(false)
			}),
		);

		let handle = RT.get().unwrap().handle();
		handle.spawn(async move {
			let sys_conn = match Connection::system().await {
				Ok(conn) => conn,
				Err(_) => return, //TODO err msg
			};
			// let sys_conn_cell = OnceCell::new();
			// sys_conn_cell.set(sys_conn).unwrap();
			// let sys_conn = sys_conn_cell.get().unwrap();
			// let nm_proxy = match SettingsProxy::new(sys_conn).await {
			// 	Ok(p) => p,
			// 	Err(_) => return, //TODO err msg
			// };
			// let known_connections = Self::network_connections_as_id(conns, &sys_conn_cell).await;
			// let _ = net_tx.send(NetworksEvent::IdList(known_connections));

			// let mut conns_changed = nm_proxy.receive_connections_changed().await;
			// while let Some(conns) = conns_changed.next().await {
			// 	let conns = match conns.get().await {
			// 		Ok(c) => c,
			// 		Err(_) => continue,
			// 	};
			// 	let networks = Self::network_connections_as_id(conns, &sys_conn_cell).await;
			// 	let _ = net_tx.send(NetworksEvent::IdList(networks));
			// }

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
					match w.get_all_access_points().await {
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

							let _ = net_tx.send(NetworksEvent::IdList(ssids));
						}
						Err(_) => continue,
					};
					todo!();
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
