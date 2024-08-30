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
pub use btleplug::platform::{Manager, Adapter};

pub async fn get_adapter()-> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).ok_or("No adapters found")?;
    Ok(central)
}

pub async fn start_scan(adapter: Option<Adapter>) -> Result<Adapter, Box<dyn Error>> {
    let adapter = match adapter {
        Some(ad) => ad,
        None => {
            get_adapter().await?
        }
    };
    adapter.start_scan(ScanFilter::default()).await?;
    Ok(adapter)
}