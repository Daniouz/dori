use std::{env, fs};
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
fn program_path(program: &str) -> Result<(PathBuf, String)> {
    #[cfg(windows)]
    {
        let mut homedir =
            homedir::get_my_home()?.with_context(|| "Failed to find home directory")?;
        homedir.push(format!("AppData\\Local\\{program}"));
        return Ok((homedir, format!("{program}.exe")));
    }
    #[cfg(unix)]
    {
        let path = PathBuf::new();
        path.push(format!("/opt/{program}/{program}"));
        return Ok((path, program.to_string()));
    }
    bail!("Unsupported platform");
}

fn client_exe() -> Result<PathBuf> {
    let mut client_exe_path = env::current_exe()
        .with_context(|| "Failed to get directory of clint executable")?
        .parent()
        .with_context(|| "Failed to get parent directory of clint executable")?
        .to_path_buf();

    client_exe_path.push("dori-client.exe");
    Ok(client_exe_path)
}

fn main() -> Result<()> {
    let config = load_config()?;
    let (mut program_path, exe_name) = program_path(config.program_name())?;

    println!("Program path: {}", program_path.display());
    println!("Program arguments: {:?}", &config.to_args());

    fs::create_dir(&program_path).with_context(|| "Failed to create program directory")?;

    program_path.push(exe_name);

    fs::copy(client_exe()?, &program_path).with_context(|| "Failed to copy client executable")?;

    AutoLaunchBuilder::new()
        .set_app_name(config.program_name())
        .set_args(&config.to_args())
        .set_use_launch_agent(false)
        .set_app_path(&program_path.display().to_string())
        .build()?
        .enable()
        .with_context(|| "Failed to enable auto-launch")
}
