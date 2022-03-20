use serde::Serialize;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
pub struct MemoryObject {
    total: u64,
    used: u64,
    available: u64,
}

pub fn create_swap_object(system: &System) -> MemoryObject {
    MemoryObject {
        total: system.total_swap(),
        used: system.used_swap(),
        available: system.free_swap(),
    }
}

pub fn create_memory_object(system: &System) -> MemoryObject {
    MemoryObject {
        total: system.total_memory(),
        used: system.used_memory(),
        available: system.available_memory(),
    }
}