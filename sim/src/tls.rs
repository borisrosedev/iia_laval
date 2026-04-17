#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TlsState {
    Idle,
    ClientHelloSent,
    ServerHelloDone,
    KeyExchangeSent,
    ChangeCipherSpecSent,
    Established,
}

#[derive(Debug, Clone)]
pub enum TlsHandshakeType {
    ClientHello,
    ServerHello,
    Certificate,
    ServerHelloDone,
    ClientKeyExchange,
    ChangeCipherSpec,
    Finished,
}

#[derive(Debug, Clone)]
pub struct TlsMessage {
    pub handshake_type: TlsHandshakeType,
    pub payload: Vec<u8>,
}

impl TlsMessage {
    pub fn new(handshake_type: TlsHandshakeType, payload: Vec<u8>) -> Self {
        TlsMessage { handshake_type, payload }
    }
}

#[derive(Debug)]
pub struct TlsSession {
    pub role: &'static str,
    pub state: TlsState,
}

impl TlsSession {
    pub fn new_client() -> Self {
        TlsSession { role: "Client", state: TlsState::Idle }
    }

    pub fn new_server() -> Self {
        TlsSession { role: "Server", state: TlsState::Idle }
    }

    /// Client initiates the handshake by sending ClientHello
    pub fn client_hello(&mut self) -> Option<TlsMessage> {
        if self.state != TlsState::Idle {
            println!("[{}] Not in Idle state, cannot send ClientHello.", self.role);
            return None;
        }
        println!("[{}] Sending ClientHello...", self.role);
        self.state = TlsState::ClientHelloSent;
        Some(TlsMessage::new(TlsHandshakeType::ClientHello, b"supported_ciphers:TLS_RSA_WITH_AES_128_CBC_SHA".to_vec()))
    }

    /// Server receives ClientHello and responds with ServerHello + Certificate + ServerHelloDone
    pub fn receive_client_hello(&mut self, msg: TlsMessage) -> Option<Vec<TlsMessage>> {
        if self.state != TlsState::Idle {
            println!("[{}] Unexpected ClientHello in state {:?}.", self.role, self.state);
            return None;
        }
        if !matches!(msg.handshake_type, TlsHandshakeType::ClientHello) {
            return None;
        }
        println!("[{}] Received ClientHello. Sending ServerHello, Certificate, ServerHelloDone...", self.role);
        self.state = TlsState::ServerHelloDone;
        Some(vec![
            TlsMessage::new(TlsHandshakeType::ServerHello,     b"chosen_cipher:TLS_RSA_WITH_AES_128_CBC_SHA".to_vec()),
            TlsMessage::new(TlsHandshakeType::Certificate,     b"server_certificate_data".to_vec()),
            TlsMessage::new(TlsHandshakeType::ServerHelloDone, Vec::new()),
        ])
    }

    /// Client receives ServerHello + Certificate + ServerHelloDone, responds with
    /// ClientKeyExchange + ChangeCipherSpec + Finished
    pub fn receive_server_hello_done(&mut self, msgs: Vec<TlsMessage>) -> Option<Vec<TlsMessage>> {
        if self.state != TlsState::ClientHelloSent {
            println!("[{}] Unexpected server messages in state {:?}.", self.role, self.state);
            return None;
        }
        let has_server_hello_done = msgs.iter().any(|m| matches!(m.handshake_type, TlsHandshakeType::ServerHelloDone));
        if !has_server_hello_done {
            println!("[{}] Did not receive ServerHelloDone.", self.role);
            return None;
        }
        println!("[{}] Received ServerHello + Certificate + ServerHelloDone.", self.role);
        println!("[{}] Sending ClientKeyExchange, ChangeCipherSpec, Finished...", self.role);
        self.state = TlsState::KeyExchangeSent;
        Some(vec![
            TlsMessage::new(TlsHandshakeType::ClientKeyExchange, b"encrypted_pre_master_secret".to_vec()),
            TlsMessage::new(TlsHandshakeType::ChangeCipherSpec,  b"1".to_vec()),
            TlsMessage::new(TlsHandshakeType::Finished,          b"client_verify_data".to_vec()),
        ])
    }

    /// Server receives ClientKeyExchange + ChangeCipherSpec + Finished,
    /// responds with its own ChangeCipherSpec + Finished and transitions to Established
    pub fn receive_client_key_exchange(&mut self, msgs: Vec<TlsMessage>) -> Option<Vec<TlsMessage>> {
        if self.state != TlsState::ServerHelloDone {
            println!("[{}] Unexpected client key exchange in state {:?}.", self.role, self.state);
            return None;
        }
        let has_finished = msgs.iter().any(|m| matches!(m.handshake_type, TlsHandshakeType::Finished));
        if !has_finished {
            println!("[{}] Did not receive client Finished.", self.role);
            return None;
        }
        println!("[{}] Received ClientKeyExchange + ChangeCipherSpec + Finished.", self.role);
        println!("[{}] Sending ChangeCipherSpec + Finished. Connection established.", self.role);
        self.state = TlsState::Established;
        Some(vec![
            TlsMessage::new(TlsHandshakeType::ChangeCipherSpec, b"1".to_vec()),
            TlsMessage::new(TlsHandshakeType::Finished,         b"server_verify_data".to_vec()),
        ])
    }

    /// Client receives server's ChangeCipherSpec + Finished, transitions to Established
    pub fn receive_server_finished(&mut self, msgs: Vec<TlsMessage>) -> bool {
        if self.state != TlsState::KeyExchangeSent {
            println!("[{}] Unexpected server Finished in state {:?}.", self.role, self.state);
            return false;
        }
        let has_finished = msgs.iter().any(|m| matches!(m.handshake_type, TlsHandshakeType::Finished));
        if !has_finished {
            println!("[{}] Did not receive server Finished.", self.role);
            return false;
        }
        println!("[{}] Received server ChangeCipherSpec + Finished. TLS session established.", self.role);
        self.state = TlsState::Established;
        true
    }
}

