use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MetricsData {
    pub cpu_usage: f32,
    pub cpu_name: String,
    pub cpu_frequency: u32,
    pub cpu_is_supported: bool,
    pub cpu_temp: f32,

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
    postcard::to_allocvec_cobs(data)
}

#[cfg(test)]
    mod tests{
        use super::*;
        #[test]
        // checks if data is the same afterv deserialization
        fn round_trip(){
            let data_pre = MetricsData {
                cpu_usage: 98.0,
                cpu_name: "String".to_string(),
                cpu_frequency: 2500,
                cpu_is_supported: true,
                cpu_temp: 0.0,
    
                total_ram: 12345678,
                used_ram: 12345678,
    
                gpu_name: "String".to_string(),
                gpu_usage: 10.0,
                gpu_temp: 20,
                gpu_memory_total: 12345678,
                gpu_memory_used: 12345678,
                gpu_freq: 2500,
                gpu_supported: true,
            };
            let bytes = serialize(&data_pre).unwrap();
            let deser:MetricsData = postcard::from_bytes_cobs(&mut bytes.clone()).unwrap();
            assert_eq!(data_pre, deser)
        }
        
    }