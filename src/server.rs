use std::net::TcpStream;
use std::io::{Read, BufRead, BufReader};
use std::fs::File;

fn get_server_addr() -> std::io::Result<String> {
    let file = File::open("config.prop")?;
    let mut buf_reader = BufReader::new(file);
    let mut addr = String::new();
    buf_reader.read_line(&mut addr)?;
    Ok(addr.trim().to_string())
}

fn main() {
    let addr = get_server_addr().expect("Could not read config.prop");
    let mut stream = TcpStream::connect(addr).expect("Could not connect to server");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..]);
    println!("Message from client: {}", message);
}
