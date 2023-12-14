use std::io;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dori_lib::operation::{Operation, Response};
use rand::Rng;

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

fn run(config_name: &str) -> Result<()> {
    let config = config::load_config(config_name)?;
    let listener = ClientListener::bind(config.bind_address)?;

    let mut stream = listener.accept_from(config.client_name, config.key)?;

    if let Err(err) = stream.send_operation(&Operation::Ping) {
        println!("{err}");
    }

    let response = stream.read_response()?;
    assert!(matches!(response, Response::Pong));

    Ok(())
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
        Command::Run { name } => {
            loop {
                if let Err(err) = run(&name) {
                    if let Some(err) = err.downcast_ref::<io::Error>() {
                        if err.kind() == io::ErrorKind::ConnectionAborted {
                            println!("Warning: Connection aborted by the client.");
                            println!("Restarting listener..");
                            continue;
                        }
                    }
                    return Err(err);
                }
                break Ok(());
            }
        },

        Command::Delete { name } => config::delete_config(&name),

        Command::GenerateKey => {
            generate_key();
            Ok(())
        }
    }
}
