use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub trait CommandExt {
    fn check(&mut self, log: bool) -> anyhow::Result<()>;
}

impl CommandExt for Command {
    /// Executes the command and checks the exit status.
    /// Optionally logs the command before executing.
    fn check(&mut self, log: bool) -> anyhow::Result<()> {
        let cmd = self.get_program().to_str().unwrap().to_owned();
        let args = self
            .get_args()
            .map(|a| a.to_str().unwrap().to_owned().replace(' ', "\\ "))
            .collect::<Vec<String>>()
            .join(" ");
        let mut env = self
            .get_envs()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    k.to_str().unwrap(),
                    v.unwrap_or_default().to_str().unwrap()
                )
            })
            .collect::<Vec<String>>()
            .join(" ");
        if !env.is_empty() {
            env = format!("{} ", env);
        }
        if log {
            info!("➔  {}{} {}", env, cmd, args);
        }
        let status = self.status()?;
        if !status.success() {
            let args = self
                .get_args()
                .map(|a| a.to_str().unwrap().to_owned())
                .collect::<Vec<String>>()
                .join(" ");
            anyhow::bail!("failed to execute command: {}{} {}", env, cmd, args);
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

#[cfg(target_os = "macos")]
pub fn get_playdate_serial_device() -> anyhow::Result<PathBuf> {
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

#[cfg(target_os = "linux")]
pub fn get_playdate_serial_device() -> anyhow::Result<PathBuf> {
    if Path::new("/dev/serial/by-id").is_dir() {
        for device in std::fs::read_dir("/dev/serial/by-id")? {
            let device = device?;
            let path = device.path();
            if path
                .file_name()
                .map(|f| f.to_str().unwrap())
                .unwrap()
                .starts_with("usb-Panic_Inc_Playdate_")
            {
                return Ok(path);
            }
        }
    }
    Err(anyhow::anyhow!(
        "Could not find Playdate serial device. Did you forget to plug it in or unlock it?"
    ))
}

#[cfg(target_os = "macos")]
pub fn get_playdate_data_volume(wait: bool) -> anyhow::Result<PathBuf> {
    assert!(cfg!(target_os = "macos"));
    let playdate_data_volume = PathBuf::from("/Volumes/PLAYDATE");
    if !wait {
        if !playdate_data_volume.join("Games").is_dir() {
            anyhow::bail!("Playdate data volume is not mounted");
        }
    } else {
        wait_until(|| playdate_data_volume.join("Games").is_dir(), Some(60))?;
    }
    Ok(playdate_data_volume)
}

#[cfg(target_os = "linux")]
pub fn get_playdate_data_volume(wait: bool) -> anyhow::Result<PathBuf> {
    let user = std::env::var("USER")?;
    let playdate_data_volumes = [
        PathBuf::from("/media").join(&user).join("PLAYDATE"),
        PathBuf::from("/run/media").join(&user).join("PLAYDATE"),
    ];
    if wait {
        println!("Waiting for Playdate data volume to be mounted...");
        println!("Please ensure the datadisk is mounted to one of the following locations:");
        for v in &playdate_data_volumes {
            println!(" - {}", v.to_string_lossy());
        }
        wait_until(
            || {
                for v in &playdate_data_volumes {
                    if v.join("Games").is_dir() {
                        return true;
                    }
                }
                false
            },
            None,
        )?;
    }
    for v in &playdate_data_volumes {
        if v.join("Games").is_dir() {
            return Ok(v.clone());
        }
    }
    anyhow::bail!("Playdate data volume is not mounted");
}

pub fn mount_playdate_data_volume() -> anyhow::Result<PathBuf> {
    let dev = get_playdate_serial_device()?;
    let playdate_sdk_path = get_playdate_sdk_path()?;
    let pdutil = playdate_sdk_path.join("bin").join("pdutil");
    info!("Mounting Playdate data volume");
    Command::new(pdutil).arg(dev).arg("datadisk").check(true)?;
    get_playdate_data_volume(true)
}

#[cfg(target_os = "macos")]
pub fn eject_playdate_data_volume() -> anyhow::Result<()> {
    info!("Ejecting Playdate data volume");
    Command::new("diskutil")
        .arg("eject")
        .arg("/Volumes/PLAYDATE")
        .check(true)?;
    wait_until(|| get_playdate_serial_device().is_ok(), Some(60))?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn eject_playdate_data_volume() -> anyhow::Result<()> {
    info!("Ejecting Playdate data volume");
    if Path::new("/dev/disk/by-id").is_dir() {
        for device in std::fs::read_dir("/dev/disk/by-id")? {
            let device = device?;
            let path = device.path();
            if path
                .file_name()
                .map(|f| f.to_str().unwrap())
                .unwrap()
                .starts_with("usb-Panic_Inc_Playdate_")
            {
                Command::new("umount").arg(path).check(true)?;
            }
        }
    }
    info!("Please press A on the Playdate to restart");
    wait_until(|| get_playdate_serial_device().is_ok(), None)?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

fn wait_until(predicate: impl Fn() -> bool, limit_secs: Option<u64>) -> anyhow::Result<()> {
    let t = std::time::SystemTime::now();
    while !predicate() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if let Some(limit_secs) = limit_secs {
            if t.elapsed().unwrap().as_secs() > limit_secs {
                anyhow::bail!("Timed out");
            }
        }
    }
    Ok(())
}
