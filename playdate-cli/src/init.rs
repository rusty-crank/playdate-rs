use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{util::CommandExt, Runnable};

static CARGO_TOML: &str = include_str!("./templates/hello-world/Cargo.toml");
static LIB_RS: &str = include_str!("./templates/hello-world/src/lib.rs");
static RUST_PNG: &[u8] = include_bytes!("./templates/hello-world/assets/rust.png");
static GITIGNORE: &str = include_str!("./templates/hello-world/.gitignore");

/// Create a new cargo playdate package in an existing directory
#[derive(clap::Args, Debug)]
pub struct Init {
    #[clap(default_value = ".")]
    path: PathBuf,
}

impl Init {
    /// Initialize playdate project
    fn init_playdate_impl(new: bool, path: &Path) -> anyhow::Result<()> {
        info!("Configuring cargo playdate project ...");
        // Adding game assets and overwrite files
        info!("+  overwrite files");
        std::fs::create_dir_all("assets")?;
        std::fs::write(PathBuf::from("assets").join("rust.png"), RUST_PNG)?;
        std::fs::write(PathBuf::from(".gitignore"), GITIGNORE)?;
        std::fs::write(PathBuf::from("src").join("lib.rs"), LIB_RS)?;
        let original_cargo_toml =
            toml::from_str::<toml::Value>(&std::fs::read_to_string("Cargo.toml")?)?;
        let name = original_cargo_toml["package"]["name"].as_str().unwrap();
        let mut env = minijinja::Environment::new();
        env.add_template("Cargo.toml", CARGO_TOML)?;
        let template = env.get_template("Cargo.toml").unwrap();
        let pd_cargo_toml = template.render(minijinja::context! {
            name => name,
        })?;
        std::fs::write("Cargo.toml", pd_cargo_toml)?;
        // Add playdate-rs dependency
        info!("➔  cargo add playdate-rs");
        Command::new("cargo")
            .arg("add")
            .arg("playdate-rs")
            .check()?;
        println!("🎉 Initialized playdate project: {}", name);
        let cmd = if new {
            format!("cd {} && cargo playdate run", path.to_string_lossy())
        } else {
            "cargo playdate run".to_owned()
        };
        println!("🚀 Getting started by running it in the playdate simulator:");
        println!("   ➔  {}", cmd);
        Ok(())
    }

    /// Initialize playdate project
    pub fn init_playdate(new: bool, path: &PathBuf) -> anyhow::Result<()> {
        let cwd = std::env::current_dir()?;
        std::env::set_current_dir(path)?;
        let result = Self::init_playdate_impl(new, path);
        std::env::set_current_dir(cwd)?;
        result
    }
}

impl Runnable for Init {
    fn run(&self) -> anyhow::Result<()> {
        // Create cargo project
        info!("Initializing cargo project ...");
        info!("➔  cargo init --lib {}", self.path.to_string_lossy());
        Command::new("cargo").arg("init").arg("--lib").check()?;
        // Initialize playdate project
        Self::init_playdate(false, &self.path)?;
        Ok(())
    }
}
