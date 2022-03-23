use serde::Serialize;
use sysinfo::{System, SystemExt, ProcessorExt};
use std::env::consts::ARCH;

use crate::cpu_stats::cpu_load::{cpu_load, cpu_temp};

#[derive(Serialize)]
pub struct CpuObject {
    core_count: usize,
    temperature: String,
    architecture: String,
    vendor: String,
    utilization: UtilizationObject,
}

#[derive(Serialize)]
struct UtilizationObject {
    total: f32,
    per_core: Vec<f32>
}

pub fn create_cpu_object(system: &System) -> CpuObject {
    let cpu = system.global_processor_info();

    let load = cpu_load();
    let cpu_utilization = UtilizationObject {
        total: load.0,
        per_core: load.1,
    };

    CpuObject {
        core_count: get_physical_cores(system.physical_core_count()),
        temperature: cpu_temp(),
        architecture: ARCH.to_string(),
        vendor: cpu.vendor_id().to_string(),
        utilization: cpu_utilization,
    }
}

fn get_physical_cores(cores: Option<usize>) -> usize {
    match cores {
        Some(s) => s,
        None => 0
    }
}