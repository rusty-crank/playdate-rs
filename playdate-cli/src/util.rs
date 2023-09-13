use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub trait CommandExt {
    fn check(&mut self, log: bool) -> anyhow::Result<()>;
}

impl CommandExt for Command {
    fn check(&mut self, log: bool) -> anyhow::Result<()> {
        let cmd = self.get_program().to_str().unwrap();
        let args = self
            .get_args()
            .map(|a| a.to_str().unwrap().to_owned().replace(" ", "\\ "))
            .collect::<Vec<String>>()
            .join(" ");
        if log {
            info!("➔  {} {}", cmd, args);
        }
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

pub fn get_playdate_serial_device() -> anyhow::Result<PathBuf> {
    assert!(cfg!(target_os = "macos"));
    for device in std::fs::read_dir("/dev")? {
        let device = device?;
        let path = device.path();
        if path
            .file_name()
            .map(|f| f.to_str().unwrap())
            .unwrap()
            .starts_with("cu.usbmodemPDU")
        {
            return Ok(path);
        }
    }
    Err(anyhow::anyhow!(
        "Could not find Playdate serial device. Did you forget to plug it in or unlock it?"
    ))
}

pub fn get_playdate_data_volume(wait: bool) -> anyhow::Result<PathBuf> {
    assert!(cfg!(target_os = "macos"));
    let playdate_data_volume = PathBuf::from("/Volumes/PLAYDATE");
    if !wait {
        if !playdate_data_volume.join("Games").is_dir() {
            anyhow::bail!("Playdate data volume is not mounted");
        }
    } else {
        wait_until(|| playdate_data_volume.join("Games").is_dir())?;
    }
    Ok(playdate_data_volume)
}

pub fn mount_playdate_data_volume() -> anyhow::Result<PathBuf> {
    assert!(cfg!(target_os = "macos"));
    let dev = get_playdate_serial_device()?;
    let playdate_sdk_path = get_playdate_sdk_path()?;
    let pdutil = playdate_sdk_path.join("bin").join("pdutil");
    info!("Mounting Playdate data volume");
    Command::new(pdutil).arg(dev).arg("datadisk").check(true)?;
    get_playdate_data_volume(true)
}

pub fn eject_playdate_data_volume() -> anyhow::Result<()> {
    assert!(cfg!(target_os = "macos"));
    info!("Ejecting Playdate data volume");
    Command::new("diskutil")
        .arg("eject")
        .arg("/Volumes/PLAYDATE")
        .check(true)?;
    wait_until(|| !get_playdate_serial_device().is_err())?;
    Ok(())
}

fn wait_until(predicate: impl Fn() -> bool) -> anyhow::Result<()> {
    let t = std::time::SystemTime::now();
    while !predicate() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if t.elapsed().unwrap().as_secs() > 60 {
            anyhow::bail!("Timed out");
        }
    }
    Ok(())
}
