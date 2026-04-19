use netstat2::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).expect("aaaaaaaaaaaaaaaaaaaaaaaaaaa"); //message d'erreur

    let mut file = File::create("log.txt").expect("Unable to create file");
    let mut line: String;


    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                line = format!("TCP {}:{} -> {}:{} {:?} - {}\n",
                    tcp_si.local_addr,
                    tcp_si.local_port,
                    tcp_si.remote_addr,
                    tcp_si.remote_port,
                    si.associated_pids,
                    tcp_si.state);
                print!("{}",line.as_str());
            }

            ProtocolSocketInfo::Udp(udp_si) => {
                line = format!("UDP {}:{} -> *:* {:?}\n",
                    udp_si.local_addr, udp_si.local_port, si.associated_pids);
                print!("{}",line.as_str());
            }
         };
        file.write_all(line.as_bytes()).expect("Unable to write to file");
    }
}
