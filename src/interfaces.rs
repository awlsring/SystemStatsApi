use std::collections::HashMap;

use network_interface::Addr;
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use serde::Serialize;
use sysinfo::{NetworkData, NetworkExt, Networks};
use systemstat::Ipv4Addr;
use systemstat::Ipv6Addr;

#[derive(Serialize)]
pub struct NetworkInterfaceObject {
    name: String,
    version: String,
    ip: String,
    broadcast: String,
    netmask: String,
    bytes: NetworkDataObject,
    packets: NetworkDataObject
}

#[derive(Serialize)]
struct NetworkDataObject {
    transmitted: u64,
    recieved: u64
}

pub fn create_interface_vec(networks: &Networks) -> Vec<NetworkInterfaceObject>{
    let network_interfaces = NetworkInterface::show().unwrap();
    let mut interface_objects = Vec::new();
    let mut interface_data = HashMap::new();

    for (interface_name, data) in networks {
        interface_data.insert(interface_name, interface_transmission_data(data));
    }

    for interface in network_interfaces.iter() {

        if interface.name == "lo" {
            continue;
        }

        let name = &interface.name;
        let ip_info = get_interface_ip(interface.addr);
        let data_objects = create_interface_data_object(&name, &interface_data);

        interface_objects.push(
            NetworkInterfaceObject { 
                name: name.to_string(),
                version: ip_info.0, 
                ip: ip_info.1, 
                broadcast: ip_info.2, 
                netmask: ip_info.3,
                bytes: data_objects.0,
                packets: data_objects.1
            }
        );
    }
    interface_objects
}

pub fn get_primary_interface() -> String{
    let network_interfaces = NetworkInterface::show().unwrap();
    for interface in network_interfaces.iter() {
        if interface.name == "lo" {
            continue;
        }
        let ip_info = get_interface_ip(interface.addr);
        return ip_info.1
    }
    panic!("No interface was found")

}

fn interface_transmission_data(data: &NetworkData) -> (u64, u64, u64, u64) {
    (
        data.total_transmitted(), 
        data.total_received(), 
        data.total_packets_transmitted(),
        data.total_packets_received()
    )
}

fn create_interface_data_object(
    name: &String,
    map: &HashMap<&String, (u64, u64, u64, u64)>
) -> (NetworkDataObject, NetworkDataObject) {
    match map.get(name) {
        Some(s) => {
            (
                NetworkDataObject {
                    transmitted: s.0,
                    recieved: s.1
                },
                NetworkDataObject {
                    transmitted: s.2,
                    recieved: s.3
                }
            )
        }
        None => {
            (
                NetworkDataObject {
                    transmitted: 0,
                    recieved: 0
                },
                NetworkDataObject {
                    transmitted: 0,
                    recieved: 0
                }
            )
        },
    }
}

fn get_interface_ip(address: Option<Addr>) -> (String, String, String, String) {
    match address {
        Some(s) => {
            match s {
                Addr::V4(add) => {
                    ( 
                        "v4".to_string(),
                        add.ip.to_string(), 
                        handle_optional_ipv4(add.broadcast), 
                        handle_optional_ipv4(add.netmask)
                    )
                },
                Addr::V6(add) => {
                    (
                        "v6".to_string(),
                        add.ip.to_string(), 
                        handle_optional_ipv6(add.broadcast), 
                        handle_optional_ipv6(add.netmask)
                    )
                },
            }
        },
        None => {
            let u = "Unknown";
            (u.to_string(), u.to_string(), u.to_string(), u.to_string())
        }
    }
}

fn handle_optional_ipv4(option: Option<Ipv4Addr>) -> String {
    match option {
        Some(s) => s.to_string(),
        None => "None".to_string()
    }
}

fn handle_optional_ipv6(option: Option<Ipv6Addr>) -> String {
    match option {
        Some(s) => s.to_string(),
        None => "None".to_string()
    }
}