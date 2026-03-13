# System Monitor Server
Part of esp32-sysmon-* project; desktop server to send the data to ESP32

## Build
1) Clone this repository;

2) cargo run

## Troubleshooting

*Permission denied for serial port **[Serial port name]**. Check device access permissions.*

**Fix**:
```bash
sudo usermod -aG uucp "$(whoami)"
```
after this command: reboot/logout.