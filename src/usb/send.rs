use serialport;

pub fn find_port() -> Option<String> {
    let ports = serialport::available_ports().unwrap_or_else(|_| Vec::new());
    for port in ports {
        if let serialport::SerialPortType::UsbPort(info) = port.port_type {
            if info.vid == 0x303A && info.pid == 0x3001 {
                return Some(port.port_name);
            }
        }
    }
    println!("ESP32 not found.");
    None
}
