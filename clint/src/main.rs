use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use auto_launch::AutoLaunchBuilder;
use dori_client::config::ClientConfiguration;

fn load_config() -> Result<ClientConfiguration> {
    let txt = fs::read_to_string(".dori.toml")
        .with_context(|| "Failed to read from configuration file")?;

    toml::from_str(&txt).with_context(|| "Failed to deserialize configuration")
}

//noinspection RsExternalLinter
fn program_path(program: &str) -> Result<PathBuf> {
    #[cfg(windows)]
    {
        let mut homedir =
            homedir::get_my_home()?.with_context(|| "Failed to find home directory")?;
        homedir.push(format!("\\AppData\\Local\\{program}\\{program}.exe"));
        return Ok(homedir);
    }
    #[cfg(unix)]
    {
        let path = PathBuf::new();
        path.push(format!("/opt/{program}/{program}"));
        return Ok(path);
    }
    bail!("Unsupported platform");
}

fn main() -> Result<()> {
    let config = load_config()?;
    let program_path = program_path(config.program_name())?;

    fs::copy("dori-client.exe", program_path)?;

    AutoLaunchBuilder::new()
        .set_app_name(config.program_name())
        .set_args(&config.to_args())
        .set_use_launch_agent(false)
        .build()?
        .enable()
        .with_context(|| "Failed to enable auto-launch")
}
