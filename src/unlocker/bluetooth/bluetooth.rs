/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use std::error::Error;
use btleplug::api::{Central, ScanFilter, Manager as _};
use tokio::time::{sleep, Duration};
pub use btleplug::platform::{Manager, Adapter};
use log::{info, error};

pub async fn get_adapter()-> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).ok_or("No adapters found")?;
    Ok(central)
}

pub async fn start_scan(adapter: Option<Adapter>, wait_for_adapter: bool) -> Result<Adapter, Box<dyn Error>> {
    let adapter = match adapter {
        Some(ad) => ad,
        None => {
            get_adapter().await?
        }
    };
    loop {
        match adapter.start_scan(ScanFilter::default()).await {
            Ok(_) => return Ok(adapter),
            Err(e) => {
                if wait_for_adapter {
                    info!("Adapter not ready for scanning, waiting...");
                    sleep(Duration::from_secs(5)).await;
                } else {
                    error!("Failed to start scanning: {}", e);
                    return Err(e.into());
                }
            }
        }
    }
}