#[macro_use]
extern crate log;

use clap::Parser;

mod build;
mod run;

/// playdate rust development tools
#[derive(Parser, Debug)]
#[command(author = "Wenyu Zhao <wenyu.zhao@anu.edu.au>", version = "0.0.1")]
enum Commands {
    Build(build::Build),
    Run(run::Run),
}

trait Runnable<T = ()> {
    fn run(&self) -> anyhow::Result<T>;
}

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .init();

    let command = Commands::parse();

    match command {
        Commands::Build(cmd) => {
            cmd.run()?;
        }
        Commands::Run(cmd) => {
            cmd.run()?;
        }
    }
    Ok(())
}
