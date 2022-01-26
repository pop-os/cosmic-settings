// SPDX-License-Identifier: GPL-3.0-only

use cosmic_dbus_networkmanager::{
	device::{Device, SpecificDevice},
	interface::enums::ApSecurityFlags,
	nm::NetworkManager,
};
use futures::StreamExt;
use itertools::Itertools;
use slotmap::{DefaultKey, SlotMap};
use std::sync::Arc;
use tokio::sync::{
	mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	RwLock,
};
use zbus::Connection;

#[derive(Debug, Clone)]
pub enum NetworkManagerMessage {}

pub enum NetworkManagerEvent {
	Aps(SlotMap<DefaultKey, AccessPoint>),
}

pub struct NetworkManagerService {
	rx: UnboundedReceiver<NetworkManagerMessage>,
	pub(crate) tx: Arc<UnboundedSender<NetworkManagerMessage>>,
	pub(crate) event_tx: Arc<RwLock<Vec<UnboundedSender<NetworkManagerEvent>>>>,
}

impl NetworkManagerService {
	pub fn new() -> Self {
		let (tx, rx) = unbounded_channel();
		Self {
			rx,
			tx: Arc::new(tx),
			event_tx: Arc::default(),
		}
	}

	pub async fn run(mut self) {
		tokio::spawn(Self::scan_for_networks(self.event_tx.clone()));
		while let Some(request) = self.rx.recv().await {
			// TODO
		}
	}

	async fn scan_for_networks(event_tx: Arc<RwLock<Vec<UnboundedSender<NetworkManagerEvent>>>>) {
		let sys_conn = match Connection::system().await {
			Ok(conn) => conn,
			Err(err) => {
				error!(%err, "Failed to connect to system bus");
				return;
			}
		};
		let nm = match NetworkManager::new(&sys_conn).await {
			Ok(p) => p,
			Err(err) => {
				error!(%err, "Failed to create NetworkManager proxy");
				return;
			}
		};
		loop {
			let devices = match nm.devices().await {
				Ok(d) => d,
				Err(err) => {
					error!(%err, "Failed to fetch devices from NetworkManager");
					return;
				}
			};
			let mut all_aps = SlotMap::new();
			for device in devices {
				Self::handle_wireless_device(device, &mut all_aps, event_tx.clone()).await;
			}
			let event_handlers = event_tx.read().await;
			for event in event_handlers.iter() {
				if let Err(err) = event.send(NetworkManagerEvent::Aps(all_aps.clone())) {
					error!(%err, file = %file!(), line = %line!(), "Failed to send APs to event handler");
				}
			}
			tokio::time::sleep(std::time::Duration::from_secs(5)).await;
		}
	}

	async fn handle_wireless_device<'a>(
		device: Device<'a>,
		all_aps: &mut SlotMap<DefaultKey, AccessPoint>,
		event_tx: Arc<RwLock<Vec<UnboundedSender<NetworkManagerEvent>>>>,
	) {
		let wireless = match device.downcast_to_device().await {
			Ok(Some(SpecificDevice::Wireless(w))) => w,
			_ => return,
		};
		if let Err(err) = wireless
			.request_scan(std::collections::HashMap::new())
			.await
		{
			error!(%err, "Failed to request scan");
			return;
		}
		let mut scan_changed = wireless.receive_last_scan_changed().await;
		if let Some(changed) = scan_changed.next().await {
			match changed.get().await {
				Ok(-1) => {
					error!("Failed to get scan results");
					return;
				}
				Ok(_) => (),
				Err(err) => {
					error!(%err, "Failed to get scan results");
					return;
				}
			}
			match wireless.get_access_points().await {
				Ok(aps) => {
					if !aps.is_empty() {
						for ap in AccessPoint::from_list(aps).await {
							all_aps.insert(ap);
						}
					}
				}
				Err(err) => {
					error!(%err, "Failed to get access points");
					return;
				}
			}
		}
	}
}

#[derive(Debug, Clone)]
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
		let mut ret = out
			.into_iter()
			.sorted_by(|a, b| a.strength.cmp(&b.strength))
			.rev()
			.unique_by(|ap| ap.ssid.clone())
			.collect::<Vec<Self>>();
		// for some reason adding .rev() messes up unique_by, so we do this instead
		ret.reverse();
		ret
	}
}
