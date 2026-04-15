use netstat::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo,
};

fn main() {
    // On veut regarder IPv4 + IPv6
    let familles_adresses = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;

    // On s'intéresse uniquement aux connexions TCP
    let protocoles = ProtocolFlags::TCP;

    // On récupère les informations des sockets
    match get_sockets_info(familles_adresses, protocoles) {
        Ok(liste_sockets) => {

            for socket in liste_sockets {

                if let ProtocolSocketInfo::Tcp(info_tcp) = socket.protocol_socket_info {

                    if let netstat::TcpState::Listen = info_tcp.state {

                        // On affiche le port ouvert
                        println!("Port ouvert : {}", info_tcp.local_port);
                    }
                }
            }
        }

        // En cas d’erreur
        Err(erreur) => {
            eprintln!("Erreur lors de la récupération des sockets : {}", erreur);
        }
    }
}