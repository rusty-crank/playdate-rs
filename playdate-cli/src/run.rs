use std::{path::PathBuf, process::Command};

use crate::{build::BuildInfo, util::CommandExt, Runnable};

/// Run the local package on the playdate simulator or a device
#[derive(clap::Args, Debug)]
pub struct Run {
    #[command(flatten)]
    build: crate::build::Build,
}

impl Run {
    fn run_simulator(&self, build_info: BuildInfo) -> anyhow::Result<()> {
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
        Command::new(simulator).arg(build_info.pdx).check(true)?;
        Ok(())
    }

    fn run_device(&self, build_info: BuildInfo) -> anyhow::Result<()> {
        // mount the playdate data volume if it's not already mounted
        let playdate_data_volume = crate::util::get_playdate_data_volume(false)
            .or_else(|_| crate::util::mount_playdate_data_volume())?;
        // copy game to playdate data volume
        let game_dir = playdate_data_volume.join("Games");
        let target_dir = game_dir.join(build_info.pdx.file_name().unwrap());
        if target_dir.is_dir() {
            info!("Remove existing {}", target_dir.to_string_lossy());
            std::fs::remove_dir_all(&target_dir)?;
        }
        info!(
            "Copying {} to {}",
            build_info.pdx.to_string_lossy(),
            target_dir.to_string_lossy()
        );
        Command::new("cp")
            .arg("-r")
            .arg(&build_info.pdx)
            .arg(target_dir)
            .check(false)?;
        // Eject data volume
        crate::util::eject_playdate_data_volume()?;
        // Run the game
        let playdate_sdk_path = crate::util::get_playdate_sdk_path()?;
        let pdutil = playdate_sdk_path.join("bin").join("pdutil");
        info!("Running {}", build_info.name);
        let serial_device = crate::util::get_playdate_serial_device()?;
        Command::new(pdutil)
            .arg(serial_device)
            .arg("run")
            .arg(PathBuf::from("/Games").join(build_info.pdx.file_name().unwrap()))
            .check(true)?;
        Ok(())
    }
}

impl Runnable for Run {
    fn run(&self) -> anyhow::Result<()> {
        let build_info = self.build.run()?;
        if self.build.device {
            self.run_device(build_info)
        } else {
            self.run_simulator(build_info)
        }
    }
}
