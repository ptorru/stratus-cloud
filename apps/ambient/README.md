# Ambient


# Compilation
We have set up a cross-compilation environment.

## Requirements
For the cross compilation we need to have the toolchain for the Raspberry Pi Zero available:

```bash
# Add the ARM target to the Rust environment.

# Download the tool chain from the Raspberry Pi tool repository.
# Takes about 8 mins.
git clone git@github.com:raspberrypi/tools.git raspberrypi_tools
```

## Compiling Server
After installing the requirements we are ready to compile/copy to the pi via:
```bash
export IPADD=192.168.1.182
export PIUSR=pi

# Options are:
#   * No arguments: build only
#   * clean: clean then build, nothing else
#   * send: builds and sends binary
#   * run: builds, sends, and runs binary
source build.sh run
```

## Compiling Client
```bash
cargo run --bin helloworld-client
```
## Using clean

I have added the clean target on the build script because I have found that cargo will not recompile/relink your design after a tool-chain change. You may get confused and think you are compiling with a different toolchain and you are not! (It did happen to me! :-( )
