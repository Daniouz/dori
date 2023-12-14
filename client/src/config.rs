use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ClientConfiguration {
    client_name: String,
    program_name: String,
    host_address: SocketAddr,
    key: String,
}

impl ClientConfiguration {
    /// Returns this configuration as a set of command line arguments.
    pub fn to_args(&self) -> [String; 4] {
        [
            self.client_name.clone(),
            self.program_name.clone(),
            self.host_address.to_string(),
            self.key.clone(),
        ]
    }

    /// Returns the client name.
    pub fn client_name(&self) -> &str {
        &self.client_name
    }

    /// Returns the program name.
    ///
    /// Used in registries.
    pub fn program_name(&self) -> &str {
        &self.program_name
    }

    /// Returns the host's address.
    pub fn host_address(&self) -> &SocketAddr {
        &self.host_address
    }

    /// Returns the cipher key used for secure streams.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Instantiates a new ClientConfiguration.
    pub const fn new(
        client_name: String,
        program_name: String,
        host_address: SocketAddr,
        key: String,
    ) -> Self {
        Self {
            client_name,
            program_name,
            host_address,
            key,
        }
    }
}
