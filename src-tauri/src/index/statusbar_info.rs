use sysinfo::{CpuExt, System, SystemExt};

#[derive(Clone, serde::Serialize)]
pub struct StatusBarUsageInfo {
    pub cpu_usage: f32,
    pub per_cpu_usage: Vec<f32>,
    pub mem_usage: f64,
    pub mem_used: u64,
    pub mem_total: u64,
    pub swap_usage: f64,
    pub swap_used: u64,
    pub swap_total: u64,
}

impl StatusBarUsageInfo {
    // StatusBarUsageInfo 构造关联函数
    pub fn new() -> StatusBarUsageInfo {
        return StatusBarUsageInfo {
            cpu_usage: 0f32,
            per_cpu_usage: vec![],
            mem_usage: 0f64,
            mem_used: 0u64,
            mem_total: 0u64,
            swap_usage: 0f64,
            swap_used: 0u64,
            swap_total: 0u64,
        };
    }
    // 刷新系统信息方法
    pub fn update(&mut self, sys: &mut System) {
        // 刷新系统信息
        sys.refresh_all();

        // CPU
        let cpus = sys.cpus();
        let cpu_num = cpus.len() as f32;
        // CPU利用率
        self.cpu_usage = {
            let mut total_cpu_usage = 0f32;
            for cpu in sys.cpus() {
                // CPU每个核心利用率
                self.per_cpu_usage.push(cpu.cpu_usage() / 100f32);
                total_cpu_usage += cpu.cpu_usage();
            }
            total_cpu_usage / cpu_num / 100f32
        };

        // 内存
        // 总内存
        self.mem_total = sys.total_memory();
        // 已用内存
        self.mem_used = sys.used_memory();
        // 内存使用率
        self.mem_usage = self.mem_used as f64 / self.mem_total as f64;
        // 虚拟内存
        self.swap_total = sys.total_swap();
        // 已用内存
        self.swap_used = sys.used_swap();
        // 内存使用率
        self.swap_usage = self.swap_used as f64 / self.swap_total as f64;
    }
}
