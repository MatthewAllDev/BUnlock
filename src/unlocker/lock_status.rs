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
use std::time::Duration;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

type LockStatusFn = fn() -> Pin<Box<dyn Future<Output = Result<bool, Box<dyn Error>>>>>;

pub fn get_check_lock_func() -> LockStatusFn {
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match desktop_env.as_str() {
        "GNOME" => || Box::pin(get_lock_status_gnome()),
        "KDE" => || Box::pin(get_lock_status_kde()),
        "XFCE" => || Box::pin(get_lock_status_xfce()),
        "MATE" => || Box::pin(get_lock_status_mate()),
        "Cinnamon" => || Box::pin(get_lock_status_cinnamon()),
        "Pantheon" => || Box::pin(get_lock_status_gnome()),
        "Deepin" => || Box::pin(get_lock_status_dde()),
        _ => {
            println!("Unsupported desktop environment.");
            || Box::pin(async { Err("Unsupported desktop environment".into()) })
        }
    }
}

async fn get_lock_status_gnome() -> Result<bool, Box<dyn Error>> {
    dbus_get_lock_status(
        "org.gnome.ScreenSaver",
        "/org/gnome/ScreenSaver",
        "org.gnome.ScreenSaver",
        "GetActive",
    ).await
}

async fn get_lock_status_kde() -> Result<bool, Box<dyn Error>> {
    dbus_get_lock_status(
        "org.freedesktop.ScreenSaver",
        "/org/freedesktop/ScreenSaver",
        "org.freedesktop.ScreenSaver",
        "GetActive",
    ).await
}

async fn get_lock_status_xfce() -> Result<bool, Box<dyn Error>> {
    dbus_get_lock_status(
        "org.xfce.ScreenSaver",
        "/org/xfce/ScreenSaver",
        "org.xfce.ScreenSaver",
        "GetActive",
    ).await
}

async fn get_lock_status_mate() -> Result<bool, Box<dyn Error>> {
    dbus_get_lock_status(
        "org.mate.ScreenSaver",
        "/org/mate/ScreenSaver",
        "org.mate.ScreenSaver",
        "GetActive",
    ).await
}

async fn get_lock_status_cinnamon() -> Result<bool, Box<dyn Error>> {
    dbus_get_lock_status(
        "org.cinnamon.ScreenSaver",
        "/org/cinnamon/ScreenSaver",
        "org.cinnamon.ScreenSaver",
        "GetActive",
    ).await
}

async fn get_lock_status_dde() -> Result<bool, Box<dyn Error>> {
    dbus_get_lock_status(
        "com.deepin.dde.LockService",
        "/com/deepin/dde/LockService",
        "com.deepin.dde.LockService",
        "GetLockStatus",
    ).await
}

async fn dbus_get_lock_status(dest: &str, path: &str, iface: &str, method: &str) -> Result<bool, Box<dyn Error>> {
    let connection = Connection::new_session()?;
    let msg = Message::new_method_call(dest, path, iface, method)?;
    let reply = connection.send_with_reply_and_block(msg, Duration::from_secs(2))?;
    let is_active: bool = reply.read1()?;
    Ok(is_active)
}