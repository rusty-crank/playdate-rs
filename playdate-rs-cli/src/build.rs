use std::{path::PathBuf, process::Command};

use cargo_metadata::{Metadata, MetadataCommand, Package, Target};

use crate::{util::CommandExt, Runnable};

static PDXINFO: &str = include_str!("../pdxinfo");

static DYLIB_EXT: &str = if cfg!(target_os = "macos") {
    "dylib"
} else {
    "so"
};

static PDEX_SO: &str = if cfg!(target_os = "macos") {
    "pdex.dylib"
} else {
    "pdex.so"
};

/// Build a playdate project
#[derive(clap::Args, Debug)]
pub struct Build {
    /// Build the project in release mode, with optimizations
    #[clap(long)]
    release: bool,
    #[command(flatten)]
    features: clap_cargo::Features,
    #[arg(short, long)]
    /// Package to process (see `cargo help pkgid`)
    pub package: Option<String>,
    /// Build for the real device (default is simulator)
    #[clap(long, default_value = "false")]
    pub device: bool,
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
        if let Some(pkg) = self.package.as_ref() {
            flags.push("--package".to_owned());
            flags.push(pkg.to_owned());
        }
        if self.device {
            flags.push("--target".to_owned());
            flags.push(" thumbv7em-none-eabihf".to_owned());
        }
        flags
    }

    fn load_metadata(&self) -> anyhow::Result<Metadata> {
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .exec()?;
        Ok(meta)
    }

    fn get_package(&self, meta: &Metadata) -> anyhow::Result<Package> {
        if self.package.is_none() {
            meta.root_package()
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No rust package found under current directory"))
        } else {
            let pkg = meta
                .packages
                .iter()
                .find(|p| &p.name == self.package.as_ref().unwrap())
                .cloned();
            pkg.ok_or_else(|| {
                anyhow::anyhow!(
                    "No rust package found with name {}",
                    self.package.as_ref().unwrap()
                )
            })
        }
    }

    fn get_assets_dir(&self, meta: &Metadata) -> anyhow::Result<PathBuf> {
        let project_dir = self
            .get_package(meta)?
            .manifest_path
            .as_std_path()
            .parent()
            .unwrap()
            .to_owned();
        Ok(project_dir.join("assets"))
    }

    fn get_target_dir(&self, meta: &Metadata) -> anyhow::Result<PathBuf> {
        let mut target_dir = meta.target_directory.clone().into_std_path_buf();
        if self.device {
            target_dir.push("thumbv7em-none-eabihf");
        }
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
    pub binary: PathBuf,
    pub pdx: PathBuf,
}

impl Build {
    fn link_arm_binary(
        &self,
        name: &str,
        target_dir: &PathBuf,
        lib_path: &PathBuf,
    ) -> anyhow::Result<()> {
        let linker_script = crate::util::get_playdate_sdk_path()?
            .join("C_API")
            .join("buildsupport")
            .join("link_map.ld");
        const LINKER_ARGS: &str = "-nostartfiles -mthumb -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16 -D__FPU_USED=1 -Wl,--gc-sections,--no-warn-mismatch,--emit-relocs -fno-exceptions";
        let mut args = vec![];
        args.push(lib_path.to_str().unwrap().to_owned());
        args.append(
            &mut LINKER_ARGS
                .split(" ")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>(),
        );
        args.push("-T".to_owned());
        args.push(linker_script.to_str().unwrap().to_owned());
        args.push("-o".to_owned());
        args.push(
            target_dir
                .join(format!("{}.elf", name))
                .to_str()
                .unwrap()
                .to_owned(),
        );
        args.push("--entry".to_owned());
        args.push("eventHandler".to_owned());
        info!("➔  arm-none-eabi-gcc {}", args.join(" "));
        Command::new("arm-none-eabi-gcc").args(&args).check()?;
        Ok(())
    }

    fn copy_build_output(
        &self,
        target: &Target,
        target_dir: &PathBuf,
        binary: &PathBuf,
        package: &Package,
    ) -> anyhow::Result<PathBuf> {
        // Create pdx folder
        let pdx_src = target_dir.join(format!("{}.source", target.name));
        Command::new("rm").arg("-rf").arg(&pdx_src).check()?;
        Command::new("mkdir").arg("-p").arg(&pdx_src).check()?;
        // Copy output files
        let pdex_so = if self.device { "pdex.elf" } else { PDEX_SO };
        Command::new("cp")
            .arg(&binary)
            .arg(pdx_src.join(pdex_so))
            .check()?;
        let pdxinfo = self.load_pdxinfo(&package, target)?;
        std::fs::write(pdx_src.join("pdxinfo"), pdxinfo)?;
        Ok(pdx_src)
    }

    fn copy_assets(&self, meta: &Metadata, pdx_src: &PathBuf) -> anyhow::Result<()> {
        let assets_dir = self.get_assets_dir(&meta)?;
        if assets_dir.exists() && assets_dir.is_dir() {
            for entry in std::fs::read_dir(&assets_dir)? {
                let entry = entry?;
                let path: PathBuf = entry.path();
                Command::new("cp").arg(&path).arg(&pdx_src).check()?;
            }
        }
        Ok(())
    }

    fn invoke_pdc(
        &self,
        target: &Target,
        target_dir: &PathBuf,
        pdx_src: &PathBuf,
    ) -> anyhow::Result<PathBuf> {
        let pdx_out = target_dir.join(format!("{}.pdx", target.name));
        let playdate_sdk_path = crate::util::get_playdate_sdk_path()?;
        let pdx_bin = playdate_sdk_path.join("bin").join("pdc");
        info!(
            "➔  {} {} {}",
            pdx_bin.to_string_lossy().replace(" ", "\\ "),
            pdx_src.to_string_lossy().replace(" ", "\\ "),
            pdx_out.to_string_lossy().replace(" ", "\\ "),
        );
        Command::new(pdx_bin).arg(&pdx_src).arg(&pdx_out).check()?;
        Ok(pdx_out)
    }
}

impl Runnable<BuildInfo> for Build {
    fn run(&self) -> anyhow::Result<BuildInfo> {
        // Find dylib target
        let meta = self.load_metadata()?;
        let package = self.get_package(&meta)?;
        info!("Building {}", package.name);
        let target = &package
            .targets
            .iter()
            .find(|t| t.crate_types.contains(&"cdylib".to_lowercase()));
        let Some(target) = target else {
            anyhow::bail!("Current crate has no cdylib target");
        };
        let target_dir = self.get_target_dir(&meta)?;
        let mut binary = target_dir.join(format!(
            "lib{}.{}",
            target.name.replace('-', "_"),
            DYLIB_EXT
        ));
        // Build rust project
        Command::new("cargo")
            .arg("build")
            .args(self.get_cargo_flags())
            .check()?;
        if self.device {
            // TODO: Link the staticlib using arm-none-eabi-gcc
            let staticlib = target_dir.join(format!("lib{}.a", target.name.replace('-', "_"),));
            self.link_arm_binary(&target.name.replace('-', "_"), &target_dir, &staticlib)?;
            binary = target_dir.join(format!("{}.elf", target.name.replace('-', "_")));
        }
        // Create pdx folder and copy output files
        let pdx_src = self.copy_build_output(target, &target_dir, &binary, &package)?;
        // Copy assets
        self.copy_assets(&meta, &pdx_src)?;
        // call pdc
        let pdx = self.invoke_pdc(target, &target_dir, &pdx_src)?;

        Ok(BuildInfo {
            name: target.name.clone(),
            binary,
            pdx,
        })
    }
}
