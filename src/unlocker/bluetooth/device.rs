/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use btleplug::api::{Central, Peripheral as _, ScanFilter};
use btleplug::platform::Peripheral;
use log::{debug, error};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::{error::Error, i16};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::{sleep, Duration};
#[path = "bluetooth.rs"]
mod bluetooth;

pub async fn get_all() -> Result<Vec<Device>, Box<dyn Error>> {
    let adapter = bluetooth::start_scan(None, false).await?;
    let mut devices: Vec<Device> = vec![];
    for peripheral in adapter.peripherals().await? {
        let d = Device::from_peripheral(peripheral)
            .await
            .unwrap_or_default();
        devices.push(d);
    }
    devices.sort_by(|a, b| b.rssi.cmp(&a.rssi));
    Ok(devices)
}

#[derive(Clone)]
pub struct Device {
    peripheral: Option<Peripheral>,
    pub id: String,
    pub name: String,
    pub rssi: i16,
}

impl Device {
    pub fn new(
        peripheral: Option<Peripheral>,
        id: String,
        name: String,
        rssi: Option<i16>,
    ) -> Result<Device, Box<dyn Error>> {
        let rssi = rssi.unwrap_or(i16::MIN);
        Ok(Device {
            peripheral,
            id,
            name,
            rssi,
        })
    }

    pub async fn update_peripheral(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("Searching for peripheral with id: {}", self.id);
        let adapter = bluetooth::start_scan(None, true).await?;
        let mut attempt_count = 0;
        let max_attempts_before_delay = 10;
        let mut current_delay = Duration::from_millis(1000);
        let mut sigterm = signal(SignalKind::terminate())?;
        let mut sigint = signal(SignalKind::interrupt())?;
        let mut should_stop = false;
    
        loop {
            tokio::select! {
                _ = sigterm.recv() => {
                    debug!("Received SIGTERM, shutting down...");
                    should_stop = true;
                    return Ok(());
                }
                _ = sigint.recv() => {
                    debug!("Received SIGINT, shutting down...");
                    should_stop = true;
                    return Ok(());
                }
                result = async {
                    match self.search_peripheral(&adapter).await {
                        Ok(Some(peripheral)) => {
                            self.peripheral = Some(peripheral);
                            debug!("Peripheral found and set.");
                            should_stop = true;
                            return Ok(());
                        }
                        Ok(None) => {
                            debug!("Peripheral not found, attempt {}/{}", attempt_count + 1, max_attempts_before_delay);
                            attempt_count += 1;
                            if attempt_count >= max_attempts_before_delay {
                                current_delay = Duration::from_secs(5);
                                debug!("Increasing delay to {:?}", current_delay);
                            }
                            sleep(current_delay).await;
                            Ok(())
                        }
                        Err(e) => {
                            error!("Error searching for peripheral: {}", e);
                            sleep(current_delay).await;
                            Err(e)
                        }
                    }
                } => {
                    if let Err(e) = result {
                        error!("Error during peripheral update: {}", e);
                    }
                }
            }
            if should_stop {
                return Ok(());
            }
        }
    }

    async fn search_peripheral(
        &self,
        adapter: &bluetooth::Adapter,
    ) -> Result<Option<Peripheral>, Box<dyn Error>> {
        match adapter.peripherals().await {
            Ok(peripherals) => {
                for peripheral in peripherals {
                    if self.id == peripheral.address().to_string() {
                        return Ok(Some(peripheral));
                    }
                }
                debug!("Device not found: {}", self.to_string());
            }
            Err(e) => {
                error!("Failed to get peripherals: {}", e);
            }
        }
        Ok(None)
    }

    pub async fn update_rssi(&mut self) -> i16 {
        let rssi: i16 = if let Some(peripheral) = &self.peripheral {
            match peripheral.properties().await {
                Ok(properties) => properties.and_then(|p| p.rssi).unwrap_or(i16::MIN),
                Err(_) => i16::MIN,
            }
        } else {
            self.update_peripheral().await.ok();
            i16::MIN
        };
        self.rssi = rssi;
        return rssi;
    }

    pub async fn from_peripheral(peripheral: Peripheral) -> Result<Device, Box<dyn Error>> {
        let address = peripheral.address().to_string();
        let properties = peripheral.properties().await?;
        let rssi = properties.as_ref().and_then(|p| p.rssi);
        let name = properties
            .as_ref()
            .and_then(|p| p.local_name.clone())
            .unwrap_or("(unknown)".to_string());
        Device::new(Some(peripheral), address, name, rssi)
    }

    pub fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    pub fn serialize<S>(device: &Device, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "id".to_string(),
            serde_json::Value::String(device.id.clone()),
        );
        map.insert(
            "name".to_string(),
            serde_json::Value::String(device.name.clone()),
        );
        map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Device, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DeviceHelper {
            id: String,
            name: String,
        }

        let helper = DeviceHelper::deserialize(deserializer)?;
        let device = Device {
            peripheral: None,
            id: helper.id,
            name: helper.name,
            rssi: i16::MIN,
        };
        Ok(device)
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:?})", self.name, self.id)
    }
}

impl Default for Device {
    fn default() -> Self {
        Device {
            peripheral: None,
            id: String::new(),
            name: String::from("(unknown)"),
            rssi: i16::MIN,
        }
    }
}
