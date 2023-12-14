use std::io;
use std::io::Write;
use std::net::TcpStream;

use tora::read::ToraRead;
use tora::write::ToraWrite;
use tora::{ReadEnum, ReadStruct, WriteEnum, WriteStruct};

use crate::stream::SecureTcpStream;

/// A handshake between the host and client.
#[derive(ReadStruct, WriteStruct)]
pub struct Handshake {
    client_name: String,
    key: String,
}

impl Handshake {
    pub const fn new(client_name: String, key: String) -> Self {
        Self { client_name, key }
    }
}

#[derive(Debug, ReadEnum, WriteEnum)]
pub enum HostRejectionReason {
    /// The client's name is incorrect.
    WrongClientName,

    /// The handshake could not be decrypted.
    DecryptionError,
}

/// Performs a handshake with the host and if successful, returns a secure TCP stream.
///
/// Returns None if the host rejected the connection.
pub fn perform_client_handshake(
    stream: TcpStream,
    form: Handshake,
) -> io::Result<Option<SecureTcpStream>> {
    let mut secure_stream = SecureTcpStream::new(stream, form.key);

    secure_stream.writes(&form.client_name)?;
    secure_stream.flush()?;

    Ok(secure_stream.reads::<bool>()?.then_some(secure_stream))
}

/// Performs a handshake with the client and if successful, returns a secure TCP stream.
pub fn perform_host_handshake(
    stream: TcpStream,
    form: Handshake,
) -> io::Result<Result<SecureTcpStream, HostRejectionReason>> {
    let mut secure_stream = SecureTcpStream::new(stream, form.key);

    let client_name: String = secure_stream.reads()?;

    if client_name != form.client_name {
        secure_stream.writes(&false)?;
        secure_stream.flush()?;
        return Ok(Err(HostRejectionReason::WrongClientName));
    }
    secure_stream.writes(&true)?;
    secure_stream.flush()?;
    Ok(Ok(secure_stream))
}
