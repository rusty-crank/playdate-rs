use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

use cargo_metadata::{Metadata, MetadataCommand, Package, Target};

use crate::{util::CommandExt, Runnable};

static PDXINFO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/pdxinfo.template"
));

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

/// Compile the current package
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
    #[arg(short, long)]
    /// Build only the specified example
    pub example: Option<String>,
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
        if let Some(pkg) = self.example.as_ref() {
            flags.push("--example".to_owned());
            flags.push(pkg.to_owned());
        }
        if self.device {
            flags.push("--target".to_owned());
            flags.push("thumbv7em-none-eabihf".to_owned());
            flags.push("-Zbuild-std=core,alloc,compiler_builtins".to_owned());
            flags.push("-Zbuild-std-features=compiler-builtins-mem".to_owned());
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

    fn get_target(&self, package: &Package) -> anyhow::Result<Target> {
        if let Some(example) = self.example.as_ref() {
            let t = package
                .targets
                .iter()
                .find(|t| t.is_example() && &t.name == example);
            if t.is_none() {
                anyhow::bail!("No example found with name {}", example);
            }
            Ok(t.unwrap().clone())
        } else {
            let t = package.targets.iter().find(|t| {
                t.crate_types.contains(&"cdylib".to_owned())
                    && t.crate_types.contains(&"staticlib".to_owned())
            });
            if t.is_none() {
                anyhow::bail!(
                    "Current crate has no target with `crate-type = [\"cdylib\", \"staticlib\"]`"
                );
            };
            Ok(t.unwrap().clone())
        }
    }

    fn get_target_name(&self, target: &Target) -> anyhow::Result<String> {
        Ok(target.name.replace('-', "_"))
    }

    fn get_assets_dir(&self, meta: &Metadata) -> anyhow::Result<PathBuf> {
        let project_dir = self
            .get_package(meta)?
            .manifest_path
            .as_std_path()
            .parent()
            .unwrap()
            .to_owned();
        if self.example.is_some() {
            // Use <project>/examples/assets/ as the assets dir
            return Ok(project_dir.join("examples").join("assets"));
        }
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
        if self.example.is_some() {
            target_dir.push("examples");
        }
        Ok(target_dir)
    }

    fn load_pdxinfo(&self, pkg: &Package, target_name: &str) -> anyhow::Result<String> {
        if self.example.is_some() {
            let pdxinfo = format!(
                "name={}\nauthor=playdate-rs\ndescription=playdate-rs-example\nbundleID=me.wenyu.playdate.example.{}\n",
                target_name, target_name
            );
            return Ok(pdxinfo);
        }
        // If there is a pdxinfo file under package root, just use it
        let pdxinfo_path = pkg
            .manifest_path
            .as_std_path()
            .parent()
            .unwrap()
            .join("pdxinfo");
        if pdxinfo_path.is_file() || pdxinfo_path.is_symlink() {
            info!("Using pdxinfo from {}", pdxinfo_path.to_string_lossy());
            return Ok(std::fs::read_to_string(pdxinfo_path)?);
        }
        // Generate from pdxinfo template
        let pdxinfo_meta = pkg
            .metadata
            .get("pdxinfo")
            .map(|v| v.as_object().unwrap().clone())
            .unwrap_or_default();
        let get_meta = |key: &str, default: &str, warn: Option<&str>| -> String {
            pdxinfo_meta
                .get(key)
                .map(|v| v.as_str().unwrap().to_owned())
                .unwrap_or_else(|| {
                    if let Some(warn) = warn {
                        warn!("{}", warn);
                    }
                    default.to_owned()
                })
        };
        let fmt_content_warning = |n: &str, s: &str| -> String {
            let s = s.trim();
            if !s.is_empty() {
                format!("{}={}\n", n, s)
            } else {
                "".to_owned()
            }
        };
        let mut env = minijinja::Environment::new();
        env.add_template("pdxinfo", PDXINFO)?;
        let template = env.get_template("pdxinfo").unwrap();
        let default_bundle_id = format!("com.example.{}", pkg.name);
        let s = template.render(minijinja::context! {
            name => get_meta("name", target_name, None),
            author => get_meta("author", &pkg.authors.join(", "), None),
            description => get_meta("description", pkg.description.as_ref().unwrap_or(&"".to_owned()), None),
            bundle_id => get_meta("bundle_id", &default_bundle_id, Some(&format!("Using default bundle id: {}", default_bundle_id))),
            image_path => get_meta("image_path", "", None),
            launch_sound_path => get_meta("launch_sound_path", "", None),
            content_warning => fmt_content_warning("contentWarning", &get_meta("content_warning", "", None)),
            content_warning2 => fmt_content_warning("contentWarning2", &get_meta("content_warning2", "", None)),
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
    fn build_setup(&self, target_dir: &Path) -> anyhow::Result<PathBuf> {
        let sdk = crate::util::get_playdate_sdk_path()?;
        let setup_c = sdk.join("C_API").join("buildsupport").join("setup.c");
        Command::new("arm-none-eabi-gcc")
            .args(["-DTARGET_EXTENSION=1", "-DTARGET_PLAYDATE=1"])
            .arg(format!("-I{}", sdk.join("C_API").to_string_lossy()))
            .args("-std=gnu11 -Wall -Wno-unknown-pragmas -Wdouble-promotion -mthumb -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16 -D__FPU_USED=1 -falign-functions=16 -fomit-frame-pointer -gdwarf-2 -fverbose-asm -ffunction-sections -fdata-sections -mword-relocations -fno-common -MD -MT".split(' '))
            .arg(format!("{}/setup.c.obj", target_dir.to_string_lossy()))
            .arg("-MF")
            .arg(format!("{}/setup.c.obj.d", target_dir.to_string_lossy()))
            .arg("-o")
            .arg(format!("{}/setup.c.obj", target_dir.to_string_lossy()))
            .arg("-c")
            .arg(setup_c)
            .check(true)?;
        Ok(target_dir.join("setup.c.obj"))
    }

    fn link_arm_binary(
        &self,
        target_name: &str,
        target_dir: &Path,
        lib_path: &Path,
    ) -> anyhow::Result<PathBuf> {
        // check arm-none-eabi-gcc
        if Command::new("arm-none-eabi-gcc")
            .arg("--version")
            .output()
            .is_err()
        {
            anyhow::bail!(
                "arm-none-eabi-gcc not found. Please ensure it is installed and in PATH."
            );
        }
        // build setup.c
        let setup_obj = self.build_setup(target_dir)?;
        // link
        let linker_script = crate::util::get_playdate_sdk_path()?
            .join("C_API")
            .join("buildsupport")
            .join("link_map.ld");
        Command::new("arm-none-eabi-gcc")
        .args("-nostartfiles -mthumb -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16 -D__FPU_USED=1".split(' '))
        .arg("-T")
        .arg(linker_script )
        .args("-Wl,--gc-sections,--no-warn-mismatch,--emit-relocs".split(' '))
        .args(["--entry", "eventHandlerShim"])
        .arg(setup_obj)
        .arg(lib_path)
        .arg("-o")
        .arg( target_dir
            .join(format!("{}.elf", target_name)))
        .check(true)?;
        Ok(target_dir.join(format!("{}.elf", target_name)))
    }

    fn copy_build_output(
        &self,
        target_name: &str,
        target_dir: &Path,
        binary: &Path,
        package: &Package,
    ) -> anyhow::Result<PathBuf> {
        // Create pdx folder
        let pdx_src = target_dir.join(format!("{}.source", target_name));
        Command::new("rm").arg("-rf").arg(&pdx_src).check(false)?;
        Command::new("mkdir").arg("-p").arg(&pdx_src).check(false)?;
        // Copy output files
        let pdex_so = if self.device { "pdex.elf" } else { PDEX_SO };
        Command::new("cp")
            .arg(binary)
            .arg(pdx_src.join(pdex_so))
            .check(false)?;
        let pdxinfo = self.load_pdxinfo(package, target_name)?;
        std::fs::write(pdx_src.join("pdxinfo"), pdxinfo)?;
        Ok(pdx_src)
    }

    fn copy_assets(&self, meta: &Metadata, pdx_src: &PathBuf) -> anyhow::Result<()> {
        let assets_dir = self.get_assets_dir(meta)?;
        if assets_dir.exists() && assets_dir.is_dir() {
            for entry in std::fs::read_dir(&assets_dir)? {
                let entry = entry?;
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    Command::new("cp")
                        .arg("-r")
                        .arg(&path)
                        .arg(pdx_src)
                        .check(false)?;
                } else {
                    Command::new("cp").arg(&path).arg(pdx_src).check(false)?;
                }
            }
        }
        Ok(())
    }

    fn invoke_pdc(
        &self,
        target_name: &str,
        target_dir: &Path,
        pdx_src: &Path,
    ) -> anyhow::Result<PathBuf> {
        let pdx_out = target_dir.join(format!("{}.pdx", target_name));
        let playdate_sdk_path = crate::util::get_playdate_sdk_path()?;
        let pdx_bin = playdate_sdk_path.join("bin").join("pdc");
        Command::new(pdx_bin)
            .arg("--strip")
            .arg(pdx_src)
            .arg(&pdx_out)
            .check(true)?;
        Ok(pdx_out)
    }
}

impl Runnable<BuildInfo> for Build {
    fn run(&self) -> anyhow::Result<BuildInfo> {
        // Find dylib target
        let meta = self.load_metadata()?;
        let package = self.get_package(&meta)?;
        let target = self.get_target(&package)?;
        info!("Building {}", target.name);
        // Find target name and target output dir
        let target_name = self.get_target_name(&target)?;
        let target_dir = self.get_target_dir(&meta)?;
        let mut binary = target_dir.join(format!("lib{}.{}", target_name, DYLIB_EXT));
        // Build rust project
        Command::new("cargo")
            .arg("+nightly")
            .arg("build")
            .args(self.get_cargo_flags())
            .envs(&if self.device {
                let mut map = HashMap::new();
                map.insert("RUSTFLAGS", ["-Crelocation-model=pic"].join(" "));
                map
            } else {
                Default::default()
            })
            .check(true)?;
        if self.device {
            // Link the staticlib using arm-none-eabi-gcc
            let staticlib = target_dir.join(format!("lib{}.a", target_name));
            binary = self.link_arm_binary(&target_name, &target_dir, &staticlib)?;
        }
        // Create pdx folder and copy output files
        let pdx_src = self.copy_build_output(&target_name, &target_dir, &binary, &package)?;
        // Copy assets
        self.copy_assets(&meta, &pdx_src)?;
        // call pdc
        let pdx = self.invoke_pdc(&target_name, &target_dir, &pdx_src)?;

        Ok(BuildInfo {
            name: target_name,
            binary,
            pdx,
        })
    }
}
