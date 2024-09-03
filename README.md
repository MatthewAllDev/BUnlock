# BUnlock

- [Desktop Environment Support Status](#desktop-environment-support-status)
- [Installation](#installation)
- [Usage](#usage)
- [Service Management](#service-management)
- [Uninstallation](#uninstallation)
- [License](#license)
- [Authors](#authors)
- [TODO / Future Features](#todo--future-features)


BUnlock is a command-line tool that allows you to unlock your system using a Bluetooth device. The tool manages a systemd service that runs in the background, monitoring the proximity of your paired Bluetooth device and unlocking the system when the device is nearby.

### ⚠️ Security Warning

Using BUnlock to automatically unlock your system based on Bluetooth proximity can introduce potential security risks. If your Bluetooth device is lost, stolen, or if someone mimics its signal, unauthorized access to your system may occur. Ensure that you understand these risks before enabling this feature.

## Desktop Environment Support Status

The following table shows the implementation and testing status of different desktop environments:
| DE        | Support Status     | Tested |
|-----------|--------------------|--------|
| GNOME     | ✅ Implemented      | ✅     |
| KDE       | ✅ Implemented      |        |
| XFCE      | ✅ Implemented      |        |
| MATE      | ✅ Implemented      |        |
| Cinnamon  | ✅ Implemented      |        |
| Pantheon  | ✅ Implemented      |        |
| DDE       | ✅ Implemented      |        |



## Installation

### Prerequisites

- A Bluetooth adapter and a device to pair with.
- GNOME desktop environment (currently, BUnlock supports only GNOME).
- `loginctl` utility (used for unlocking; ensure it's available in your Linux distribution).


### Installation from Release

1. Download the latest release from the [GitHub releases page](https://github.com/MatthewAllDev/bunlock/releases).
   
2. Extract the archive to a directory of your choice:

   ```bash
   tar -xzf bunlock-v0.1.0.tar.gz
   cd bunlock-v0.1.0
   ```

3. Run the installation script:
    ```bash
    ./install.sh
    ```
    This script will copy the bunlock binary to ~/.local/bin, add this directory to your PATH if it’s not already there, and set up the systemd service.

## Usage
### Configuration

To set up BUnlock with your Bluetooth device, run:
```bush
bunlock config
```
This command will guide you through the configuration process. You'll be able to:

+ **Select a Bluetooth Device:** Choose which Bluetooth device will trigger the unlock.
+ **Set the Unlocking Distance:** Define the signal strength (in dB) required to unlock your system.
+ **Save and Exit:** Save your settings and restart the service if it's running.

## Service Management

You can manage the BUnlock systemd service using the following commands:

* **Enable the service:**
    ```bush
    bunlock service enable
    ```
    \* This command enables the service, creates it if it doesn’t already exist, and sets it up to start on boot.

* **Disable the service:**
    ```bush
    bunlock service disable
    ```

* **Start the service:**
    ```bush
    bunlock service start
    ```
* **Stop the service:**
    ```bush
    bunlock service stop
    ```

* **Restart the service:**
    ```bush
    bunlock service restart
    ```

* **Check if the service is active:**
    ```bush
    bunlock service is_active
    ```
    This command returns 0 if the service is active, and 1 if it is not.

## Uninstallation

To uninstall BUnlock, run:
```bash
./uninstall.sh
```
This script will remove the bunlock binary, disable the systemd service, and remove BUnlock's configurations from your system.

## License

BUnlock is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](./LICENSE) file for details.

## Authors

Ilia MatthewAllDev Kuvarzin <luceo2011@yandex.ru>

## TODO / Future Features

- **Enhanced Security:** Ongoing improvements to strengthen security features, including better handling of Bluetooth device impersonation risks.
- **Performance Improvements:** Efforts to optimize the application, reducing resource usage and improving responsiveness.