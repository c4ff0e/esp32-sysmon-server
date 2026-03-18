mod common;
mod linux;
mod metrics;
mod usb;
mod windows;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

use crate::metrics::cpu_ram;
use crate::metrics::gpu;

use crate::common::logs;
use log::info;

use crate::usb::send;
use crate::usb::serialize;

#[cfg(target_os = "windows")]
use crate::windows::tray;

#[cfg(target_os = "linux")]
use crate::linux::args;

// lifetime checker
fn should_stop(run: &Arc<AtomicBool>) -> bool {
    !run.load(Ordering::Relaxed)
}

fn main() {
    
    //creating logger
    let log_dir = match logs::log_dir() {
        Ok(log_dir) => log_dir,
        Err(e) => {
            panic!("Failed to get project directory: {}", e);
        }
    };
    let log_file = log_dir.join("server.log");
    logs::create_logger(&log_file);

    info!("Logger ok");
    let run = Arc::new(AtomicBool::new(true));
    let worker_run = Arc::clone(&run);

    //linux: accept cli args
    #[cfg(target_os = "linux")]
    args::get_args(log_file);

    // windows: tray icon
    #[cfg(target_os = "windows")]
    let _tray = match tray::build_tray() {
        Ok(tray) => tray,
        Err(e) => {
            panic!("tray build error: {}", e);
        }
    };

    let worker_thread = thread::spawn(move || worker(worker_run));

    #[cfg(target_os = "windows")]
    windows::tray::run_event_loop(Arc::clone(&run), &log_file);

    worker_thread.join().unwrap();
}

fn worker(run: Arc<AtomicBool>) {
    let mut cpu_ram_metrics = cpu_ram::CpuRamMetrics::new();
    let mut gpu_metrics = gpu::GpuMetrics::new();

    let mut port = loop {
        match send::connect(&run) {
            Ok(port_handle) => break port_handle,
            Err(e) => {
                info!("ERROR CONNECTING TO PORT: {}", e);

                if should_stop(&run) {
                    return;
                }
            }
        }
    };

    '_main: loop {
        if should_stop(&run) {
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
            Ok(_) => {}
            Err(_) => {
                info!("Error while sending data. Trying to reconnect...");
                //the same code as before main loop. DRY broken :(
                port = match send::connect(&run) {
                    Ok(port_handle) => port_handle,
                    Err(e) => {
                        info!("ERROR RECONNECTING TO PORT: {}", e);

                        if should_stop(&run) {
                            break;
                        }

                        continue '_main;
                    }
                };
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1.5)); // timeout
    }
}
