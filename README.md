# Bevy Metamask

[![crates.io](https://img.shields.io/crates/v/bevy_metamask.svg)](https://crates.io/crates/bevy_metamask)
[![docs](https://docs.rs/bevy_metamask/badge.svg)](https://docs.rs/bevy_metamask)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)

Bevy meets Metamask! This plugin is only supported in wasm target. The core concept of the plugin is a communication via a request event and the corresponding response event. Whenever you send a request event, `IoTaskPool` handles the request and then send the response event.

## Getting Started

1. Implement a request event `E` which implements [`MetamaskRequestEvent`](./bevy_metamask/src/lib.rs)
2. Implement corresponding response event `E::ResponseEvent (as MetamaskRequestEvent)`
3. Add [`MetamaskPlugin<E>`](./bevy_metamask/src/lib.rs)

```rust
struct EthereumRequestEvent;

struct EthereumResponseEvent(String);

impl bevy_metamask::MetamaskRequestEvent for EthereumRequestEvent {
    type ResponseEvent = EthereumResponseEvent;

    fn to_req(&self) -> bevy_metamask::MetamaskRequest {
        bevy_metamask::MetamaskRequest::BlockNumber(bevy_metamask::BlockNumber)
    }

    fn from_resp(resp: bevy_metamask::MetamaskResponse) -> Self::ResponseEvent {
        match resp {
            bevy_metamask::MetamaskResponse::BlockNumber(block_number) => {
                EthereumResponseEvent(block_number)
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    // ...
    app.add_plugin(MetamaskPlugin::<EthereumRequestEvent>::new())
    // ...
}
```

In order to separate response events such as `Call` to A contract and `Call` to B contract, you should define two dintinct events and add two plugins.

For further information, please see an [example](./bevy_metamask_example).

## Compatible Bevy versions

Compatibility of `bevy_metamask` versions:
| `bevy_metamask` | `bevy` |
| :-------------- | :----- |
| `0.1`           | `0.9`  |
| -               | -      |
