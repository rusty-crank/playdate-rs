use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{util::CommandExt, Runnable};

static CARGO_TOML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/hello-world/Cargo.toml.template"
));
static LIB_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/hello-world/src/lib.rs"
));
static RUST_PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/hello-world/assets/rust.png"
));
static GITIGNORE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/hello-world/.gitignore"
));

/// Create a new cargo playdate package in an existing directory
#[derive(clap::Args, Debug)]
pub struct Init {
    #[clap(default_value = ".")]
    path: PathBuf,
    /// Use local playdate-rs crate
    #[clap(long)]
    use_local_playdate_rs: Option<PathBuf>,
}

impl Init {
    /// Initialize playdate project
    fn init_playdate_impl(
        new: bool,
        path: &Path,
        use_local_playdate_rs: &Option<PathBuf>,
    ) -> anyhow::Result<()> {
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
        if let Some(local_path) = use_local_playdate_rs {
            Command::new("cargo")
                .arg("add")
                .arg("playdate-rs")
                .arg("--path")
                .arg(local_path)
                .check(true)?;
        } else {
            Command::new("cargo")
                .arg("add")
                .arg("playdate-rs")
                .check(true)?;
        }
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
    pub fn init_playdate(
        new: bool,
        path: &PathBuf,
        use_local_playdate_rs: &Option<PathBuf>,
    ) -> anyhow::Result<()> {
        let cwd = std::env::current_dir()?;
        std::env::set_current_dir(path)?;
        let result = Self::init_playdate_impl(new, path, use_local_playdate_rs);
        std::env::set_current_dir(cwd)?;
        result
    }
}

impl Runnable for Init {
    fn run(&self) -> anyhow::Result<()> {
        // Create cargo project
        info!("Initializing cargo project ...");
        info!("➔  cargo init --lib {}", self.path.to_string_lossy());
        Command::new("cargo")
            .arg("init")
            .arg("--lib")
            .check(false)?;
        // Initialize playdate project
        Self::init_playdate(false, &self.path, &self.use_local_playdate_rs)?;
        Ok(())
    }
}
