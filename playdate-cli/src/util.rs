use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub trait CommandExt {
    fn check(&mut self) -> anyhow::Result<()>;
}

impl CommandExt for Command {
    fn check(&mut self) -> anyhow::Result<()> {
        let status = self.status()?;
        if !status.success() {
            let args = self
                .get_args()
                .map(|a| a.to_str().unwrap().to_owned())
                .collect::<Vec<String>>()
                .join(" ");
            anyhow::bail!(
                "failed to execute command: {} {}",
                self.get_program().to_str().unwrap(),
                args
            );
        }
        Ok(())
    }
}

pub fn get_playdate_sdk_path() -> anyhow::Result<PathBuf> {
    let is_correct_sdk_path = |path: &Path| path.join("bin").join("pdc").is_file();
    if cfg!(target_os = "macos") {
        let playdate_sdk_path = home::home_dir()
            .expect("Could not find home directory")
            .join("Developer")
            .join("PlaydateSDK");
        if is_correct_sdk_path(&playdate_sdk_path) {
            return Ok(playdate_sdk_path);
        }
    }
    let playdate_sdk_path = PathBuf::from(std::env::var("PLAYDATE_SDK_PATH")?);
    if !is_correct_sdk_path(&playdate_sdk_path) {
        anyhow::bail!(
            "PLAYDATE_SDK_PATH ({}) is not set to the root of the Playdate SDK",
            playdate_sdk_path.to_string_lossy()
        )
    }
    Ok(playdate_sdk_path)
}
