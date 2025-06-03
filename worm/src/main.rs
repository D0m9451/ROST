use std::net::{UdpSocket, TcpListener, IpAddr, SocketAddr,TcpStream};
use std::io::Write;
use std::str::FromStr;
use std::process::Command;
use std::io::{self, BufRead};


fn send(depth: u32, addr:SocketAddr, port: u16) {
    if let Ok(mut stream) = TcpStream::connect(addr) {
        stream.write(b"this message was sent via tcp port: {port}").unwrap();
    }
}

fn search(depth: u32) {
    let target = "10.4.106.49";
    let port = 8080;
    let ip = IpAddr::from_str(target).expect("Invalid IP address");
    let addr = SocketAddr::new(ip, port);

    send(depth, addr, port);
}


fn main() {
    search(1);
}
