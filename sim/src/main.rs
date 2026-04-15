pub mod tcp;

fn main() {
    let mut client = tcp::TcpConnection::new(12345, 80);
    let mut server = tcp::TcpConnection::new(80, 12345);

    server.listen();
    
    // client sends SYN to initiate connection
    if let Some(syn_segment) = client.connect() {
        // server receives SYN and responds with SYN-ACK
        if let Some(syn_ack_segment) =server.receive(syn_segment) {
            // client receives SYN-ACK and responds with ACK
            if let Some(ack_segment) = client.receive(syn_ack_segment) {
                // server receives ACK and establishes connection
                server.receive(ack_segment);
            }
        }
    }
}
