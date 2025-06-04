use std::net::{UdpSocket, TcpListener, IpAddr, SocketAddr,TcpStream};
use std::io::Write;
use std::str::FromStr;
use std::process::Command;
use std::io::{self, BufRead};

/*
fn send(open_ports: Vec<u16>) {
    if let Ok(mut stream) = TcpStream::connect(addr) {
        let message = format!("this message was sent via tcp port: {}", port);
        stream.write(message.as_bytes()).unwrap();
    }
}*/
/* 
fn search(depth: u32) {
    let target = "192.168.1.1";
    let port = 8080;
    let ip = IpAddr::from_str(target).expect("Invalid IP address");
    let addr = SocketAddr::new(ip, port);

    send(depth, addr, port);
}
*/
fn search(target: &str, ports: &str) -> io::Result<Vec<u16>> {
    let output = Command::new("nmap")
        .args(&["-p", ports, target])
        .output()?;

    if !output.status.success() {
        eprintln!("Nmap scan failed: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(Vec::new());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut open_ports = Vec::new();

    for line in stdout.lines() {
        if line.contains("open") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(port_proto) = parts.get(0) {
                if let Some(port_str) = port_proto.split('/').next() {
                    if let Ok(port) = port_str.parse::<u16>() {
                        open_ports.push(port);
                    }
                }
            }
        }
    }
    Ok(open_ports)
}

fn main() -> io::Result<()> {
    let target = "10.5.10.18";
    let ports = "1-8080";

    let open_ports = search(target, ports)?;
    //send(open_ports);

    println!("open ports: {:?}", open_ports);
    Ok(())
}
