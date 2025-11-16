use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_client((mut stream, addr): (TcpStream, SocketAddr)) -> std::io::Result<()> {
    println!("Read bytes from {addr}");
    let mut buf: Vec<u8> = vec![];
    loop {
        let mut tmp_buf = [0u8; 1024];
        match stream.read(&mut tmp_buf) {
            Ok(0) => {
                println!("Read 0 bytes. Will stop");
                break;
            }
            Ok(bytes) => {
                println!("Read {bytes} bytes");
                buf.extend_from_slice(&tmp_buf);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    let input = String::from_utf8(buf).unwrap();
    println!("{input}");
    println!("Close connection");
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    handle_client(listener.accept()?)?;
    Ok(())
}
