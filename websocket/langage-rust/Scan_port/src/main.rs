use netstat::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo,
};

fn main() {
    let familles_adresses = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;

    let protocoles = ProtocolFlags::TCP;

    match get_sockets_info(familles_adresses, protocoles) {
        Ok(liste_sockets) => {

            for socket in liste_sockets {

                if let ProtocolSocketInfo::Tcp(info_tcp) = socket.protocol_socket_info {

                    if let netstat::TcpState::Listen = info_tcp.state {

                        println!("Port ouvert : {}", info_tcp.local_port);
                    }
                }
            }
        }

        Err(erreur) => {
            eprintln!("Erreur lors de la récupération des sockets : {}", erreur);
        }
    }
}