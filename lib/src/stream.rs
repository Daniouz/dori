use std::io;
use std::io::{Cursor, Write};
use std::net::TcpStream;

use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use tora::read::{FromReader, ToraRead};
use tora::write::ToraWrite;

/// A write-buffered 256-bit encrypted [TcpStream].
pub struct SecureTcpStream {
    stream: TcpStream,
    crypt: MagicCrypt256,
    buf: Cursor<Vec<u8>>,
}

impl SecureTcpStream {
    /// Instantiates a new SecureTcpStream.
    pub fn new(stream: TcpStream, key: String) -> Self {
        Self {
            stream,
            crypt: MagicCrypt256::new::<_, String>(key, None),
            buf: Cursor::new(Vec::new()),
        }
    }
}

impl ToraRead for SecureTcpStream {
    fn reads<T>(&mut self) -> io::Result<T> where T: FromReader {
        let data: Vec<u8> = self.stream.reads()?;
        let bytes = self
            .crypt
            .decrypt_bytes_to_bytes(&data)
            .map_err(|_| io::ErrorKind::InvalidInput)?;

        let mut reader = Cursor::new(bytes);
        reader.reads()
    }
}

impl Write for SecureTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let bytes = self.crypt.encrypt_bytes_to_bytes(self.buf.get_ref());

        self.stream.writes(&bytes)?;
        self.stream.flush()?;

        self.buf.set_position(0);
        self.buf.get_mut().clear();
        Ok(())
    }
}
