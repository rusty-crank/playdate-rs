use std::process::Command;

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
