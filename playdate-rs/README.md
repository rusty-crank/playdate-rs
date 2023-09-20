# playdate-rs

[![crates.io](https://img.shields.io/crates/v/playdate-rs?style=for-the-badge&logo=rust)](https://crates.io/crates/playdate-rs)
[![docs](https://img.shields.io/docsrs/playdate-rs/latest?style=for-the-badge&logo=docs.rs)](https://docs.rs/playdate-rs)
[![workflow-status](https://img.shields.io/github/actions/workflow/status/rusty-crank/playdate-rs/ci.yml?style=for-the-badge&logo=github&label=checks)](https://github.com/rusty-crank/playdate-rs/actions/workflows/ci.yml)

***Note: Prior to reaching v0.1.0, this is a work in progress. The API is incomplete, and breaking changes can occur frequently across versions.***

Safe binding for the [Playdate](https://play.date) SDK C-API that:

1. Is easy to use and well documented.
2. Designed with memory safety in mind.

Only works on Linux/macOS with the playdate simulator for now.

# Getting Started

1. Ensure that the environment variable `PLAYDATE_SDK_PATH` is correctly set.
   * Skip this step on macOS, as the crate will automatically find the SDK by checking the default installation location.
2. Install the CLI tool: `cargo install playdate-cli`
3. Create a new project: `cargo playdate new hello-world`
4. Run the project: `cd hello-world && cargo playdate run`

_Please refer to [Playdate CLI docs](playdate-cli/README.md) for all the available CLI commands._

# Application Bundling

The `cargo playdate build` command will automatically create a `target/<profile>/<package_name>.pdx` folder that can run on the simulator. For the device build, it will be located at `target/thumbv7em-none-eabihf/<profile>/<package_name>.pdx`.

## Game assets bundling

Please put all assets files under the `assets` folder in the project's root directory (the folder containing `Cargo.toml`). The CLI will automatically copy all contents to the `.pdx` folder. All supported resources will be transformed by the `pdc` compiler.

For more details, please refer to the [examples/hello-world](examples/hello-world) project.

##  `pdxinfo` generation and bundling

The CLI will automatically generate a `pdxinfo` file under the `.pdx` folder. There are two ways to set the content of the `pdxinfo` file:

1. Create a pdxinfo file in the project's root directory (the folder containing `Cargo.toml`). The CLI will automatically pick it up.
2. Create a `[package.metadata.pdxinfo]` section in `Cargo.toml`:

```toml
[package.metadata.pdxinfo]
name = "Your game name" # Default value: package.name
author = "Your Name" # Default value: package.authors
description = "Your game description" # Default value: package.description
bundle_id = "com.your-game.bundle-id" # Default value: "com.example." + package.name
image_path = "image/path" # Default value: empty string
launch_sound_path = "launch/sound/path" # Default value: empty string
content_warning = "Content warning" # Default value: empty string
content_warning2 = "Content warning 2" # Default value: empty string
```

_Note that all fields in `[package.metadata.pdxinfo]` are optional. The default value will be derived from other fields in `Cargo.toml`. Please refer to the `# Default value` comments above._

Example: [examples/hello-world/Cargo.toml](examples/hello-world/Cargo.toml).

# TODO

* [x] Linux / macOS simulator build
* [x] Cortex-M7F build
* [x] Run on real playdate device
* [ ] Support all public PlaydateSDK C-API
