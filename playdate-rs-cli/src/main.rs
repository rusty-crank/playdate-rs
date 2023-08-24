#[macro_use]
extern crate log;

use clap::Parser;

#[macro_use]
mod util;

mod build;
mod init;
mod new;
mod run;

/// playdate rust development tools
#[derive(Parser, Debug)]
#[command(author = "Wenyu Zhao <wenyu.zhao@anu.edu.au>", version = "0.0.1")]
enum Commands {
    #[clap(alias = "b")]
    Build(build::Build),
    #[clap(alias = "r")]
    Run(run::Run),
    New(new::New),
    Init(init::Init),
}

trait Runnable<T = ()> {
    fn run(&self) -> anyhow::Result<T>;
}

fn main() -> anyhow::Result<()> {
    println!("playdate-rs-cli");
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .init();

    let args = std::env::args();
    let mut args = args.collect::<Vec<_>>();
    println!("{:?}", args);
    if args[1] == "playdate" {
        args = args[1..].to_vec();
    }
    let command = Commands::parse_from(args);

    match command {
        Commands::Build(cmd) => {
            cmd.run()?;
        }
        Commands::Run(cmd) => {
            cmd.run()?;
        }
        Commands::New(cmd) => {
            cmd.run()?;
        }
        Commands::Init(cmd) => {
            cmd.run()?;
        }
    }
    Ok(())
}
