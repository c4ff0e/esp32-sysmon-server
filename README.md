# ESP32 System Monitor Server
Part of esp32-sysmon-* project; desktop server to send the data to ESP32 over USB. 
[ESP32 part of this project](https://github.com/c4ff0e/esp32-sysmon-display)

## OS support
**Linux**: 100% compatible, CLI mode

**Windows**: 100% compatible, tray mode

**macOS**: Should work, untested

## Hardware support
**DEPENDS ON CRATES**\
Crates used to get hardware metrics:\
**[all-smi](https://crates.io/crates/all-smi)**\
**[sysinfo](https://crates.io/crates/sysinfo)**\
**[systemstat](https://crates.io/crates/systemstat)**

## Limitations
#1 - In my tests, cpu temperature was not collected reliably. I don't think i can do something with this issue. CPU temperature is not displayed.

#2 - I can not implement tray mode for macOS, because i don't have a Mac device, so macOS uses CLI args.

#3 - Some hardware may not be supported. If so, the display will not show metrics for  
that hardware, but it will continue to show all supported metrics.

#4 - On my machine (Windows 11, Intel Core i5-12400f) CPU frequency remains constant. It is possible that on other machines CPU frequency remains constant

#5 - It is possible that other connected ESP32 "steals" port connection

## Build from source
```bash
git clone https://github.com/c4ff0e/esp32-sysmon-server
cd esp32-sysmon-server 
cargo bl # build linux binary
cargo bw # build windows binary
# no cargo alias for macos
```
## Run (Release)

### Linux
Download binary from **[latest release](https://github.com/c4ff0e/esp32-sysmon-server/releases/latest)**.

```bash
./esp32-sysmon-server run # start
./esp32-sysmon-server logs # prints logs and path to log file
```

**Add to PATH**\
You can put the binary wherever you like, for example: ~/.local/bin;

```bash
mkdir -p ~/.local/bin
mv esp32-sysmon-server ~/.local/bin
```
Then:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

**Autorun**
```bash
mkdir -p ~/.config/systemd/user
nano ~/.config/systemd/user/esp32-sysmon-server.service
```
Paste this inside the file:

```ini
[Unit]
Description=ESP32 System Monitor Server
After=default.target

[Service]
ExecStart=%h/.local/bin/esp32-sysmon-server run
Restart=always
RestartSec=3

[Install]
WantedBy=default.target
```
Enable and start the service:
```bash
systemctl --user daemon-reload
systemctl --user enable esp32-sysmon-server.service
systemctl --user start esp32-sysmon-server.service
```
Check:
```bash
systemctl --user status esp32-sysmon-server.service
```
**Logs are overwritten on server startup**


### Windows
Launch the esp32-sysmon-server.exe binary from **[latest release](https://github.com/c4ff0e/esp32-sysmon-server/releases/latest)**. After launch, a tray icon will appear in the system tray, which can be used to stop the server and open logs. 

**Autorun**\
Put the shortcut to the .exe inside the startup folder:
%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup

**Logs are overwritten on server startup.**

### macOS
[Build from source](#build-from-source)
```bash
./esp32-sysmon-server run # start
./esp32-sysmon-server logs # prints logs and path to log file
```

## Troubleshooting

*Permission denied for serial port **[Serial port name]**. Check device access permissions.*

**Fix**:
```bash
sudo usermod -aG uucp "$(whoami)"
```
After running this command, log out and log back in, or reboot.
