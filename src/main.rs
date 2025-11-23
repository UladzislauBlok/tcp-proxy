mod socket_channel;

use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_client((mut stream, addr): (TcpStream, SocketAddr)) -> std::io::Result<()> {
    println!("Read bytes from {addr}");
    let mut count = 0;
    loop {
        let mut tmp_buf = [0u8; 1024];
        match stream.read(&mut tmp_buf) {
            Ok(0) => {
                println!("Read 0 bytes. Will stop");
                break;
            }
            Ok(bytes) => {
                count += 1;
                println!("Read {bytes} bytes; {count}");

                println!(
                    "{}",
                    &tmp_buf[..bytes]
                        .iter()
                        .map(|b| format!("{b:02x}").to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    println!("Close connection");
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    handle_client(listener.accept()?)?;
    Ok(())
}
