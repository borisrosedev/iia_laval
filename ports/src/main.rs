use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

// adapted from netstat2 example @ https://docs.rs/netstat2/latest/netstat2/
fn main() {
    const PROTO_W: usize = 5;
    const LPORT_W: usize = 11;
    const RHOST_W: usize = 39;
    const RPORT_W: usize = 11;
    const STATE_W: usize = 12;

    let header = format!(
        "{:<PROTO_W$} {:<LPORT_W$} {:<RHOST_W$} {:<RPORT_W$} {:<STATE_W$}",
        "PROTO", "LOCAL_PORT", "REMOTE_HOST", "REMOTE_PORT", "STATE"
    );
    let separator = "-".repeat(PROTO_W + LPORT_W + RHOST_W + RPORT_W + STATE_W + 4);


    let file = File::create("connections.log").unwrap_or_else(|err| {
        eprintln!("Error creating log file: {}", err);
        std::process::exit(1);
    });
    let mut log_file = BufWriter::new(file);
    writeln!(log_file, "{}", header).unwrap();
    writeln!(log_file, "{}", separator).unwrap();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap_or_else(|err| {
        eprintln!("Error getting socket information: {}", err);
        std::process::exit(1);
    });

    let mut line: String;
    for info in sockets_info {
        match info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => {
                line = format!(
                    "{:<PROTO_W$} {:<LPORT_W$} {:<RHOST_W$} {:<RPORT_W$} {:<STATE_W$}",
                    "TCP",
                    tcp_info.local_port,
                    tcp_info.remote_addr,
                    tcp_info.remote_port,
                    format!("{:?}", tcp_info.state)
                );
            }
            ProtocolSocketInfo::Udp(udp_info) => {
                line = format!(
                    "{:<PROTO_W$} {:<LPORT_W$} {:<RHOST_W$} {:<RPORT_W$} {:<STATE_W$}",
                    "UDP",
                    udp_info.local_port,
                    "-",
                    "-",
                    "-"
                );
            }
        }
        println!("{}", line);
        writeln!(log_file, "{}", line).unwrap();
    }
}
