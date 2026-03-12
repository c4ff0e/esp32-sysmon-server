use all_smi::prelude::*;

pub struct GpuMetrics {
    smi: AllSmi,
    pub gpu_name: String,
    pub gpu_usage: f32,
    pub gpu_temp: u32,
    pub gpu_memory_total: u64,
    pub gpu_memory_used: u64,
    pub gpu_freq: u32,
}

impl GpuMetrics {
    pub fn new() -> Self {

        let smi = match AllSmi::new(){
            Ok(smi) => smi,
            Err(e) => panic!("GPU Metrics failed to init!{}. Crash for now.",e) //TODO: you know
        };

        let gpus = smi.get_gpu_info();
        let gpu = match gpus.first(){ // usually first is main
            Some(gpu) => gpu,
            None => panic!("No GPU found. Crash for now")
        };
        let gpu_name = gpu.name.clone();
        let gpu_usage = gpu.utilization as f32;
        let gpu_temp = gpu.temperature;
        let gpu_memory_total = gpu.total_memory as u64;
        let gpu_memory_used = gpu.used_memory as u64;
        let gpu_freq = gpu.frequency;

        Self{
            smi,
            gpu_name,
            gpu_usage,
            gpu_temp,
            gpu_memory_total,
            gpu_memory_used,
            gpu_freq
        }
    }
    pub fn refresh(&mut self){
        let gpus = self.smi.get_gpu_info();
        let gpu = match gpus.first(){
            Some(gpu) => gpu,
            None => panic!("No GPU found. Crash for now")
        };
        self.gpu_name = gpu.name.clone();
        self.gpu_usage = gpu.utilization as f32;
        self.gpu_temp = gpu.temperature;
        self.gpu_memory_total = gpu.total_memory as u64;
        self.gpu_memory_used = gpu.used_memory as u64;
        self.gpu_freq = gpu.frequency;
    }
}