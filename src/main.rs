
use actix_web::{get, web, Responder, HttpServer, App};
use serde::Serialize;
use sysinfo::{System, SystemExt};

mod interface;
mod system;
mod cpu;
mod memory;
mod disk;

use interface::{create_interface_vec, NetworkInterfaceObject};
use disk::{create_disk_vec, DiskObject};
use system::{create_system_object, SystemObject};
use cpu::{CpuObject, create_cpu_object};
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
    HttpServer::new(|| {
        App::new()
            .service(info)
    })
    .bind("127.0.0.1:7032")?
    .run()
    .await
}