use all_smi::prelude::*;
use log::info;

pub struct GpuMetrics {
    smi: Option<AllSmi>,
    pub gpu_name: String,
    pub gpu_usage: f32,
    pub gpu_temp: u32,
    pub gpu_memory_total: u64,
    pub gpu_memory_used: u64,
    pub gpu_freq: u32,
    pub supported: bool,
}

impl GpuMetrics {
    pub fn new() -> Self {
        let smi = match AllSmi::new() {
            Ok(smi) => Some(smi),
            Err(_) => {
                info!("Failed to initialize GPU metrics backend.");
                None
            }
        };

        let smi = match smi {
            Some(smi) => smi,
            None => {
                return Self {
                    smi: None,
                    gpu_name: "UNSUPPORTED".to_string(),
                    gpu_usage: 0.0,
                    gpu_temp: 0,
                    gpu_memory_total: 0,
                    gpu_memory_used: 0,
                    gpu_freq: 0,
                    supported: false,
                };
            }
        };

        let gpus = smi.get_gpu_info();
        let gpu = match gpus.first() {
            // usually first is main
            Some(gpu) => gpu,
            None => {
                return Self {
                    smi: None,
                    gpu_name: "UNSUPPORTED".to_string(),
                    gpu_usage: 0.0,
                    gpu_temp: 0,
                    gpu_memory_total: 0,
                    gpu_memory_used: 0,
                    gpu_freq: 0,
                    supported: false,
                };
            }
        };
        let gpu_name = gpu.name.clone();
        let gpu_usage = gpu.utilization as f32;
        let gpu_temp = gpu.temperature;
        let gpu_memory_total = gpu.total_memory as u64;
        let gpu_memory_used = gpu.used_memory as u64;
        let gpu_freq = gpu.frequency;

        Self {
            smi: Some(smi),
            gpu_name,
            gpu_usage,
            gpu_temp,
            gpu_memory_total,
            gpu_memory_used,
            gpu_freq,
            supported: true,
        }
    }
    pub fn refresh(&mut self) {
        let smi = match &self.smi {
            Some(smi) => smi,
            None => {
                self.gpu_name = "UNSUPPORTED".to_string();
                self.gpu_usage = 0.0;
                self.gpu_temp = 0;
                self.gpu_memory_total = 0;
                self.gpu_memory_used = 0;
                self.gpu_freq = 0;
                self.supported = false;
                return;
            }
        };
        let gpus = smi.get_gpu_info();
        let gpu = match gpus.first() {
            Some(gpu) => gpu,
            None => {
                self.gpu_name = "UNSUPPORTED".to_string();
                self.gpu_usage = 0.0;
                self.gpu_temp = 0;
                self.gpu_memory_total = 0;
                self.gpu_memory_used = 0;
                self.gpu_freq = 0;
                self.supported = false;
                return;
            }
        };
        self.gpu_name = gpu.name.clone();
        self.gpu_usage = gpu.utilization as f32;
        self.gpu_temp = gpu.temperature;
        self.gpu_memory_total = gpu.total_memory;
        self.gpu_memory_used = gpu.used_memory;
        self.gpu_freq = gpu.frequency;
        self.supported = true;
    }
}
