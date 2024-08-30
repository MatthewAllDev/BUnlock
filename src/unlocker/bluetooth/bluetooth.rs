/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use std::error::Error;
use btleplug::api::Manager as _;
use btleplug::platform::{Manager, Adapter};

pub async fn get_adapter()-> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).ok_or("No adapters found")?;
    Ok(central)
}