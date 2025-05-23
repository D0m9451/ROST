use std::net::TcpStream;
use std::io::Write;

fn worm(depth: u32) {
    if let Ok(mut stream) = TcpStream::connect("10.4.77.17:8080") {
        stream.write(b"this message was sent via tcp").unwrap();
    }
}




fn main() {
    worm(1);
}
