use std::io;
use std::io::Write;
use std::net::{TcpListener, ToSocketAddrs};
use std::time::Duration;

use dori_lib::handshake;
use dori_lib::handshake::Handshake;
use dori_lib::operation::{Operation, Response};
use dori_lib::stream::SecureTcpStream;
use tora::read::ToraRead;
use tora::write::ToraWrite;

/// A secure connection to the client.
pub struct ClientConnection {
    stream: SecureTcpStream,
}

impl ClientConnection {
    /// Serializes and writes the given operation to the inner stream.
    /// Flushes the inner stream.
    pub fn send_operation(&mut self, operation: &Operation) -> io::Result<()> {
        self.stream.writes(operation)?;
        self.stream.flush()
    }

    /// Reads and deserializes [Response] from the stream.
    pub fn read_response(&mut self) -> io::Result<Response> {
        self.stream.reads()
    }
}

/// A listener that only establishes secure connections with the specified client.
pub struct ClientListener {
    inner: TcpListener,
}

impl ClientListener {
    /// Listens for incoming connections and attempts to establish a secure connection.
    ///
    /// Only accepts clients with the given name.
    pub fn accept_from(&self, client_name: String, key: String) -> io::Result<ClientConnection> {
        let stream = loop {
            let (conn, end_addr) = self.inner.accept()?;
            let timeout = Duration::from_secs(30);

            conn.set_read_timeout(Some(timeout))?;
            conn.set_write_timeout(Some(timeout))?;

            println!("Initiating handshake with {end_addr}..");

            let handshake = Handshake::new(client_name.clone(), key.clone());

            match handshake::perform_host_handshake(conn, handshake)? {
                Ok(stream) => break stream,
                Err(reason) => {
                    println!("Handshake failed: {reason:?}");
                    continue;
                }
            }
        };
        Ok(ClientConnection { stream })
    }

    /// Binds a listener to the given address.
    pub fn bind<A>(addr: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            inner: TcpListener::bind(addr)?,
        })
    }
}
