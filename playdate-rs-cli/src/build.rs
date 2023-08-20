use std::{path::PathBuf, process::Command};

use cargo_metadata::{Metadata, MetadataCommand, Package, Target};

use crate::{util::CommandExt, Runnable};

static PDXINFO: &str = include_str!("../pdxinfo");

/// Build the current playdate project
#[derive(clap::Args, Debug)]
pub struct Build {
    /// Build the project in release mode, with optimizations
    #[clap(long)]
    release: bool,
    #[command(flatten)]
    features: clap_cargo::Features,
}

impl Build {
    fn get_cargo_flags(&self) -> Vec<String> {
        let mut flags = vec![];
        if self.release {
            flags.push("--release".to_owned());
        }
        if !self.features.features.is_empty() {
            flags.push("--features".to_owned());
            flags.push(self.features.features.join(","));
        }
        if self.features.no_default_features {
            flags.push("--no-default-features".to_owned());
        }
        if self.features.all_features {
            flags.push("--all-features".to_owned());
        }
        flags
    }

    fn load_metadata(&self) -> anyhow::Result<Metadata> {
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .exec()?;
        Ok(meta)
    }

    fn get_target_dir(&self, meta: &Metadata) -> anyhow::Result<PathBuf> {
        let mut target_dir = meta.target_directory.clone().into_std_path_buf();
        if self.release {
            target_dir.push("release");
        } else {
            target_dir.push("debug");
        }
        Ok(target_dir)
    }

    fn load_pdxinfo(&self, pkg: &Package, target: &Target) -> anyhow::Result<String> {
        let mut env = minijinja::Environment::new();
        env.add_template("pdxinfo", PDXINFO)?;
        let template = env.get_template("pdxinfo").unwrap();
        let s = template.render(minijinja::context! {
            name => target.name,
            author => pkg.authors.join(", "),
            description => pkg.description.as_ref().unwrap_or(&"".to_owned()),
        })?;
        Ok(s)
    }
}

pub struct BuildInfo {
    pub name: String,
    pub dylib: PathBuf,
    pub pdx: PathBuf,
}

impl Runnable<BuildInfo> for Build {
    fn run(&self) -> anyhow::Result<BuildInfo> {
        info!("Building the project {}", self.release);
        // Build rust project
        Command::new("cargo")
            .arg("build")
            .args(self.get_cargo_flags())
            .check()?;
        // Find dylib target
        let meta = self.load_metadata()?;
        let package = meta.root_package().unwrap();
        let target = &package
            .targets
            .iter()
            .find(|t| t.crate_types.contains(&"cdylib".to_lowercase()));
        let Some(target) = target else {
            anyhow::bail!("Current crate has no cdylib target");
        };
        let target_dir = self.get_target_dir(&meta)?;
        let dylib = target_dir.join(format!("lib{}.so", target.name).replace('-', "_"));
        // Create pdx folder
        let pdx = target_dir.join(format!("{}.pdx", target.name));
        Command::new("mkdir").arg("-p").arg(&pdx).check()?;
        // Copy output files
        Command::new("rm")
            .arg("-f")
            .arg(pdx.join("pdex.so"))
            .check()?;
        Command::new("cp")
            .arg(&dylib)
            .arg(pdx.join("pdex.so"))
            .check()?;
        Command::new("rm")
            .arg("-f")
            .arg(pdx.join("pdxinfo"))
            .check()?;
        let pdxinfo = self.load_pdxinfo(package, target)?;
        std::fs::write(pdx.join("pdxinfo"), pdxinfo)?;
        Ok(BuildInfo {
            name: target.name.clone(),
            dylib,
            pdx,
        })
    }
}
