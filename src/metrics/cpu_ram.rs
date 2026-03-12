use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct CpuRamMetrics {
    sys: System,
    pub cpu_usage: f32,
    pub cpu_name: String,
    pub cpu_frequency: u32,
    pub total_ram: u64,
    pub used_ram: u64,
    pub is_supported: bool,
}

impl CpuRamMetrics{
    pub fn new() -> Self {
        let sys = System::new_with_specifics(RefreshKind::nothing()
            .with_cpu(
                CpuRefreshKind::everything()
                .with_frequency()
                .with_cpu_usage()
            )
            .with_memory(
                MemoryRefreshKind::nothing()
                .with_ram())
            );
        let cpu_name = sys.cpus()[0].brand().to_string();
        let cpu_frequency = sys.cpus()[0].frequency() as u32;

        let total_ram = sys.total_memory() as u64;
        let used_ram = sys.used_memory() as u64;

        let is_supported = sysinfo::IS_SUPPORTED_SYSTEM;
        Self{
            sys,
            cpu_usage: 0.0,
            cpu_name,
            cpu_frequency,
            total_ram,
            used_ram,
            is_supported
        }
    }

    pub fn refresh(&mut self){
        
        self.sys.refresh_cpu_frequency();
        self.update_cpu_usage();

        self.sys.refresh_memory();
        self.total_ram = self.sys.total_memory() as u64;
        self.used_ram = self.sys.used_memory() as u64;

    }
    fn update_cpu_usage(&mut self){
        self.sys.refresh_cpu_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_cpu_all();
        let mut sum = 0.0;
        let mut count = 0;
        for cpu in self.sys.cpus() {
            sum += cpu.cpu_usage() as f32;
            count += 1;
        }
        self.cpu_usage = sum / count as f32;

    }
}