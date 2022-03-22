use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};

pub fn cpu_load() -> (f32, Vec<f32>) {
    let sys = System::new();
    let aggregate = sys.cpu_load_aggregate();
    let all = sys.cpu_load();
    let mut load = Vec::new();
    
    thread::sleep(Duration::from_secs(1));

    match all {
        Ok(cpus)=> {
            let cpu_done = cpus.done().unwrap();
            for cpu in cpu_done {
                load.push(cpu.system * 100.0)
            }
        },
        Err(_e) => load.push(0.0)
    }

    let aggregate_load;
    match aggregate {
        Ok(cpu)=> {
            let cpu = cpu.done().unwrap();
            aggregate_load = cpu.system * 100.0
        },
        Err(_e) => aggregate_load = 0.0
    }
    (aggregate_load, load)
}

pub fn cpu_temp() -> String {
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => format!("{}°C", cpu_temp).to_string(),
        Err(_e) => "Unknown".to_string()
    }
}