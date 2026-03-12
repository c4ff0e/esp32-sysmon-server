mod metrics;
use crate::metrics::cpu_ram;
use crate::metrics::gpu;
fn main() {
    let mut cpu_ram_metrics = cpu_ram::CpuRamMetrics::new();
    let mut gpu_metrics = gpu::GpuMetrics::new();
    loop {
        cpu_ram_metrics.refresh();
        gpu_metrics.refresh();
        println!("================================================");
        println!("CPU Usage: {:.2}%", cpu_ram_metrics.cpu_usage);
        println!("CPU Name: {}", cpu_ram_metrics.cpu_name);
        println!("CPU Frequency: {} MHz", cpu_ram_metrics.cpu_frequency);
        println!("Total RAM: {} MB", cpu_ram_metrics.total_ram / 1024 / 1024);
        println!("Used RAM: {} MB", cpu_ram_metrics.used_ram / 1024 / 1024);
        println!("Is Supported: {}", cpu_ram_metrics.is_supported);
        println!("================================================");
        println!("GPU: {}", gpu_metrics.gpu_name);
        println!("GPU Usage: {:.2}%", gpu_metrics.gpu_usage);
        println!("GPU Temp: {}°C", gpu_metrics.gpu_temp);
        println!("GPU Memory Total: {} MB", gpu_metrics.gpu_memory_total / 1024 / 1024);
        println!("GPU Memory Used: {} MB", gpu_metrics.gpu_memory_used / 1024 / 1024);
        println!("GPU Frequency: {} MHz", gpu_metrics.gpu_freq);
    }


}
