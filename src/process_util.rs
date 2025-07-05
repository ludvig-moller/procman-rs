
use eframe::emath::Numeric;
use sysinfo::{System, Pid,};

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: f64,
}

pub fn get_processes(sys: &mut System, sort: String, filter: String) -> Vec<ProcessInfo> {
    sys.refresh_all();
    
    let mut processes: Vec<ProcessInfo> = sys.processes()
        .iter()
        .filter_map(
            |(pid, process)|
                if pid.as_u32() != 0 && process.name().to_os_string().into_string().unwrap().to_lowercase().contains(&filter.to_lowercase()) {
                    Some(ProcessInfo {
                        pid: pid.as_u32(),
                        name: process.name().to_os_string().into_string().unwrap(),
                        cpu_usage: ((process.cpu_usage()*100.0).round())/100.0,
                        memory_usage: (((process.memory().to_f64()/sys.total_memory().to_f64())*10000.0).round())/100.0,
                    })
                } else {
                    None
                }
            )
        .collect();
    
    processes.sort_by(|a, b| 
        if sort == "CPU" {
            b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()
        } else {
            b.memory_usage.partial_cmp(&a.memory_usage).unwrap()
        }
    );

    processes
}

pub fn kill_process(sys: &mut System, pid: u32) {
    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        process.kill();
    }
}
