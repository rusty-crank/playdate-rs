# playdate-rs

***Note: Prior to reaching v0.1.0, this is a work in progress. The API is incomplete, and breaking changes can occur frequently across versions.***

Safe binding for the [Playdate](https://play.date) SDK C-API that:

1. Is easy to use and well documented.
2. Does not require the Rust nightly toolchain.
3. Is designed with memory safety in mind.

Only works on Linux/macOS with the playdate simulator for now.

# Getting Started

1. Ensure that the environment variable `PLAYDATE_SDK_PATH` is correctly set.
   * Skip this step on macOS, as the crate will automatically find the SDK by checking the default installation location.
2. Install the CLI tool: `cargo install playdate-cli`
3. Create a new project: `cargo playdate new hello-world`
4. Run the project: `cd hello-world && cargo playdate run`

Please refer to [github.com/rusty-crank/playdate-rs](https://github.com/rusty-crank/playdate-rs) for more details on how to use the package, and [docs.rs/playdate-rs](https://docs.rs/playdate-rs/latest/playdate_rs/) for the API documentation.
