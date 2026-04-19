use netstat::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState,
};
use std::collections::BTreeMap;

fn main() {
    println!("Analyse des ports réseau ouverts...\n");

    let flags_adresses = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let flags_protocoles = ProtocolFlags::TCP;

    let sockets = match get_sockets_info(flags_adresses, flags_protocoles) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Impossible d'accéder aux informations réseau : {}", e);
            return;
        }
    };

    let mut ports_ouverts: BTreeMap<String, Vec<u16>> = BTreeMap::new();

    for socket in sockets {
        if let ProtocolSocketInfo::Tcp(tcp) = socket.protocol_socket_info {
            if tcp.state == TcpState::Listen {
                let ip = tcp.local_addr.to_string();
                ports_ouverts
                    .entry(ip)
                    .or_insert_with(Vec::new)
                    .push(tcp.local_port);
            }
        }
    }

    if ports_ouverts.is_empty() {
        println!("Aucun port TCP ouvert détecté.");
        return;
    }

    println!("Ports TCP ouverts détectés :\n");

    for (adresse, ports) in ports_ouverts {
        println!("Adresse {}", adresse);
        for port in ports {
            println!("  └─ Port {}", port);
        }
        println!();
    }

    println!("Analyse terminée.");
}