use std::{path::PathBuf, process::Command};

use crate::Runnable;

/// Build the current playdate project
#[derive(clap::Args, Debug)]
pub struct Run {
    #[command(flatten)]
    build: crate::build::Build,
}

impl Runnable for Run {
    fn run(&self) -> anyhow::Result<()> {
        let build_info = self.build.run()?;
        let playdate_sdk_path = std::env::var("PLAYDATE_SDK_PATH")
            .expect("Environment variable PLAYDATE_SDK_PATH is not set");
        let simulator = PathBuf::from(playdate_sdk_path)
            .join("bin")
            .join("PlaydateSimulator");

        info!("Running {}", build_info.name);
        info!(
            "➔  {} {}",
            simulator.to_string_lossy(),
            build_info.pdx.to_string_lossy()
        );
        Command::new(simulator).arg(build_info.pdx).status()?;
        Ok(())
    }
}
