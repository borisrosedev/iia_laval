pub mod tcp;
pub mod tls;

fn main() {
    println!("=== TCP Three-Way Handshake ===");
    let mut client = tcp::TcpConnection::new(12345, 80);
    let mut server = tcp::TcpConnection::new(80, 12345);

    server.listen();

    if let Some(syn_segment) = client.connect() {
        if let Some(syn_ack_segment) = server.receive(syn_segment) {
            if let Some(ack_segment) = client.receive(syn_ack_segment) {
                server.receive(ack_segment);
            }
        }
    }

    println!("\n=== TLS 1.2 Handshake ===");
    let mut tls_client = tls::TlsSession::new_client();
    let mut tls_server = tls::TlsSession::new_server();

    if let Some(client_hello) = tls_client.client_hello() {
        if let Some(server_msgs) = tls_server.receive_client_hello(client_hello) {
            if let Some(client_msgs) = tls_client.receive_server_hello_done(server_msgs) {
                if let Some(server_finish_msgs) = tls_server.receive_client_key_exchange(client_msgs) {
                    tls_client.receive_server_finished(server_finish_msgs);
                }
            }
        }
    }

    println!("\nClient TLS state: {:?}", tls_client.state);
    println!("Server TLS state: {:?}", tls_server.state);
}
