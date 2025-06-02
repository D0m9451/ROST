use std::net::{UdpSocket, TcpListener, IpAddr, SocketAddr};
use std::io::Write;

fn send(depth: u32, addr:IpAddr) {
    if let Ok(mut stream) = TcpStream::connect(addr) {
        stream.write(b"this message was sent via tcp port: 8080").unwrap();
    }
}

fn search(depth: u32) {
    let target = "10.5.8.183";
    let addr = SocketAddr::new(target:IpAddr, 8080);
    send(depth: u32, addr)

}


fn main() {
    search(1);
}
