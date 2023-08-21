use std::process::Command;

use crate::{util::CommandExt, Runnable};

/// Build the current playdate project
#[derive(clap::Args, Debug)]
pub struct Run {
    #[command(flatten)]
    build: crate::build::Build,
}

impl Runnable for Run {
    fn run(&self) -> anyhow::Result<()> {
        let build_info = self.build.run()?;
        let playdate_sdk_path = crate::util::get_playdate_sdk_path()?;
        let simulator = if cfg!(target_os = "macos") {
            playdate_sdk_path
                .join("bin")
                .join("Playdate Simulator.app")
                .join("Contents")
                .join("MacOS")
                .join("Playdate Simulator")
        } else {
            playdate_sdk_path.join("bin").join("PlaydateSimulator")
        };

        info!("Running {}", build_info.name);
        info!(
            "➔  {} {}",
            simulator.to_string_lossy(),
            build_info.pdx.to_string_lossy()
        );
        Command::new(simulator).arg(build_info.pdx).check()?;
        Ok(())
    }
}
