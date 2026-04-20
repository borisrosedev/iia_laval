use port_check::*;
use std::net::*;
use std::time::Duration;

// --------------------------------------------------------------------
// If not specified, all port checks are performed for IPv4 addresses.
// --------------------------------------------------------------------

// get a free local port
let free_port = free_local_port().unwrap();

// get a free local port between 10000 and 15000
let free_port_in_range = free_local_port_in_range(10000..=15000);

// check whether a remote port is reachable
let is_reachable = is_port_reachable("192.0.2.0:8080");
// or
let is_reachable = is_port_reachable_with_timeout("192.0.2.0:8080", Duration::from_millis(10_000));



// --------------------------------------------------------------------
// IPv6 checks are supported too
// --------------------------------------------------------------------

let free_ipv6_port = free_local_ipv6_port().unwrap();

let is_ipv6_port_free = is_local_port_free(Port::ipv6(free_ipv6_port));
// or
let is_ipv6_port_free = is_local_ipv6_port_free(free_ipv6_port);



// --------------------------------------------------------------------
// Retry-based approach
//
// The library provides a retry-based approach via the with_free_port(...) function
// to help mitigate TOC/TOU race conditions.
//
// --------------------------------------------------------------------


let (server, port) = with_free_port::<_, std::io::Error, _>(|port| {
    // Trying to use the port. If it fails, we try again with another port.
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
    Ok(listener)
}).unwrap();

let (server, port) = with_free_ipv4_port::<_, std::io::Error, _>(|port| {
    // Trying to use the port. If it fails, we try again with another port.
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
    Ok(listener)
}).unwrap();


let (server, port) = with_free_ipv6_port::<_, std::io::Error, _>(|port| {
    // Trying to use the port. If it fails, we try again with another port.
    let listener = TcpListener::bind(SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0))?;
    Ok(listener)
}).unwrap();