# playdate-rs

Bindings for the [Playdate](https://play.date) SDK C-API.

Only works on Linux/macOS with the playdate simulator for now.

# Getting Started

1. Make sure the env variable `PLAYDATE_SDK_PATH` is correctly set.
   * Skip this step on macOS. The crate will automatically find the SDK by checking the default install location.
2. `cargo install playdate-cli`
3. `cargo playdate new hello-world`
4. `cd hello-world && cargo playdate run`

# Application Bundling

The `cargo playdate build` command will automatically create a `target/<profile>/<package_name>.pdx` folder that can run on the simulator. For the device build, it will be located at `target/thumbv7em-none-eabihf/<profile>/<package_name>.pdx`.

## Game assets bundling

Please put all the assets file under the `assets` folder under the project root directory (the folder containing `Cargo.toml`). The CLI will automatically copy all the contents to the `.pdx` folder. All supported resources will be transformed by the `pdc` compiler.

Please refer to the [examples/hello-world](examples/hello-world) project for more details.

##  `pdxinfo` generation and bundling

The CLI will automatically generate a `pdxinfo` file under the `.pdx` folder. There are two ways to set the content of the `pdxinfo` file:

1. Create a pdxinfo file under the project root directory (the folder containing `Cargo.toml`). The CLI will automatically pick it up.
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

_Note that all the fields in `[package.metadata.pdxinfo]` are optional. The default value will be derived from other fields in `Cargo.toml`. Please refer to the `# Default value` comments above._

Example: [examples/hello-world/Cargo.toml](examples/hello-world/Cargo.toml).

# TODO

* [x] Linux / macOS simulator build
* [x] Cortex-M7F build
* [ ] Run on real playdate device
* [ ] Async support