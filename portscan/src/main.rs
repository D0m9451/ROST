use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn scan_port(ip: &str, port: u16) {
    let addr = format!("{}:{}", ip, port);
    if let Ok(_) = TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(1000)) {
        println!("Port {} is open! :)", port);
    }
    else {
        //println!("port {} is not open :(", port)
    }
}

fn main() {
    let target_ip = "127.0.0.1";
    let mut handles = vec![];
    println!("Loading...");
    for port in 1..65535 {
        let ip_clone = target_ip.to_string();
        let handle = thread::spawn(move || {
            scan_port(&ip_clone, port);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("DONE!");
    let _ = io::stdout().flush();
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}
