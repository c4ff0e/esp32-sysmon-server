use log::info;
use serialport::SerialPort;
use serialport::{self};
use std::sync::{Arc, atomic::AtomicBool};

use crate::should_stop;

pub fn find_port(run: &Arc<AtomicBool>) -> serialport::Result<String> {
    loop {
        if should_stop(run) {
            return Err(serialport::Error::new(
                serialport::ErrorKind::NoDevice,
                "Server stopped by user",
            ));
        }

        let ports = serialport::available_ports()?;

        for port in ports {
            if let serialport::SerialPortType::UsbPort(info) = port.port_type
                && info.vid == 0x303A
                && info.pid == 0x3001
            {
                return Ok(port.port_name);
            }
        }
        info!("No device found. Retrying...");
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

pub fn open_port(port_name: &str) -> serialport::Result<Box<dyn SerialPort>> {
    info!("Found device: {}. ", port_name);
    serialport::new(port_name, 115200).open()
}

pub fn send(port: &mut dyn SerialPort, serialized_data: &[u8]) -> Result<(), std::io::Error> {
    port.write_all(serialized_data)
}

pub fn connect(run: &Arc<AtomicBool>) -> serialport::Result<Box<dyn SerialPort>> {
    let port_name = match find_port(run) {
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
