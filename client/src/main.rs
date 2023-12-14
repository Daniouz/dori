use std::net::{SocketAddr, TcpStream};
use std::{env, io};

use dori_client::config::ClientConfiguration;
use dori_lib::handshake;
use dori_lib::handshake::Handshake;
use dori_lib::operation::{Operation, Response};

use crate::connection::HostConnection;

mod connection;
mod errors;

fn parse_arg_config() -> Result<ClientConfiguration, i32> {
    let mut args = env::args().skip(1);

    let client_name = args.next().ok_or(errors::MISSING_CLIENT_NAME)?;
    let program_name = args.next().ok_or(errors::MISSING_PROGRAM_NAME)?;
    let host_address = args.next().ok_or(errors::MISSING_HOST_ADDRESS)?;

    let host_address: SocketAddr = host_address
        .parse()
        .map_err(|_| errors::INVALID_HOST_ADDRESS)?;

    let key = args.next().ok_or(errors::MISSING_KEY)?;

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
        // TODO implement responses for all operations
        let response = match conn.read_operation()? {
            Operation::Ping => Response::Pong,
            _ => Response::Pong,
        };
        conn.send_response(&response)?;
    }
}

fn main() -> Result<(), i32> {
    let config = parse_arg_config()?;

    loop {
        let _ = run(&config);
    }
}
