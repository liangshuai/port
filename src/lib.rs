use std::net::{Ipv4Addr, TcpListener, UdpSocket};
use std::str::FromStr;
pub type Port = u16;


fn is_tcp_port_available(host: &str, p: Port) -> bool {
    matches!(
        TcpListener::bind((Ipv4Addr::from_str(host).unwrap(), p)).is_ok(),
        true
    ) 
}

fn is_udp_port_available(host: &str, p: Port) -> bool {
    matches!(
        UdpSocket::bind((Ipv4Addr::from_str(host).unwrap(), p)).is_ok(),
        true
    )
}

pub fn check_port(host: &str, port: Port) -> bool {
    is_tcp_port_available(host, port) && is_udp_port_available(host, port)
}


#[cfg(test)]
mod tests {
    use crate::check_port;
    #[test]
    fn test_is_free() {
        assert!(check_port("127.0.0.1", 32200));
    }
}