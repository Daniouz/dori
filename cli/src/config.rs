use std::net::SocketAddr;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HostConfig {
    bind_address: SocketAddr,
    client_name: String,
    key: String,
}

impl HostConfig {
    /// Returns the address the host will bind to.
    pub fn bind_address(&self) -> SocketAddr {
        self.bind_address
    }

    /// Returns the client's name.
    pub fn client_name(&self) -> &str {
        &self.client_name
    }

    /// Returns the cipher key used for secure streams.
    pub fn key(&self) -> &str {
        &self.key
    }
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            bind_address: SocketAddr::from(([127, 0, 0, 1], 12700)),
            client_name: "my-client".to_string(),
            key: "<- Enter cipher key here ->".to_string(),
        }
    }
}

/// Creates a configuration file with the given client/file name.
///
/// Creates the file in `config/<NAME>.toml`
pub fn create_config(name: &str) -> Result<()> {
    let config = HostConfig::default();
    let content = toml::to_string_pretty(&config)
        .with_context(|| "Failed to serialize default configuration")?;

    fs::write(format!("config/{name}.toml"), content)
        .with_context(|| "Failed to write to configuration file")
}

/// Deletes the configuration file with the given name.
///
/// Looks for the file in `config/<NAME>.toml`
pub fn delete_config(name: &str) -> Result<()> {
    fs::remove_file(format!("config/{name}.toml"))
        .with_context(|| "Failed to delete configuration file")
}

/// Reads and deserializes the configuration with the given name.
///
/// Looks for the configuration file in `config/<NAME>.toml`
pub fn load_config(name: &str) -> Result<HostConfig> {
    let mut path = PathBuf::from(
        env::current_exe()?
            .parent()
            .with_context(|| "Failed to get parent directory of dori-cli executable")?,
    );

    path.push(format!("config/{name}.toml"));

    let txt = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read from {}", path.display()))?;

    toml::from_str(&txt).with_context(|| "Failed to deserialize configuration")
}
