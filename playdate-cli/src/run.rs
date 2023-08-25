use std::process::Command;

use crate::{util::CommandExt, Runnable};

/// Run the local package on the playdate simulator or a device
#[derive(clap::Args, Debug)]
pub struct Run {
    #[command(flatten)]
    build: crate::build::Build,
}

impl Runnable for Run {
    fn run(&self) -> anyhow::Result<()> {
        if self.build.device {
            anyhow::bail!("Running on device is not supported yet");
        }
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
            simulator.to_string_lossy().replace(' ', "\\ "),
            build_info.pdx.to_string_lossy().replace(' ', "\\ ")
        );
        Command::new(simulator).arg(build_info.pdx).check()?;
        Ok(())
    }
}
