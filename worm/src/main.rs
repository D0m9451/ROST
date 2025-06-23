use std::net::{UdpSocket, TcpListener, IpAddr, SocketAddr,TcpStream};
use std::io::Write;
use std::str::FromStr;
use std::process::Command;
use std::io::{self, BufRead};



fn payload() {
    
}


fn send(open_ports: Vec<u16>, target: &str) {
    let ip = IpAddr::from_str(target).expect("Invalid IP address");
    for port in open_ports {
        let addr = SocketAddr::new(ip, port);
        println!("sending...");
        
        if let Ok(mut stream) = TcpStream::connect(addr) {
            let message = format!("Hello! :D I am a Worm!! your port num: {:?} is vulnerable! :) bye!", port);
            stream.write(message.as_bytes()).unwrap();

            if let Err(e) = stream.write_all(message.as_bytes()) {
                eprintln!("Failed to send message to {}: {}", addr, e);
            } else {
                println!("Message sent to {}", addr);
            }
        } else {
            eprintln!("Failed to connect to {}", addr);
        }
    }
}

fn ipsearch() {

}


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
                        println!("{:?}", open_ports);
                    }
                }
            }
        }
    }
    Ok(open_ports)
}

fn main() -> io::Result<()> {
    let target = "10.5.8.170";
    let ports = "1-65535";
    println!("scanning: {}", target);

    let open_ports = search(target, ports)?;
    println!("scan complete!");
    println!("open ports: {:?}", open_ports);
    

    send(open_ports, target);

    Ok(())
}
