

# Compiling for Raspberry Pi 3,4
## Add dependencies to Cross container
```bash
docker build -t crossbuild:local .
```

## Compile
```bash
cross build --target=armv7-unknown-linux-gnueabihf --release --bin ambient_prom
```


# References
* [Fabian Writes - Cross-Compiling Rust apps for the Raspberry Pi](https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/)
