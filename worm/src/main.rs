use std::net::TcpStream;
use std::io::Write;

fn worm(depth: u32) {
    if let Ok(mut stream) = TcpStream::connect("10.5.8.192:8080") {
        stream.write(b"this message was sent via tcp port: 8080").unwrap();
    }
}




fn main() {
    worm(1);
}
