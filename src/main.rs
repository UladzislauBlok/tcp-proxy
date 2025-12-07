mod socket_channel;

use log::{error, info, trace};
use std::thread;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(
    mut client: &TcpStream,
    client_addr: &str,
    mut server: &TcpStream,
    server_addr: &str,
) {
    loop {
        let mut buf_req = [0u8; 8192];
        match client.read(&mut buf_req) {
            Ok(0) => {
                info!(
                    "Client {client_addr} has closed connection. Close server connection {server_addr}"
                );
                break;
            }
            Ok(bytes) => {
                info!("Forward {bytes} bytes from {client_addr} to {server_addr}");
                trace!(
                    "{}",
                    &buf_req[..bytes]
                        .iter()
                        .map(|b| format!("{b:02x}").to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                match server.write(&buf_req[..bytes]) {
                    Ok(__) => {}
                    Err(__) => {}
                }
            }
            Err(e) => {
                error!("Unexpected error while reading from {client_addr} : {e}");
            }
        };

        let mut buf_resp = [0u8; 8192];
        match server.read(&mut buf_resp) {
            Ok(0) => {
                info!(
                    "Server {server_addr} has closed connection. Close client connection {client_addr}"
                );
                break;
            }
            Ok(bytes) => {
                info!("Forward {bytes} bytes from {server_addr} to {client_addr}");
                trace!(
                    "{}",
                    &buf_req[..bytes]
                        .iter()
                        .map(|b| format!("{b:02x}").to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                match client.write(&buf_resp[..bytes]) {
                    Ok(__) => {}
                    Err(__) => {}
                }
            }
            Err(e) => {
                error!("Unexpected error while reading from {server_addr} : {e}");
            }
        }
    }
}

fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .init();
}

fn main() -> std::io::Result<()> {
    init_logger();
    info!("Start proxy");
    let server_addr = "127.0.0.1:8086";
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    let (client, client_addr) = listener.accept()?;
    let client_addr_str = client_addr.to_string();
    let server = TcpStream::connect(server_addr)?;
    info!("Start client - server connection");
    thread::spawn(move || {
        handle_client(&client, &client_addr_str, &server, server_addr);
    })
    .join();
    Ok(())
}
