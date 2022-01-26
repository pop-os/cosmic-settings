use tokio::sync::mpsc::UnboundedSender;

// SPDX-License-Identifier: GPL-3.0-only

pub mod nm;

#[derive(Debug, Clone)]
pub enum ServiceMessage {}

pub enum ServiceHandler {
	NetworkManager(UnboundedSender<nm::NetworkManagerEvent>),
}
