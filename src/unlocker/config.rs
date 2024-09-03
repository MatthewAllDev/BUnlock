/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error as StdError;
use std::fs::{File, Permissions, create_dir_all, set_permissions};
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
pub use crate::unlocker::bluetooth;
pub use crate::unlocker::service;

const CONFIG_PATH: &str = "~/.config/bunlock/config.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(
        serialize_with = "bluetooth::device::Device::serialize",
        deserialize_with = "bluetooth::device::Device::deserialize"
    )]
    pub device: bluetooth::device::Device,
    pub distance: i16,
    pub delay_seconds: u32,
}

impl Config {
    pub async fn new() -> Result<Self, Box<dyn StdError>> {
        let path = CONFIG_PATH.replace("~", &service::get_home_dir());
        if Path::new(&path).exists() {
            return Config::load_from_file(path).await;
        } else {
            Ok(Config::default())
        }
    }

    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn StdError>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config =
            serde_json::from_str(&contents).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
        Ok(config)
    }

    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let path = path.as_ref();
    
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
    
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        let permissions = Permissions::from_mode(0o600);
        set_permissions(path, permissions)?;
        Ok(())
    }

    pub async fn configurate(&mut self) -> Result<(), Box<dyn StdError>> {
        let theme = ColorfulTheme::default();
        let mut selected_index: usize = 0;
        loop {
            let menu_items = vec![
                format!("Select Bluetooth device (current: {})", self.device.name),
                format!("Distance for unlocking (current: {} dB)", self.distance),
                "Save and Exit".to_string(),
            ];

            selected_index = Select::with_theme(&theme)
                .with_prompt("Main menu")
                .default(selected_index)
                .items(&menu_items[..])
                .interact()?;

            match selected_index {
                0 => {
                    let devices: Vec<bluetooth::device::Device> =
                        bluetooth::device::get_all().await?;
                    let mut device_names: Vec<String> = vec![];
                    let mut default = 0;
                    for (index, device) in devices.iter().enumerate() {
                        device_names.push(device.to_string());
                        if device.id == self.device.id {
                            default = index;
                        }
                    }
                    let selected_device_index = Select::with_theme(&theme)
                        .with_prompt("Select Bluetooth device for unlocking")
                        .default(default)
                        .items(&device_names[..])
                        .interact_opt()?;
                    if let Some(index) = selected_device_index {
                        self.device = devices[index].clone();
                    }
                }
                1 => {
                    let device = &mut self.device;
                    device.update_rssi().await;
                    let rssi = if device.rssi.eq(&i16::MIN) {
                        "Device not found".to_string()
                    } else {
                        device.rssi.to_string()
                    };
                    let distance_input: String = Input::with_theme(&theme)
                        .with_prompt(format!("Current distance: {}\nEnter distance for unlocking (in dB)", rssi))
                        .default(self.distance.to_string())
                        .interact_text()?;
                    self.distance = distance_input.trim().parse().unwrap_or(0);
                }
                2 => {
                    let path = CONFIG_PATH.replace("~", &service::get_home_dir());
                    self.save_to_file(path).await?;
                    if service::is_running()? {
                        service::restart()?;
                    }
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
