# Example

This example demonstrates how to associate your custom events with `bevy_metamask`.


## Prerequisite

Install [trunk](https://trunkrs.dev/)

```shell
# Install via homebrew on Mac, Linux or Windows (WSL).
brew install trunk

# Install a release binary (great for CI).
# You will need to specify a value for ${VERSION}.
wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

# Install via cargo.
cargo install --locked trunk
# Until wasm-bindgen has pre-built binaries for Apple M1, M1 users will
# need to install wasm-bindgen manually.
cargo install --locked wasm-bindgen-cli
```

## How to Run

Spawn a web server which binds [127.0.0.1:8080](http://127.0.0.1:8080/) as a default.

```shell
bevy_metamask_example $ trunk serve
```
