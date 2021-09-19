# Poller
This simple app just says hi to the nodes it finds on the network.

# Compiling

## Sever for Raspberry Pi Zero
```bash
# Remember to have the correct linker available in the enviroment
# for the Pi Zero
cargo build --bin helloworld-server --target arm-unknown-linux-gnueabihf
```

## Running client for Master
```bash
cargo run --bin helloworld-client
```

# Credit
This app is mostly (if not entirely) based on the [rust-grpc-example](https://github.com/swiftdiaries/rust-grpc-example) by [Swiftdiaries](https://github.com/swiftdiaries) on GitHub.
