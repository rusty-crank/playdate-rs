use std::{path::PathBuf, process::Command};

use crate::{util::CommandExt, Runnable};

/// Create a new cargo playdate package
#[derive(clap::Args, Debug)]
pub struct New {
    path: PathBuf,
}

impl Runnable for New {
    fn run(&self) -> anyhow::Result<()> {
        // Create cargo project
        info!("Creating new cargo project ...");
        Command::new("cargo")
            .arg("new")
            .arg("--lib")
            .arg(&self.path)
            .check()?;
        // Initialize playdate project
        crate::init::Init::init_playdate(true, &self.path, &None)?;
        Ok(())
    }
}
