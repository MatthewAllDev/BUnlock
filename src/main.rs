/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use clap::Command;
use std::error::Error;
mod unlocker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("bunlock")
        .version("0.1.0")
        .author("Ilia MatthewAllDev Kuvarzin <luceo2011@yandex.ru>")
        .about("A tool for unlocking your system using a Bluetooth device")
        .subcommand(Command::new("config").about("Runs the configuration setup"))
        .subcommand(
            Command::new("service")
                .about("Manage the systemd service")
                .subcommand(Command::new("enable").about("Enable and create the service if it doesn't exist"))
                .subcommand(Command::new("disable").about("Disable the service"))
                .subcommand(Command::new("start").about("Start the service"))
                .subcommand(Command::new("stop").about("Stop the service"))
                .subcommand(Command::new("restart").about("Restart the service"))
                .subcommand(Command::new("is_active").about("Check if the service is active"))
                .subcommand(Command::new("remove").about("Remove the service")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("config", _)) => {
            let mut config_data = unlocker::config::Config::new().await?;
            config_data.configurate().await?;
        }
        Some(("service", service_matches)) => {
            match service_matches.subcommand() {
                Some(("enable", _)) => unlocker::service::enable()?,
                Some(("disable", _)) => unlocker::service::disable()?,
                Some(("start", _)) => unlocker::service::start()?,
                Some(("stop", _)) => unlocker::service::stop()?,
                Some(("restart", _)) => unlocker::service::restart()?,
                Some(("is_active", _)) => {
                    match unlocker::service::is_running() {
                        Ok(true) => {
                            println!("The service is active.");
                            std::process::exit(0)
                        },
                        Ok(false) => {
                            println!("The service is not active.");
                            std::process::exit(1)
                        },
                        Err(e) => {
                            eprintln!("Error checking service status: {}", e);
                            std::process::exit(1)
                        },
                    }
                },
                Some(("remove", _)) => unlocker::service::remove_service()?,
                _ => eprintln!("Unknown service command"),
            }
        }
        _ => {
            env_logger::init();
            let config_data = unlocker::config::Config::new().await?;
            unlocker::start_daemon(&config_data).await?;
        }
    }

    Ok(())
}
