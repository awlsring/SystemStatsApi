// External crates
use std::env;
use log::info;
use actix_web::{get, web, Responder, HttpServer, App};
use serde::Serialize;
use sysinfo::{System, SystemExt};

// Local modules
mod interfaces;
mod system;
mod memory;
mod disk;
mod cpu_stats;

// Module imports
use cpu_stats::cpu::{CpuObject, create_cpu_object};
use interfaces::{create_interface_vec, NetworkInterfaceObject, get_primary_interface};
use disk::{create_disk_vec, DiskObject};
use system::{create_system_object, SystemObject};
use memory::{MemoryObject, create_memory_object, create_swap_object};

#[derive(Serialize)]
pub struct ResponseObject {
    system: SystemObject,
    cpu: CpuObject,
    memory: MemoryObject,
    swap: MemoryObject,
    disks: Vec<DiskObject>,
    network_interfaces: Vec<NetworkInterfaceObject>,
}

#[get("/info")]
async fn info() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();

    
    
    let res = ResponseObject {
        system: create_system_object(&sys),
        cpu: create_cpu_object(&sys),
        memory: create_memory_object(&sys),
        swap: create_swap_object(&sys),
        disks: create_disk_vec(&sys.disks()),
        network_interfaces: create_interface_vec(&sys.networks()),
    };

    web::Json(res)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    

    let interface = match env::var("SYSTEMSTATS_IP") {
        Ok(val) => val,
        Err(_e) => get_primary_interface(),
    };

    let port = match env::var("SYSTEMSTATS_PORT") {
        Ok(val) => val,
        Err(_e) => "7032".to_string(),
    };

    let ip = format!("{}:{}", interface, port);

    info!("Starting on: {}", ip);

    HttpServer::new(|| {
        App::new()
            .service(info)
    })
    .bind(ip)?
    .run()
    .await
}