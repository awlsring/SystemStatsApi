use serde::Serialize;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
pub struct SystemObject {
    system_name: String,
    kernel_version: String,
    os_name: String,
    os_version: String,
    host_name: String,
    boot_time: u64,
    up_time: u64,
}

fn handle_optional_string(optional: Option<String>) -> String {
    match optional {
        Some(s) => s,
        None => "Unknown".to_string()
    }
}

pub fn create_system_object(system: &System) -> SystemObject {
    SystemObject {
        system_name: handle_optional_string(system.name()),
        kernel_version: handle_optional_string(system.kernel_version()),
        os_name: handle_optional_string(system.long_os_version()),
        os_version: handle_optional_string(system.os_version()),
        host_name: handle_optional_string(system.host_name()),
        boot_time: system.boot_time(),
        up_time: system.uptime(),
    }
}