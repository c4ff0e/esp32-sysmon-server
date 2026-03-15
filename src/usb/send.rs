use serialport::SerialPort;
use serialport::{self};

pub fn find_port() -> serialport::Result<String> {
    loop {
        let ports = match serialport::available_ports() {
            Ok(ports) => ports,
            Err(err) => {
                return Err(err);
            }
        };
        for port in ports {
            if let serialport::SerialPortType::UsbPort(info) = port.port_type
                && info.vid == 0x303A
                && info.pid == 0x3001
            {
                return Ok(port.port_name);
            }
        }
        println!("No port found. Retrying...");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

pub fn open_port(port_name: &str) -> serialport::Result<Box<dyn SerialPort>> {
    serialport::new(port_name, 115200).open()
}

pub fn send(port: &mut dyn SerialPort, serialized_data: &[u8]) -> Result<(), std::io::Error> {
    port.write_all(serialized_data)
}

pub fn connect() -> serialport::Result<Box<dyn SerialPort>> {
    let port_name = match find_port() {
        Ok(name) => name,
        Err(e) => {
            return Err(e);
        }
    };
    let port = match open_port(&port_name) {
        Ok(port_handle) => port_handle,
        Err(e) => {
            return Err(e);
        }
    };
    Ok(port)
}
