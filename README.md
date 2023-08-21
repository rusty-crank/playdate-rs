# playdate-rs

Bindings for the [Playdate](https://play.date) SDK C-API.

Only works on Linux with the playdate simulator for now.

# Getting Started

1. Make sure the env variable `PLAYDATE_SDK_PATH` is correctly set.
   * Skip this step on macOS. The crate will automatically find the SDK by checking the default install location.
2. `cd examples/hello_world`
3. `cargo playdate run`
