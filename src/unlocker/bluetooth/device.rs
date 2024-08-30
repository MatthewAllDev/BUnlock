/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use std::{error::Error, i16};
use tokio::time::sleep;
use btleplug::platform::Peripheral;
use btleplug::api::{Central, ScanFilter, Peripheral as _};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
#[path = "bluetooth.rs"] mod bluetooth;

pub async fn get_all() -> Result<Vec<Device>, Box<dyn Error>> {
    let adapter = bluetooth::get_adapter().await?;
    adapter.start_scan(ScanFilter::default()).await?;
    sleep(std::time::Duration::from_secs(2)).await;
    let mut devices: Vec<Device> = vec![];
    for peripheral in adapter.peripherals().await? {
        let d = Device::from_peripheral(peripheral).await.unwrap_or_default();
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
    pub rssi: i16
}

impl Device {
    pub fn new(peripheral: Option<Peripheral>, id: String, name: String, rssi: Option<i16>) -> Result<Device, Box<dyn Error>> {
        let rssi = rssi.unwrap_or(i16::MIN);
        Ok(Device { peripheral, id, name,  rssi})
    }

    pub async fn search_peripheral(&mut self) -> Result<(), Box<dyn Error>> {
        let adapter = bluetooth::get_adapter().await?;
        adapter.start_scan(ScanFilter::default()).await?;
        let peripherals = adapter.peripherals().await?;
        for peripheral in peripherals {
            if self.id == peripheral.address().to_string() {
                self.peripheral = Some(peripheral);
                return Ok(());
            }
        }
        self.peripheral = None;
        Ok(())
    }
    
    pub async fn update_rssi(&mut self) -> i16 {
        let rssi: i16 = if let Some(peripheral) = &self.peripheral {
            match peripheral.properties().await {
                Ok(properties) => properties
                    .and_then(|p| p.rssi)
                    .unwrap_or(i16::MIN),
                Err(_) => i16::MIN,
            }
        } else {
            self.search_peripheral().await.ok();
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
        map.insert("id".to_string(), serde_json::Value::String(device.id.clone()));
        map.insert("name".to_string(), serde_json::Value::String(device.name.clone()));
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
        write!(
            f,
            "{:?} ({:?})",
            self.name, self.id
        )
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
