use std::rc::Rc;

use cosmic_dbus_networkmanager::{
	device::SpecificDevice, interface::enums::ApSecurityFlags, nm::NetworkManager,
};
use futures::StreamExt;
use gtk4::{
	glib::{self, MainContext, Sender},
	prelude::*,
	Align, Button, Image, Label, Orientation, Spinner,
};
use zbus::Connection;

use crate::{sections::SettingsGroup, ui::SettingsGui, RT};

// SPDX-License-Identifier: GPL-3.0-only

pub struct VisibleNetworks {
	spinner: Spinner,
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
		Self { spinner }
	}
}

impl VisibleNetworks {
	fn handle_access_point(target: gtk4::Box, aps: Vec<AccessPoint>) {
		dbg!(aps.len());
		while let Some(widget) = target.first_child().as_ref() {
			target.remove(widget);
		}
		for ap in aps {
			// dbg!(&ap);
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
		let mut all_aps = vec![];
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
				if let Some(t) = scan_changed.next().await {
					if let Ok(t) = t.get().await {
						if t == -1 {
							println!("getting access points failed...");
							continue;
						}
					}
					println!("scan changed");
					match w.get_access_points().await {
						Ok(aps) => {
							if !aps.is_empty() {
								let mut aps = AccessPoint::from_list(aps).await;
								all_aps.append(&mut aps);
								break;
							}
						}
						Err(_) => {
							println!("getting access points failed...");
							continue;
						}
					};
				}
			}
		}
		tx.send(NetworksEvent::ApList(all_aps)).unwrap();
	}
}

enum NetworksEvent {
	ApList(Vec<AccessPoint>),
}

#[derive(Debug)]
pub struct AccessPoint {
	pub ssid: String,
	pub hw_address: String,
	pub strength: u8,
	pub wpa_flags: ApSecurityFlags,
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

impl SettingsGroup for VisibleNetworks {
	fn title(&self) -> &'static str {
		"Visible Networks"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "connect", "ssid"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let (net_tx, net_rx) = MainContext::channel::<NetworksEvent>(glib::PRIORITY_HIGH);
		net_rx.attach(
			None,
			glib::clone!(@weak target => @default-return glib::Continue(true), move |event| {
				match event {
					NetworksEvent::ApList(aps) => {
						Self::handle_access_point(target, aps);
					}
				};
				glib::Continue(true)
			}),
		);

		dbg!(&target);
		target.append(&self.spinner);

		let handle = RT.get().unwrap().handle();
		handle.spawn(async move {
			loop {
				Self::scan_for_devices(net_tx.clone()).await;
				tokio::time::sleep(std::time::Duration::from_secs(5)).await;
			}
		});
	}
}
