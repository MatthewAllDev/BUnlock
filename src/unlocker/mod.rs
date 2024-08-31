/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use dbus::blocking::{BlockingSender, Connection};
use dbus::message::Message;
use log::{debug, error, info};
use std::error::Error;
use std::time::{Duration, SystemTime};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;
mod bluetooth;
pub mod config;
pub mod service;

async fn get_lock_status() -> Result<bool, Box<dyn Error>> {
    let connection = Connection::new_session()?;
    let msg = Message::new_method_call(
        "org.gnome.ScreenSaver",
        "/org/gnome/ScreenSaver",
        "org.gnome.ScreenSaver",
        "GetActive",
    )?;
    let reply = connection.send_with_reply_and_block(msg, Duration::from_secs(2))?;
    let is_active: bool = reply.read1()?;
    Ok(is_active)
}

pub async fn start_daemon(config_data: &config::Config) -> Result<(), Box<dyn Error>> {
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
