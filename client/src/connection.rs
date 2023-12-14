use std::io;
use std::io::Write;

use dori_lib::operation::{Operation, Response};
use dori_lib::stream::SecureTcpStream;
use tora::read::ToraRead;
use tora::write::ToraWrite;

/// A secure connection to the host.
pub struct HostConnection {
    stream: SecureTcpStream,
}

impl HostConnection {
    /// Serializes and writes the given response to the host, then flushes the stream.
    pub fn send_response(&mut self, response: &Response) -> io::Result<()> {
        self.stream.writes(response)?;
        self.stream.flush()
    }

    /// Reads and deserializes an operation from the host.
    pub fn read_operation(&mut self) -> io::Result<Operation> {
        self.stream.reads()
    }

    /// Instantiates a new HostConnection.
    pub const fn new(stream: SecureTcpStream) -> Self {
        Self { stream }
    }
}
