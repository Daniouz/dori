use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::time::Instant;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use cnsl::readln;
use dori_lib::operation::{FileTransferOperation, Operation, Response};
use rand::Rng;
use crate::config::{HostConfig, load_config};

use crate::connection::ClientListener;

mod config;
mod connection;

// TODO add operation implementation

#[derive(Parser)]
#[command(name = "dori")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[command(name = "dori")]
enum Command {
    /// Create a configuration.
    Create { name: String },

    /// Runs the configuration with the given name.
    Run { name: String },

    /// Deletes a configuration.
    Delete { name: String },

    /// Generates a 256-bit cipher key.
    GenerateKey,
}

fn validate_file_dest(path: &Path) -> bool {
    if path.extension().is_none() {
        println!("Warning: The destination you entered does not have an extension.");

        loop {
            let confirm = readln!("Continue? Y/n");

            if confirm == "y" || confirm == "Y" {
                return true;
            }
            if confirm == "n" || confirm == "N" {
                return false;
            }
        }
    }
    true
}

fn run(config: &HostConfig) -> Result<()> {
    println!("Starting listener..");

    let listener = ClientListener::bind(config.bind_address()).with_context(|| "Failed to bind listener")?;
    let mut stream = listener.accept_from(config.client_name(), config.key())?;

    loop {
        let operation = readln!(">> ");

        if operation.is_empty() || operation.chars().all(char::is_whitespace) {
            continue;
        }
        
        match operation.as_str() {
            "upload" => {
                let fname = readln!("Local path to file: ");
                let dest = readln!("Client-side path: ");

                if !validate_file_dest(&PathBuf::from(&dest)) {
                    continue;
                }
                
                let data = match fs::read(&fname) {
                    Ok(v) => v,
                    Err(err) => {
                        println!("Failed to read from {fname}: {err}");
                        continue;
                    }
                };
                
                let op = FileTransferOperation::new(dest, data);
                stream.send_operation(&Operation::Upload(op))?;

                let Response::Upload(res) = stream.read_response()? else {
                    bail!("Invalid response")
                };

                if let Err(err) = res {
                    println!("Client error: {err}");
                }
            }
            "ping" => {
                let now = Instant::now();
                stream.send_operation(&Operation::Ping)?;

                let Ok(Response::Pong) = stream.read_response() else {
                    bail!("Invalid response")
                };
                println!("Ping: {:?}", now.elapsed());
            }
            _ => {
                println!("Unrecognized operation");
            }
        }
    }
}

fn generate_key() {
    let mut rng = rand::thread_rng();
    let mut bytes = Vec::with_capacity(64);

    for _ in 0..64 {
        bytes.push(rng.gen_range(1..255));
    }

    let key = String::from_utf8(bytes).unwrap();
    println!("Generated cipher key:");
    println!("{key}");
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create { name } => config::create_config(&name),
        Command::Delete { name } => config::delete_config(&name),

        Command::Run { name } => {
            let config = load_config(&name)?;

            loop {
                if let Err(err) = run(&config) {
                    println!("Error: {err}");
                    continue;
                }
                break Ok(());
            }
        },
        Command::GenerateKey => {
            generate_key();
            Ok(())
        }
    }
}
