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
        let cpu_name = Self::cpu_name_normal(sys.cpus()[0].brand().to_string());

        // format cpu name so it is usable
        // directly supports only intel/amd
        // apple silicon should not require formatting
        let cpu_frequency = sys.cpus()[0].frequency() as u32;
        let cpu_temp = sys2.cpu_temp().unwrap_or(0.0);

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
        self.update_cpu();
        

        self.sys.refresh_memory();
        self.total_ram = self.sys.total_memory();
        self.used_ram = self.sys.used_memory();
    }
    fn update_cpu(&mut self) {
        self.sys.refresh_cpu_all();
        self.cpu_temp = self.sys2.cpu_temp().unwrap_or(0.0);

        // mean cpu frequency for all cores
        let mut sum = 0;
        let mut count = 0;
        for cpu in self.sys.cpus(){
            sum += cpu.frequency();
            count += 1;
        }
        self.cpu_frequency = (sum / count) as u32;
        // to update cpu usage
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_cpu_all();

        // mean cpu usage for all cores
        let mut sum = 0.0;
        let mut count = 0;
        for cpu in self.sys.cpus() {
            sum += cpu.cpu_usage();
            count += 1;
        }
        self.cpu_usage = sum / count as f32;
    }
    fn cpu_name_normal(mut cpu_name: String) -> String{
        if cpu_name.contains("Intel"){
            //cut off everything before processor name
            cpu_name = cpu_name
            .split_once("Intel")
            .unwrap()
            .1
            .to_string();
            
            // cut off everything after @
            cpu_name = cpu_name
            .split("@")
            .next()
            .unwrap() // should not panic
            .replace("(TM)", "")
            .replace("(R)", "")
            .replace("CPU", "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_string();
        }
        else if cpu_name.contains("AMD"){
            // cut off everyhting before processor name
            cpu_name = cpu_name
            .split_once("AMD")
            .unwrap()
            .1
            .to_string();

            cpu_name = cpu_name
            .replace("Processor", "")
            .replace("with Radeon Graphics", "")
            .replace("with Radeon Vega Graphics", "")
            .replace("w/ Radeon Vega Graphics", "")
            .replace("with Radeon Vega Mobile Gfx", "")
            .replace("(tm)", "")
            .replace("(TM)", "")
            .split_whitespace()
            .filter(|part| !part.ends_with("-Core") && !part.ends_with("-Cores"))
            .collect::<Vec<_>>()
            .join(" ")
            .to_string();
        }
        // if cpu name is too long - shorten it
        if cpu_name.chars().count() > 19 {
            cpu_name = cpu_name.chars().take(16).collect();
            cpu_name.push_str("...");
            return cpu_name;
        }
        if cpu_name.chars().count() == 0 {
            cpu_name = "Unknown CPU".to_string();
            cpu_name
        }
        else {
            cpu_name
        }
    }
}

#[cfg(test)]
//there is almost to no reason to test creating/updating struct, as it is dependednt on real hardware
    mod tests{
        use super::*;
        #[test]
        fn normalizes_intel_modern(){
            let cpu_name = "12th Gen Intel(R) Core(TM) i5-12400f";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Core i5-12400f")
        }
        #[test]
        fn normalizes_intel_core(){
            let cpu_name = "Intel(R) Core(TM) Ultra 7 165U";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Core Ultra 7 165U")
        }
        #[test]
        fn normalizes_intel_old(){
            let cpu_name = "Intel(R) Core(TM) i5-6500T CPU @ 2.50GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Core i5-6500T")
        }
        #[test]
        fn normalizes_intel_xeon(){
            let cpu_name = "Intel(R) Xeon(R) Silver 4210 CPU @ 2.20GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Xeon Silver 4210")
        }
        #[test]
        fn normalizes_intel_xeon_old(){
            let cpu_name = "Intel(R) Xeon(R) CPU E5450 @ 3.00GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Xeon E5450")
        }

        #[test]
        fn normalizes_intel_pentium(){
            let cpu_name = "Intel(R) Pentium(R) Gold G6400 CPU @ 4.00GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Pentium Gold G6400")
        }
        #[test]
        fn normalizes_intel_celeron(){
            let cpu_name = "Intel(R) Celeron(R) G5900 CPU @ 3.40GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Celeron G5900")
        }
        #[test]
        fn normalizes_intel_atom(){
            let cpu_name = "Intel(R) Atom(TM) CPU D2550 @ 1.86GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Atom D2550")
        }
        #[test]
        fn normalizes_amd_ryzen(){
            let cpu_name = "AMD Ryzen 7 5800X 8-Core Processor";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Ryzen 7 5800X")
        }
        #[test]
        fn normalizes_amd_ryzen_igpu(){
            let cpu_name = "AMD Ryzen 5 5600G with Radeon Graphics";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Ryzen 5 5600G")
        }
        #[test]
        fn normalizes_amd_ryzen_igpu2(){
            let cpu_name = "AMD Ryzen 5 3500U with Radeon Vega Mobile Gfx";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Ryzen 5 3500U")
        }
        #[test]
        fn normalizes_amd_epyc(){
            let cpu_name = "AMD EPYC 4585PX 16-Core Processor";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "EPYC 4585PX")
        }
        #[test]
        fn normalizes_amd_threadripper_long(){
            let cpu_name = "AMD Ryzen Threadripper 2990X 32-Core Processor";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Ryzen Threadripp...")
        }
        #[test]
        fn normalizes_amd_athlon(){
            let cpu_name = "AMD Athlon Silver 3050U with Radeon Graphics";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Athlon Silver 3050U")
        }
        #[test]
        fn normalizes_amd_athlon2(){
            let cpu_name = "AMD Athlon 200GE with Radeon Vega Graphics";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Athlon 200GE")
        }
        #[test]
        fn normalizes_amd_athlon3(){
            let cpu_name = "AMD Athlon PRO 200GE w/ Radeon Vega Graphics";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Athlon PRO 200GE")
        }
        #[test]
        fn normalizes_amd_fx(){
            let cpu_name = "AMD FX(tm)-8350 Eight-Core Processor";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "FX-8350")
        }
        #[test]
        fn does_not_touch_apple(){
            let cpu_name = "Apple M3 Ultra";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Apple M3 Ultra")
        }
        #[test]
        fn shortens_long_exotic_name(){
            let cpu_name = "VIA QuadCore U4650 @ 1.0+ GHz";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "VIA QuadCore U46...")
        }
        #[test]
        fn empty_name(){
            let cpu_name = "";
            let normalized = CpuRamMetrics::cpu_name_normal(cpu_name.to_string());
            assert_eq!(normalized, "Unknown CPU")
        }
    }