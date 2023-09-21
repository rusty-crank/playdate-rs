//! Playdate CLI for Lua projects

use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use log::info;
use playdate_cli::util;
use playdate_cli::util::CommandExt;

/// Playdate CLI for Lua projects
#[derive(Parser, Debug)]
#[command(author = "Wenyu Zhao <wenyu.zhao@anu.edu.au>", version = "0.0.1")]
enum Commands {
    /// Build a Playdate Lua project
    #[clap(alias = "b")]
    Build,
    /// Build and run a Playdate Lua project
    #[clap(alias = "r")]
    Run {
        /// Run the project on the device
        #[clap(long)]
        device: bool,
    },
}

fn get_cwd_name() -> anyhow::Result<String> {
    let dir = std::env::current_dir()?;
    Ok(dir.file_name().unwrap().to_str().unwrap().to_owned())
}

fn get_src_path() -> anyhow::Result<PathBuf> {
    let src = PathBuf::from("Source");
    if !src.exists() {
        anyhow::bail!("Source directory not found");
    }
    Ok(src)
}

fn build() -> anyhow::Result<PathBuf> {
    let tgt = get_cwd_name()?;
    let src = get_src_path()?;
    let out_dir = PathBuf::from("out");
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir)?;
    }
    let out = out_dir.join(tgt).with_extension("pdx");
    let sdk = util::get_playdate_sdk_path()?;
    let pdc = sdk.join("bin").join("pdc");
    Command::new(pdc).arg(src).arg(&out).check(true)?;
    Ok(out)
}

fn run_device(out: PathBuf) -> anyhow::Result<()> {
    // Eject data volume
    util::eject_playdate_data_volume()?;
    // Run the game
    let playdate_sdk_path = util::get_playdate_sdk_path()?;
    let pdutil = playdate_sdk_path.join("bin").join("pdutil");
    let serial_device = util::get_playdate_serial_device()?;
    let fname = out.file_name().unwrap();
    Command::new(pdutil)
        .arg(serial_device)
        .arg("run")
        .arg(PathBuf::from("/Games").join(fname))
        .check(true)?;
    Ok(())
}

fn run_simulator(out: PathBuf) -> anyhow::Result<()> {
    let simulator = util::get_playdate_simulator()?;
    Command::new(simulator).arg(out).check(true)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .init();

    match Commands::parse() {
        Commands::Build => {
            build()?;
        }
        Commands::Run { device } => {
            let out = build()?;
            info!("Running {}", out.to_str().unwrap().replace(" ", "\\ "));
            if device {
                run_device(out)?;
            } else {
                run_simulator(out)?;
            }
        }
    }

    Ok(())
}
