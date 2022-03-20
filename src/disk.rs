use std::{path::Path, ffi::OsStr};

use serde::Serialize;
use sysinfo::{DiskExt, Disk, DiskType};

#[derive(Serialize)]
pub struct DiskObject {
    name: String,
    mount_point: String,
    available_space: u64,
    total_space: u64,
    file_system: String,
    is_removable: bool,
    disk_type: String,
}

pub fn create_disk_vec(disks: &[Disk]) -> Vec<DiskObject>{
    let mut disk_objects = Vec::new();

    for disk in disks {
        disk_objects.push(create_disk_object(disk));
    }
    disk_objects
}

fn create_disk_object(disk: &Disk) -> DiskObject {
    DiskObject {
        name: disk_name_to_string(disk.name()),
        file_system: fs_as_string(disk.file_system()),
        mount_point: mount_point_to_string(disk.mount_point()),
        disk_type: disk_type_as_sting(disk.type_()),
        available_space: disk.available_space(),
        total_space: disk.total_space(),
        is_removable: disk.is_removable(),
    }
}

fn disk_type_as_sting(disk_type: DiskType) -> String {
    match disk_type {
        DiskType::SSD => "SSD".to_string(),
        DiskType::HDD => "HDD".to_string(),
        _ => "Unknown".to_string()
    }
}

fn fs_as_string(file_system: &[u8]) -> String {
    let fs_string = std::str::from_utf8(file_system);
    match fs_string {
        Ok(fs_string) => fs_string.to_string().to_uppercase(),
        Err(_e) => "Unknown".to_string()
    }
}

fn mount_point_to_string(mount: &Path) -> String {
    let mount_string = mount.to_str();
    match mount_string {
        Some(m) => m.to_string(),
        None => "Unknown".to_string(),
    }
}

fn disk_name_to_string(disk_name: &OsStr) -> String {
    let disk_string = disk_name.to_str();
    match disk_string {
        Some(d) => d.to_string(),
        None => "Unknown".to_string(),
    }
}