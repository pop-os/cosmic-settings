// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui, RT};
use cosmic_dbus_networkmanager::{
	device::SpecificDevice, interface::enums::ApSecurityFlags, nm::NetworkManager,
};
use futures::StreamExt;
use gtk4::{glib, prelude::*, Align, Button, Image, Label, Orientation, Spinner};
use slotmap::{DefaultKey, SlotMap};
use std::rc::Rc;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};
use tokio::sync::mpsc::UnboundedSender;
use zbus::Connection;

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
	fn handle_access_point(
		target: &gtk4::Box,
		tx: &UnboundedSender<NetworksEvent>,
		aps: &SlotMap<DefaultKey, AccessPoint>,
	) {
		dbg!(aps.len());

		while let Some(widget) = target.first_child().as_ref() {
			target.remove(widget);
		}

		for (id, ap) in aps.iter() {
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

			connect_button.connect_clicked(glib::clone!(@strong tx => move |_| {
				let _ = tx.send(NetworksEvent::ConfigureDevice(id));
			}));

			settings_button.connect_clicked(glib::clone!(@strong tx => move |_| {
				let _ = tx.send(NetworksEvent::ConfigureDevice(id));
			}));

			target.prepend(&outer_box);
		}
	}

	async fn scan_for_devices(tx: UnboundedSender<NetworksEvent>) {
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
		let mut all_aps = SlotMap::new();

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
							eprintln!("getting access points failed...");
							continue;
						}
					}
					eprintln!("scan changed");
					match w.get_access_points().await {
						Ok(aps) => {
							if !aps.is_empty() {
								for ap in AccessPoint::from_list(aps).await {
									all_aps.insert(ap);
								}

								break;
							}
						}
						Err(_) => {
							eprintln!("getting access points failed...");
							continue;
						}
					};
				}
			}
		}

		if let Err(why) = tx.send(NetworksEvent::ApList(all_aps)) {
			eprintln!("{}:{}: {:?}", file!(), line!(), why);
		}
	}
}

#[derive(Debug)]
enum NetworksEvent {
	ApList(SlotMap<DefaultKey, AccessPoint>),
	ConfigureDevice(DefaultKey),
	Quit,
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
		dbg!(&target);
		target.append(&self.spinner);

		let (net_tx, mut net_rx) = tokio::sync::mpsc::unbounded_channel();

		target.connect_destroy(glib::clone!(@strong net_tx => move |_| {
			let _ = net_tx.send(NetworksEvent::Quit);
		}));

		let cancel = Arc::new(AtomicBool::new(false));

		RT.get().unwrap().spawn({
			let cancel = cancel.clone();
			let tx = net_tx.clone();
			async move {
				loop {
					if cancel.load(Ordering::Relaxed) {
						eprintln!("stopped network scanning");
						return;
					}

					Self::scan_for_devices(tx.clone()).await;
					tokio::time::sleep(std::time::Duration::from_secs(5)).await;
				}
			}
		});

		let target = target.downgrade();
		crate::task::spawn_local(async move {
			let mut aps = SlotMap::new();

			while let Some(event) = net_rx.recv().await {
				match event {
					NetworksEvent::ApList(update) => {
						if let Some(target) = target.upgrade() {
							Self::handle_access_point(&target, &net_tx, &update);
						}

						aps = update;
					}

					NetworksEvent::ConfigureDevice(device) => {
						if let Some(ap) = aps.get(device) {
							eprintln!("configuring {:?}", ap);

							let dialog = gtk4::MessageDialogBuilder::new()
								.buttons(gtk4::ButtonsType::OkCancel)
								.text(&format!("TODO {}", ap.ssid))
								.build();

							crate::task::spawn_local(async move {
								dialog.run_future().await;
								dialog.close();
							});
						}
					}

					NetworksEvent::Quit => {
						eprintln!("stop network scanning");
						cancel.store(true, Ordering::SeqCst);
						break;
					}
				}
			}
		});
	}
}
