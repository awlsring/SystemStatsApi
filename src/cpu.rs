use serde::Serialize;
use sysinfo::{System, SystemExt, ProcessorExt, Processor};
use std::env::consts::ARCH;

#[derive(Serialize)]
pub struct CpuObject {
    core_count: usize,
    total_utilization: f32,
    architecture: String,
    processors: Vec<ProcessorObject>
}

#[derive(Serialize)]
struct ProcessorObject {
    name: String,
    brand: String,
    utilization: f32,
    vendor: String,
    frequency: u64,
}

pub fn create_cpu_object(system: &System) -> CpuObject {
    let cpu = system.global_processor_info();
    let mut processors = Vec::new();

    for processor in system.processors() {
        processors.push(create_processor_object(processor))
    }

    CpuObject {
        core_count: get_physical_cores(system.physical_core_count()),
        total_utilization: cpu.cpu_usage(),
        architecture: ARCH.to_string(),
        processors: processors,
    }
}

fn get_physical_cores(cores: Option<usize>) -> usize {
    match cores {
        Some(s) => s,
        None => 0
    }
}

fn create_processor_object(processor: &Processor) -> ProcessorObject {
    ProcessorObject {
        vendor: processor.vendor_id().to_string(),
        name: processor.name().to_string(),
        brand: processor.brand().to_string(),
        utilization: processor.cpu_usage(),
        frequency: processor.frequency(),
    }
}