use crate::tcp::TcpState::SynReceived;

const FLAG_SYN: u8 = 0x02;
const FLAG_ACK: u8 = 0x10;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    LastAck,
    TimeWait,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TcpSegment {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    data_offset: u8,
    flags: u8,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Vec<u8>,
    payload: Vec<u8>,
}

impl TcpSegment {
    pub fn new(
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: u8,
        payload: Vec<u8>,
    ) -> Self {
        TcpSegment {
            source_port,
            destination_port,
            sequence_number,
            acknowledgment_number,
            data_offset: 5,
            flags,
            window_size: 65535,
            checksum: 0,
            urgent_pointer: 0,
            options: Vec::new(),
            payload,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TcpConnection {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    state: TcpState,
}

impl TcpConnection {
    pub fn new(source_port: u16, destination_port: u16) -> Self {
        TcpConnection {
            source_port,
            destination_port,
            sequence_number: 0,
            acknowledgment_number: 0,
            state: TcpState::Closed,
        }
    }
    pub fn connect(&mut self) -> Option<TcpSegment> {
        if self.state != TcpState::Closed {
            println!("Connection is not in CLOSED state, cannot connect.");
            return None;
        }

        // sends initial SYN segment to initiate connection
        println!("{} Initiating connection to {} with SYN...", self.source_port, self.destination_port);
        self.state = TcpState::SynSent;
        let syn_segment = TcpSegment::new(
            self.source_port,
            self.destination_port,
            self.sequence_number,
            0,
            FLAG_SYN,
            Vec::new(),
        );

        Some(syn_segment)
    }

    pub fn listen(&mut self) {
        // listens for incoming connections, waiting for SYN segments
        if self.state != TcpState::Closed {
            println!("Connection is not in CLOSED state, cannot listen.");
            return;
        }

        self.state = TcpState::Listen;
        println!("Listening for incoming connections...");
    }

    pub fn receive(&mut self, segment: TcpSegment) -> Option<TcpSegment> {
        match self.state {
            // handles incoming SYN segment in LISTEN state, responds with SYN-ACK
            TcpState::Listen if ((segment.flags & FLAG_SYN) != 0) => {
                println!("{} Received SYN, sending SYN-ACK to {}...", self.source_port, self.destination_port);
                self.state = SynReceived;
                self.acknowledgment_number = self.sequence_number + 1;
                let syn_ack_segment = TcpSegment::new(
                    self.source_port,
                    self.destination_port,
                    self.sequence_number,
                    self.acknowledgment_number,
                    FLAG_SYN | FLAG_ACK,
                    Vec::new(),
                );

                Some(syn_ack_segment)
            }

            // handles incoming SYN-ACK segment in SYN-SENT state, responds with ACK and transitions to ESTABLISHED
            TcpState::SynSent if ((segment.flags & (FLAG_SYN | FLAG_ACK)) == (FLAG_SYN | FLAG_ACK)) => {
                println!("{} Received SYN-ACK, sending ACK to {}...", self.source_port, self.destination_port);
                self.state = TcpState::Established;
                self.acknowledgment_number = segment.sequence_number + 1;
                let ack_segment = TcpSegment::new(
                    self.source_port,
                    self.destination_port,
                    self.sequence_number,
                    self.acknowledgment_number,
                    FLAG_ACK,
                    Vec::new(),
                );

                Some(ack_segment)
            }

            // handles incoming ACK segment in SYN-RECEIVED state, transitions to ESTABLISHED
            TcpState::SynReceived if ((segment.flags & FLAG_ACK) != 0) => {
                println!("{} Received ACK, connection established with {}.", self.source_port, self.destination_port);
                self.state = TcpState::Established;
                None
            }

            _ => {None}
        }
    }
}


