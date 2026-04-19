use netstat2::*;
use std::{thread, time::Duration};

fn main() {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();

    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => println!(
                "TCP {}:{} -> {}:{} {:?} - {}",
                tcp_si.local_addr,
                tcp_si.local_port,
                tcp_si.remote_addr,
                tcp_si.remote_port,
                si.associated_pids,
                tcp_si.state
            ),
            ProtocolSocketInfo::Udp(udp_si) => println!(
                "UDP {}:{} -> *:* {:?}",
                udp_si.local_addr, udp_si.local_port, si.associated_pids
            ),
        }
    }

    // Simulation de connexion TCP

    let mut client = Client::new();
    let mut server = Server::new();

    client.connect(&mut server);

    // Simulation de connexion TLS

    let mut clientTLS = ClientTLS::new();
    let mut serverTLS = ServerTLS::new();

    clientTLS.start_handshake(&mut serverTLS);
}

#[derive(Debug)]
enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
}

struct Client {
    state: TcpState,
}

struct Server {
    state: TcpState,
}

impl Client {
    fn new() -> Self {
        Self { state: TcpState::Closed }
    }

    fn connect(&mut self, server: &mut Server) {
        println!("CLIENT: Etat initial: {:?}", self.state);

        // Envoi SYN
        self.state = TcpState::SynSent;
        println!("CLIENT -> SERVER: SYN");
        thread::sleep(Duration::from_millis(500));

        server.receive_syn();

        // Attente SYN-ACK
        if let TcpState::SynReceived = server.state {
            println!("CLIENT <- SERVER: SYN-ACK");

            // Envoi ACK
            println!("CLIENT -> SERVER: ACK");
            self.state = TcpState::Established;
            thread::sleep(Duration::from_millis(500));

            server.receive_ack();
        }

        println!("CLIENT: Etat final: {:?}", self.state);
    }
}

impl Server {
    fn new() -> Self {
        Self { state: TcpState::Listen }
    }

    fn receive_syn(&mut self) {
        println!("SERVER: Etat initial: {:?}", self.state);

        println!("SERVER <- CLIENT: SYN");
        self.state = TcpState::SynReceived;
        thread::sleep(Duration::from_millis(500));

        println!("SERVER -> CLIENT: SYN-ACK");
    }

    fn receive_ack(&mut self) {
        println!("SERVER <- CLIENT: ACK");
        self.state = TcpState::Established;

        println!("SERVER: Etat final: {:?}", self.state);
    }
}

#[derive(Debug)]
enum TlsState {
    Init,
    ClientHelloSent,
    ServerHelloReceived,
    CertificateReceived,
    KeyExchange,
    Finished,
    Established,
}

struct ClientTLS {
    state: TlsState,
}

struct ServerTLS {
    state: TlsState,
}

impl ClientTLS {
    fn new() -> Self {
        Self { state: TlsState::Init }
    }

    fn start_handshake(&mut self, server: &mut ServerTLS) {
        println!("CLIENT: Etat initial: {:?}", self.state);

        // ClientHello
        self.state = TlsState::ClientHelloSent;
        println!("CLIENT -> SERVER: ClientHello");
        thread::sleep(Duration::from_millis(500));

        server.receive_client_hello();

        // ServerHello + Certificate
        println!("CLIENT <- SERVER: ServerHello");
        self.state = TlsState::ServerHelloReceived;

        println!("CLIENT <- SERVER: Certificate");
        self.state = TlsState::CertificateReceived;
        thread::sleep(Duration::from_millis(500));

        // Key exchange
        println!("CLIENT -> SERVER: ClientKeyExchange");
        self.state = TlsState::KeyExchange;
        thread::sleep(Duration::from_millis(500));

        server.receive_key_exchange();

        // Finished
        println!("CLIENT -> SERVER: Finished");
        thread::sleep(Duration::from_millis(500));

        println!("CLIENT <- SERVER: Finished");
        self.state = TlsState::Established;

        println!("CLIENT: Etat final: {:?}", self.state);
    }
}

impl ServerTLS {
    fn new() -> Self {
        Self { state: TlsState::Init }
    }

    fn receive_client_hello(&mut self) {
        println!("SERVER: Etat initial: {:?}", self.state);

        println!("SERVER <- CLIENT: ClientHello");

        // Réponse serveur
        println!("SERVER -> CLIENT: ServerHello");
        println!("SERVER -> CLIENT: Certificate");

        self.state = TlsState::ServerHelloReceived;
    }

    fn receive_key_exchange(&mut self) {
        println!("SERVER <- CLIENT: ClientKeyExchange");

        println!("SERVER -> CLIENT: Finished");
        self.state = TlsState::Established;

        println!("SERVER: Etat final: {:?}", self.state);
    }
}