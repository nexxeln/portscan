use clap::Parser;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    port: u16,
}

fn scan_port(address: &String, port: u16) {
    let host = format!("{}:{}", address, port);
    let socket_addr = SocketAddr::from_str(host.as_str()).unwrap();

    match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(5)) {
        Ok(_) => println!("Port {} is open.", port),
        Err(_) => println!("Port {} is NOT open", port),
    }
}

fn main() {
    let args = Arguments::parse();

    scan_port(&args.address, args.port);
}
