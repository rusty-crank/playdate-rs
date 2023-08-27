# playdate-cli

The CLI tools for [github.com/rusty-crank/playdate-rs](https://github.com/rusty-crank/playdate-rs).

Install it by running: `cargo install playdate-cli`.

# Getting Started

1. Ensure that the environment variable `PLAYDATE_SDK_PATH` is correctly set.
   * Skip this step on macOS, as the crate will automatically find the SDK by checking the default installation location.
2. Install the CLI tool: `cargo install playdate-cli`
3. Create a new project: `cargo playdate new hello-world`
4. Run the project: `cd hello-world && cargo playdate run`

Please refer to [github.com/rusty-crank/playdate-rs](https://github.com/rusty-crank/playdate-rs) for more details on how to use the `playdate-rs` package, and [docs.rs/playdate-rs](https://docs.rs/playdate-rs/latest/playdate_rs/) for the API documentation.

# Available commands

### `cargo playdate new <PATH>`

```
Create a new cargo playdate package

Usage: cargo-playdate new <PATH>

Arguments:
  <PATH>

Options:
  -h, --help  Print help
```

### `cargo playdate init`

```
Create a new cargo playdate package in an existing directory

Usage: cargo-playdate init [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -h, --help  Print help
```

### `cargo playdate build`

```
Compile the current package

Usage: cargo-playdate build [OPTIONS]

Options:
      --release              Build the project in release mode, with optimizations
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
  -F, --features <FEATURES>  Space-separated list of features to activate
  -p, --package <PACKAGE>    Package to process (see `cargo help pkgid`)
      --device               Build for the real device (default is simulator)
  -h, --help                 Print help
```

### `cargo playdate run`

```
Run the local package on the playdate simulator or a device

Usage: cargo-playdate run [OPTIONS]

Options:
      --release              Build the project in release mode, with optimizations
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
  -F, --features <FEATURES>  Space-separated list of features to activate
  -p, --package <PACKAGE>    Package to process (see `cargo help pkgid`)
      --device               Build for the real device (default is simulator)
  -h, --help                 Print help
```