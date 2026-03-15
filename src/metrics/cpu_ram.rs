use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use systemstat::{Platform, System as System2};
pub struct CpuRamMetrics {
    sys: System,
    sys2: System2,
    pub cpu_usage: f32,
    pub cpu_name: String,
    pub cpu_frequency: u32,
    pub cpu_temp: f32, //doesnt really work everywhere, but it is better to have it then not have it at all
    pub total_ram: u64,
    pub used_ram: u64,
    pub cpu_is_supported: bool,
}

impl CpuRamMetrics {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(
                    CpuRefreshKind::everything()
                        .with_frequency()
                        .with_cpu_usage(),
                )
                .with_memory(MemoryRefreshKind::nothing().with_ram()),
        );
        let sys2 = System2::new();
        let cpu_name = sys.cpus()[0].brand().to_string();
        let cpu_frequency = sys.cpus()[0].frequency() as u32;
        let cpu_temp = match sys2.cpu_temp() {
            Ok(temp) => temp,
            Err(_) => 0.0,
        };

        let total_ram = sys.total_memory();
        let used_ram = sys.used_memory();

        let is_supported = sysinfo::IS_SUPPORTED_SYSTEM;
        Self {
            sys,
            sys2,
            cpu_usage: 0.0,
            cpu_name,
            cpu_frequency,
            cpu_temp,
            total_ram,
            used_ram,
            cpu_is_supported: is_supported,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_frequency();
        self.update_cpu_usage();
        self.cpu_temp = match self.sys2.cpu_temp() {
            Ok(temp) => temp,
            Err(_) => 0.0,
        };

        self.sys.refresh_memory();
        self.total_ram = self.sys.total_memory();
        self.used_ram = self.sys.used_memory();
    }
    fn update_cpu_usage(&mut self) {
        self.sys.refresh_cpu_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_cpu_all();
        let mut sum = 0.0;
        let mut count = 0;
        for cpu in self.sys.cpus() {
            sum += cpu.cpu_usage();
            count += 1;
        }
        self.cpu_usage = sum / count as f32;
    }
}
