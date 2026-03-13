use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetricsData {
    pub cpu_usage: f32,
    pub cpu_name: String,
    pub cpu_frequency: u32,

    pub total_ram: u64,
    pub used_ram: u64,

    pub gpu_name: String,
    pub gpu_usage: f32,
    pub gpu_temp: u32,
    pub gpu_memory_total: u64,
    pub gpu_memory_used: u64,
    pub gpu_freq: u32,
    pub gpu_supported: bool,
}

pub fn serialize(data: &MetricsData) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_allocvec(data)
}
