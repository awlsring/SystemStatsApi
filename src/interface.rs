use serde::Serialize;
use sysinfo::{NetworkData, NetworkExt, Networks};

#[derive(Serialize)]
pub struct NetworkInterfaceObject {
    name: String,
    bytes_transmitted: u64,
    bytes_recieved: u64,
    packets_transmitted: u64,
    packets_recieved: u64,
}

pub fn create_interface_vec(networks: &Networks) -> Vec<NetworkInterfaceObject>{
    let mut interface_objects = Vec::new();

    for (interface_name, data) in networks {
        interface_objects.push(create_network_interface_object(interface_name, data));
    }
    interface_objects
}

fn create_network_interface_object(
    interface_name: &String,
    data: &NetworkData, 
) -> NetworkInterfaceObject {
    let interface = NetworkInterfaceObject {
        name: interface_name.to_string(),
        bytes_transmitted: data.total_transmitted(),
        bytes_recieved: data.total_received(),
        packets_transmitted: data.total_packets_transmitted(),
        packets_recieved: data.total_packets_received(),
    };
    interface
}