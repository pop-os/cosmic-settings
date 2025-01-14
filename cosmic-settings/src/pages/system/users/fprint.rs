// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{process::Output, str::FromStr};

use fprint_zbus::{FprintDeviceProxy, FprintManagerProxy};
use zbus::Connection;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FingerName {
    Any,
    LeftThumb,
    LeftIndexFinger,
    LeftMiddleFinger,
    LeftRingFinger,
    LeftLittleFinger,
    RightThumb,
    RightIndexFinger,
    RightMiddleFinger,
    RightRingFinger,
    RightLittleFinger,
}

impl FingerName {
    pub const ALL: &[FingerName] = &[
        Self::LeftThumb,
        Self::LeftIndexFinger,
        Self::LeftMiddleFinger,
        Self::LeftRingFinger,
        Self::LeftLittleFinger,
        Self::RightThumb,
        Self::RightIndexFinger,
        Self::RightMiddleFinger,
        Self::RightRingFinger,
        Self::RightLittleFinger,
    ];
}

impl FromStr for FingerName {
    type Err = ();

    fn from_str(finger: &str) -> Result<Self, Self::Err> {
        match finger.as_ref() {
            "any" => Ok(Self::Any),
            "left-thumb" => Ok(Self::LeftThumb),
            "left-index-finger" => Ok(Self::LeftIndexFinger),
            "left-middle-finger" => Ok(Self::LeftMiddleFinger),
            "left-ring-finger" => Ok(Self::LeftRingFinger),
            "left-little-finger" => Ok(Self::LeftLittleFinger),
            "right-thumb" => Ok(Self::RightThumb),
            "right-index-finger" => Ok(Self::RightIndexFinger),
            "right-middle-finger" => Ok(Self::RightMiddleFinger),
            "right-ring-finger" => Ok(Self::RightRingFinger),
            "right-little-finger" => Ok(Self::RightLittleFinger),
            other => Err(()),
        }
    }
}

impl ToString for FingerName {
    fn to_string(&self) -> String {
        match self {
            Self::Any => fl!("finger", "any"),
            Self::LeftThumb => fl!("finger", "left-thumb"),
            Self::LeftIndexFinger => fl!("finger", "left-index-finger"),
            Self::LeftMiddleFinger => fl!("finger", "left-middle-finger"),
            Self::LeftRingFinger => fl!("finger", "left-ring-finger"),
            Self::LeftLittleFinger => fl!("finger", "left-little-finger"),
            Self::RightThumb => fl!("finger", "right-thumb"),
            Self::RightIndexFinger => fl!("finger", "right-index-finger"),
            Self::RightMiddleFinger => fl!("finger", "right-middle-finger"),
            Self::RightRingFinger => fl!("finger", "right-ring-finger"),
            Self::RightLittleFinger => fl!("finger", "right-little-finger"),
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Default)]
pub struct FprintDeviceInfo {
    pub name: String,
    pub path: zbus::zvariant::OwnedObjectPath,
    pub enrolled_fingers: Vec<FingerName>,
    pub ongoing_enroll: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FprintInfo {
    pub default_device: Option<FprintDeviceInfo>,
    pub other_devices: Vec<FprintDeviceInfo>,
}

pub async fn start_enroll_finger(
    device: zbus::zvariant::OwnedObjectPath,
    username: &str,
) -> Result<(), ()> {
    let daemon = get_fprint_device_proxy(&device).await?;

    if let Err(e) = daemon.claim(username).await {
        tracing::error!("{e}");
        return Err(());
    }

    Ok(())
}

pub async fn stop_enroll_finger(
    device: zbus::zvariant::OwnedObjectPath,
    username: &str,
) -> Result<(), ()> {
    let daemon = get_fprint_device_proxy(&device).await?;

    if let Err(e) = daemon.release().await {
        tracing::error!("{e}");
        return Err(());
    }

    Ok(())
}

async fn get_fprint_device_info(
    object_path: zbus::zvariant::OwnedObjectPath,
    username: &str,
) -> Result<FprintDeviceInfo, ()> {
    let daemon = get_fprint_device_proxy(&object_path).await?;
    let name = match daemon.name().await {
        Ok(name) => name,
        Err(e) => {
            tracing::error!("{e}");
            return Err(());
        }
    };

    let fingers = match daemon.list_enrolled_fingers(username).await {
        Ok(fingers) => {
            let mut fingers_vec = Vec::new();
            for finger in fingers {
                fingers_vec.push(match finger.parse() {
                    Ok(FingerName::Any) | Err(_) => {
                        tracing::error!("Received unexpected finger name: {finger}");
                        return Err(());
                    }
                    Ok(finger) => finger,
                });
            }
            fingers_vec
        }
        Err(e) => match e.to_string().as_str() {
            "net.reactivated.Fprint.Error.NoEnrolledPrints: Failed to discover prints" => {
                Vec::new()
            }
            _ => {
                tracing::error!("{e}");
                return Err(());
            }
        },
    };

    Ok(FprintDeviceInfo {
        name,
        path: object_path,
        enrolled_fingers: fingers,
        ongoing_enroll: false,
    })
}

pub async fn get_fprint_info(username: &str) -> Result<FprintInfo, ()> {
    let daemon = get_fprint_manager_proxy().await?;

    let default_device_path = match daemon.get_default_device().await {
        Ok(device) => Some(device),
        Err(zbus::Error::MethodError(_, _, _)) => None,
        Err(e) => {
            tracing::error!("{e}");
            return Err(());
        }
    };

    let default_device_info = match default_device_path {
        Some(ref path) => Some(get_fprint_device_info(path.clone(), username).await?),
        None => None,
    };

    tracing::info!("Default device: {:?}", default_device_info);

    let other_devices = match daemon.get_devices().await {
        Ok(mut devices) => {
            if let Some(default_device) = default_device_path {
                devices.retain(|device| device != &default_device);
            }
            let mut devices_info = Vec::new();
            for device in devices {
                devices_info.push(get_fprint_device_info(device, username).await?)
            }
            devices_info
        }
        Err(e) => {
            tracing::error!("{e}");
            return Err(());
        }
    };

    tracing::info!("Other devices: {:?}", other_devices);

    Ok(FprintInfo {
        default_device: default_device_info,
        other_devices,
    })
}

async fn get_fprint_manager_proxy<'a>() -> Result<FprintManagerProxy<'a>, ()> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(());
        }
    };

    match FprintManagerProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!("Fprint daemon proxy can't be created. Is it installed? {e}");
            Err(())
        }
    }
}

async fn get_fprint_device_proxy<'a>(
    device_object_path: &'a zbus::zvariant::OwnedObjectPath,
) -> Result<FprintDeviceProxy<'a>, ()> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(());
        }
    };

    let proxy_builder = match FprintDeviceProxy::builder(&connection).path(device_object_path) {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("{e}");
            return Err(());
        }
    };

    match proxy_builder.build().await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!("Fprint daemon proxy can't be created. Is it installed? {e}");
            Err(())
        }
    }
}
