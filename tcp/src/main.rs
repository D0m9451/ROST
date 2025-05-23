use std::net::{UdpSocket, TcpListener, IpAddr, SocketAddr};
use std::io::Read;
use std::net::TcpStream;
use std::io::Write;

fn tcp(local_ip: IpAddr) {
    let addr = SocketAddr::new(local_ip, 8080);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer));
    }
}

fn search() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();
    let local_ip = socket.local_addr().unwrap().ip();

    println!("My local IP is: {}", local_ip);
    tcp(local_ip)

}

fn main() {
    search()
}