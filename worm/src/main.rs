use std::net::TcpStream;
use std::io::Write;

fn worm(depth: u32) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
        stream.write(b"Hello from worm!").unwrap();
    }
}




fn main() {
    worm(1);
}
