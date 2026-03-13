mod metrics;
mod usb;

use crate::metrics::cpu_ram;
use crate::metrics::gpu;

use crate::usb::send;
use crate::usb::serialize;
fn main() {
    let mut cpu_ram_metrics = cpu_ram::CpuRamMetrics::new();
    let mut gpu_metrics = gpu::GpuMetrics::new();

    let port ='_find_port: loop {

        // find correct port
        match send::find_port() {
            Some(port_name) => {

                // open found port
                println!("Found ESP32 on port: {}", port_name);
                let port_handle = serialport::new(&port_name, 9600).open();

                // check if port actually opens
                match port_handle {
                    Ok(port_handle) => {
                        println!("Successfully connected to ESP32.");
                        break port_handle;
                    }

                    Err(e) => {
                        match e.kind() {
                            serialport::ErrorKind::Io(std::io::ErrorKind::PermissionDenied) => {
                                eprintln!(
                                    "Permission denied for serial port {}. Check device access permissions.",
                                    port_name
                                );
                            }
                            serialport::ErrorKind::NoDevice => {
                                eprintln!("Serial device {} is no longer available.", port_name);
                            }
                            _ => {
                                eprintln!("Failed to open port {}: {}", port_name, e);
                            }
                        }
                        std::thread::sleep(std::time::Duration::from_secs(5)); // wait before retrying
                        continue;
                    }
                };
            }
            None => {
                std::thread::sleep(std::time::Duration::from_secs(5)); // wait before retrying
                continue;
            }
        };
    };

    '_main: loop {
        cpu_ram_metrics.refresh();
        gpu_metrics.refresh();

        let metrics_data = serialize::MetricsData {
            cpu_usage: cpu_ram_metrics.cpu_usage,
            cpu_name: cpu_ram_metrics.cpu_name.clone(),
            cpu_frequency: cpu_ram_metrics.cpu_frequency,
            total_ram: cpu_ram_metrics.total_ram,
            used_ram: cpu_ram_metrics.used_ram,
            gpu_name: gpu_metrics.gpu_name.clone(),
            gpu_usage: gpu_metrics.gpu_usage,
            gpu_temp: gpu_metrics.gpu_temp,
            gpu_memory_total: gpu_metrics.gpu_memory_total,
            gpu_memory_used: gpu_metrics.gpu_memory_used,
            gpu_freq: gpu_metrics.gpu_freq,
            gpu_supported: gpu_metrics.supported,
        };
        let serialized_data = serialize::serialize(&metrics_data).unwrap();
    }
}
