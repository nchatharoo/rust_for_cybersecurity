use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let target_host = "192.168.1.23";
    let target_port = 80;

    let mut client = TcpStream::connect(format!("{}:{}", target_host, target_port))?;

    client.write_all(b"ABCDEF")?;

    let mut buffer = [0; 4096];
    let n = client.read(&mut buffer)?;

    println!("{}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
