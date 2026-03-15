
mod metrics;
mod usb;

use crate::metrics::cpu_ram;
use crate::metrics::gpu;

use crate::usb::send;
use crate::usb::serialize;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;

fn main(){
    let stop = Arc::new(AtomicBool::new(true));
    let worker_stop = Arc::clone(&stop);

    let main_thread = thread::spawn(move || {
        worker(worker_stop)
        });
    main_thread.join().unwrap();
}

fn worker(stop: Arc<AtomicBool>) {
    let mut cpu_ram_metrics = cpu_ram::CpuRamMetrics::new();
    let mut gpu_metrics = gpu::GpuMetrics::new();

    let mut port = match send::connect() {
        Ok(port_handle) => port_handle,
        Err(e) => {
            eprintln!("ERROR CONNECTING TO PORT: {}", e);
            panic!()
        }
    };

    '_main: loop {
        if !stop.load(Ordering::Relaxed) {                        
            break;                                                                   
        }
        cpu_ram_metrics.refresh();
        gpu_metrics.refresh();

        // there used to be a crash if there is no metrics available;
        // however, it is better to display this information on a screen then sliently(?) stop working

        let metrics_data = serialize::MetricsData {
            cpu_usage: cpu_ram_metrics.cpu_usage,
            cpu_name: cpu_ram_metrics.cpu_name.clone(),
            cpu_frequency: cpu_ram_metrics.cpu_frequency,
            cpu_is_supported: cpu_ram_metrics.cpu_is_supported,
            cpu_temp: cpu_ram_metrics.cpu_temp,

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
        match send::send(&mut *port, &serialized_data) {
            Ok(_) => {
                
            }
            Err(_) => {
                println!("Error while sending data. Trying to reconnect...");
                port = match send::connect() {
                    Ok(port_handle) => port_handle,
                    Err(e) => {
                        eprintln!("ERROR RECONNECTING TO PORT: {}", e);
                        continue '_main;
                    }
                };
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1.5)); // timeout
    }
}
