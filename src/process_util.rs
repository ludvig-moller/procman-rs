
use eframe::emath::Numeric;
use sysinfo::{ System, Pid };

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: f64,
}

pub fn get_processes(sys: &mut System, sort: String, filter: String) -> Vec<ProcessInfo> {
    sys.refresh_all();

    let cpus = sys.cpus().len() as f32;

    let processes = sys.processes();
    let mut parent_processes: Vec<ProcessInfo> = Vec::<ProcessInfo>::new();

    for (pid, process) in processes.iter() {
        // Checking for PID 0
        let pid = pid.as_u32();
        if pid == 0 { continue; }

        // Checking for name containg filter
        let name = process.name().to_os_string().into_string().unwrap();
        if !name.to_lowercase().contains(&filter.to_lowercase()) { continue; }

        // Filtering out children with same name as parent
        let name_match_index = parent_processes.iter().position(|p| p.name == name);
        if name_match_index.is_none() {
            parent_processes.push(ProcessInfo {
                pid, name, 
                cpu_usage: ((process.cpu_usage()/cpus)*100.0).round()/100.0,
                memory_usage: (((process.memory().to_f64()/sys.total_memory().to_f64())*10000.0).round())/100.0
            });
            continue;
        }

        let name_match_index = name_match_index.unwrap();
        if process.parent().is_some() && process.parent().unwrap().as_u32() != parent_processes[name_match_index].pid {
            parent_processes.remove(name_match_index);
            parent_processes.push(ProcessInfo {
                pid, name,
                cpu_usage: ((process.cpu_usage()/cpus)*100.0).round()/100.0,
                memory_usage: (((process.memory().to_f64()/sys.total_memory().to_f64())*10000.0).round())/100.0
            });
        }
    }
    
    // Sorting for either most cpu or memory usage
    parent_processes.sort_by(|a, b| 
        if sort == "CPU" {
            b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()
        } else {
            b.memory_usage.partial_cmp(&a.memory_usage).unwrap()
        }
    );

    parent_processes
}

pub fn kill_process(sys: &mut System, pid: u32) {
    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        process.kill();
    }
}
