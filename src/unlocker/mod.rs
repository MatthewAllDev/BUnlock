/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use log::{debug, error, info};
use std::error::Error;
use std::time::{Duration, SystemTime};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;
pub mod bluetooth;
pub mod config;
pub mod service;
pub mod lock_status;


pub async fn start_daemon(config_data: &config::Config) -> Result<(), Box<dyn Error>> {
    let get_lock_status = lock_status::get_check_lock_func();
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut device = config_data.device.clone();
    let mut last_check = SystemTime::now();
    let timeout_duration = Duration::from_secs(2);
    info!("Daemon started");
    device.update_peripheral().await?;
    loop {
        tokio::select! {
            _ = sigterm.recv() => {
                debug!("Received SIGTERM, shutting down...");
                break;
            }
            _ = sigint.recv() => {
                debug!("Received SIGINT, shutting down...");
                break;
            }
            _ = async {
                let now = SystemTime::now();
                let elapsed = now.duration_since(last_check).unwrap_or(Duration::from_secs(0));
                if elapsed > timeout_duration * 2 {
                    debug!("Detected system suspend or significant delay, re-initiating Bluetooth device search.");
                    if let Err(e) = bluetooth::start_scan(None, true).await {
                        error!("Failed to get Bluetooth adapter: {}", e);
                    }
                }
                let locked = get_lock_status().await.unwrap_or(false);
                if locked {
                    let rssi = device.update_rssi().await;
                    if config_data.distance <= rssi {
                        if let Err(e) = std::process::Command::new("loginctl")
                        .arg("unlock-session")
                        .output()
                        {
                            error!("Failed to unlock session: {}", e);
                        } else {
                            info!("System unlocked wtih {}", device.name);
                        }
                    } else {
                        debug!("RSSI ({}) does not meet the unlocking criteria", rssi);
                    }
                }
                last_check = now;
                sleep(timeout_duration).await;
            } => {}
        }
    }
    info!("Daemon shutting down");
    Ok(())
}
