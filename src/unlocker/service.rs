/*
 * Copyright (c) 2024 Ilia MatthewAllDev Kuvarzin
 *
 * This file is part of the BUnlock project.
 *
 * BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0).
 * See the LICENSE file for details.
 */

use home::home_dir;
use std::error::Error as StdError;
use std::process::Command;

const SERVICE_NAME: &str = "bunlock.service";

pub fn get_home_dir() -> String {
    let home_dir = home_dir().expect("Failed to get home directory");
    home_dir
        .to_str()
        .expect("Failed to convert home_dir to str")
        .to_string()
}

pub fn create_service() -> Result<(), Box<dyn StdError>> {
    let service_file_path = format!("~/.config/systemd/user/{}", SERVICE_NAME);
    let bin_file_path = "~/.local/bin/bunlock/bunlock".replace("~", &get_home_dir());
    let service_file_content = format!(
        "[Unit]
Description=BUnlock Bluetooth Unlocker Daemon
After=network.target

[Service]
ExecStart={}
Environment=RUST_LOG=info
StandardOutput=journal
StandardError=journal
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
",
        bin_file_path
    );

    let expanded_path = service_file_path.replace("~", &get_home_dir());
    println!("{}", expanded_path);
    std::fs::write(&expanded_path, service_file_content)?;

    reload_systemd_configuration()?;

    Ok(())
}

pub fn remove_service() -> Result<(), Box<dyn StdError>> {
    if !exists()? {
        return Err("Service does not exist".into());
    }
    if is_running()? {
        stop()?;
    }
    disable()?;
    let service_file_path = format!("~/.config/systemd/user/{}", SERVICE_NAME);
    let expanded_path = service_file_path.replace("~", &get_home_dir());
    std::fs::remove_file(&expanded_path)?;
    reload_systemd_configuration()?;
    Ok(())

}

fn reload_systemd_configuration() -> Result<(), Box<dyn StdError>> {
    let status = Command::new("systemctl")
        .arg("--user")
        .arg("daemon-reload")
        .status()?;

    if !status.success() {
        return Err("Failed to reload systemd configuration".into());
    }

    Ok(())
}

pub fn exists() -> Result<bool, Box<dyn StdError>> {
    let output = Command::new("systemctl")
        .arg("--user")
        .arg("list-unit-files")
        .output()?;

    if !output.status.success() {
        return Err("Failed to list unit files".into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.contains(SERVICE_NAME))
}

pub fn is_running() -> Result<bool, Box<dyn StdError>> {
    let status_output = std::process::Command::new("systemctl")
        .arg("--user")
        .arg("is-active")
        .arg(SERVICE_NAME)
        .output()?;
    let output_str = String::from_utf8_lossy(&status_output.stdout).trim().to_string();
    Ok(output_str == "active")
}

pub fn enable() -> Result<(), Box<dyn StdError>> {
    let service_exists = exists()?;
    if !service_exists {
        create_service()?;
    }
    service_command("enable")
}

pub fn disable() -> Result<(), Box<dyn StdError>> {
    if !exists()? {
        return Err("Service does not exist, cannot disable".into());
    }

    let running = is_running()?;
    if running {
        return Err("Service is running. Please stop it before disabling".into());
    }

    service_command("disable")
}

pub fn start() -> Result<(), Box<dyn StdError>> {
    if !exists()? {
        return Err("Service does not exist, cannot start".into());
    }

    let running = is_running()?;
    if running {
        return Err("Service is already running".into());
    }

    service_command("start")
}

pub fn stop() -> Result<(), Box<dyn StdError>> {
    if !exists()? {
        return Err("Service does not exist, cannot stop".into());
    }

    let running = is_running()?;
    if !running {
        return Err("Service is not running".into());
    }

    service_command("stop")
}

pub fn restart() -> Result<(), Box<dyn StdError>> {
    if !exists()? {
        return Err("Service does not exist, cannot restart".into());
    }

    service_command("restart")
}

fn service_command(command: &str) -> Result<(), Box<dyn StdError>> {
    let status = Command::new("systemctl")
        .arg("--user")
        .arg(command)
        .arg(SERVICE_NAME)
        .status()?;

    if !status.success() {
        return Err(format!("Failed to {} service", command).into());
    }
    Ok(())
}
