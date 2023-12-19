#![windows_subsystem = "windows"]

use std::net::{SocketAddr, TcpStream};
use std::{env, fs, io};

use dori_client::config::ClientConfiguration;
use dori_lib::handshake;
use dori_lib::handshake::Handshake;
use dori_lib::operation::{Operation, Response};

use crate::connection::HostConnection;

mod connection;

fn parse_arg_config() -> Result<ClientConfiguration, &'static str> {
    let mut args = env::args().skip(1);

    let client_name = args.next().ok_or("Missing client name")?;
    let program_name = args.next().ok_or("Missing program name")?;
    let host_address = args.next().ok_or("Missing host address")?;

    let host_address: SocketAddr = host_address
        .parse()
        .map_err(|_| "Invalid host address")?;

    let key = args.next().ok_or("Missing key")?;

    Ok(ClientConfiguration::new(
        client_name,
        program_name,
        host_address,
        key,
    ))
}

fn run(config: &ClientConfiguration) -> io::Result<()> {
    let stream = TcpStream::connect(config.host_address())?;
    let handshake = Handshake::new(config.client_name().to_string(), config.key().to_string());

    let stream = handshake::perform_client_handshake(stream, handshake)?
        .ok_or(io::ErrorKind::ConnectionRefused)?;

    let mut conn = HostConnection::new(stream);

    loop {
        let response = match conn.read_operation()? {
            Operation::Upload(op) => {
                let res = fs::write(op.path(), op.content()).map_err(|e| e.to_string());
                Response::Upload(res)
            }
            Operation::Ping => Response::Pong,
            _ => return Err(io::ErrorKind::Unsupported.into())
        };
        conn.send_response(&response)?;
    }
}

fn main() {
    let config = match parse_arg_config() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Program error: {err}");
            return;
        }
    };

    loop {
        let err = run(&config);

        #[cfg(debug_assertions)]
        println!("Error: {err:#?}");
    }
}
