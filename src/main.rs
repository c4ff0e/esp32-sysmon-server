mod metrics;
mod usb;
use crate::metrics::cpu_ram;
use crate::metrics::gpu;

use crate::usb::serialize;
use crate::usb::send;
fn main() {
    let mut cpu_ram_metrics = cpu_ram::CpuRamMetrics::new();
    let mut gpu_metrics = gpu::GpuMetrics::new();
    loop {
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
        };
        let serialized_data = serialize::serialize(&metrics_data).unwrap();
        let port = match send::find_port() {
            Some(port) => port,
            None => continue,
        };

        println!("port: {}", &port)
    }
}

