use std::net::TcpListener;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("10.4.77.17:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer));
    }
}