use serde::Serialize;
use sysinfo::{NetworkData, NetworkExt, Networks};

#[derive(Serialize)]
pub struct NetworkInterfaceObject {
    name: String,
    bytes: NetworkDataObject,
    packets: NetworkDataObject
}

#[derive(Serialize)]
struct NetworkDataObject {
    transmitted: u64,
    recieved: u64
}

pub fn create_interface_vec(networks: &Networks) -> Vec<NetworkInterfaceObject>{
    let mut interface_objects = Vec::new();

    for (interface_name, data) in networks {
        interface_objects.push(create_network_interface_object(interface_name, data));
    }
    interface_objects
}

fn create_network_data_object(transmitted: u64, recieved: u64) -> NetworkDataObject {
    NetworkDataObject {
        transmitted: transmitted,
        recieved: recieved
    }
}

fn create_network_interface_object(
    interface_name: &String,
    data: &NetworkData, 
) -> NetworkInterfaceObject {
    NetworkInterfaceObject {
        name: interface_name.to_string(),
        bytes: create_network_data_object(
            data.total_transmitted(), 
    data.total_received()
        ),
        packets: create_network_data_object(
            data.total_packets_transmitted(),
    data.total_packets_received()
        ),
    }
}