# stratus-cloud
Trying to build cloud like services in a home cluster. Stratus/strato: flat/layered and smooth.

The goal is to have a collection of Applications that can be deployed in one, or many, clusters running on multiple machines in a network. For example:

* A small cluster with color screens to draw funky graphics on.
* A small cluster of Raspberry Pi zero's to get ambient data.

These are two separate applications, and they could run on the same cluster, or in different logical clusters using the same hardware, or run on a separate cluster with different underlying hardware.

# Why are we doing this?

Educational purposes, and to finally get some of the HW I have at home to do something useful!

# Project structure

This repo will be divided mostly between Applications and Infrastructure code.

Inside each folder there will be a collection of self-contained items or tools that have will have a given purpose.

* Application: Holds application specific code. New app? create a new folder!
* Infrastructure: Holds code to provision/deploy a cluster where applications can be run. Want a new cluster? want a remixed version? create a new folder!

# Environment
If targetting a Raspberry Pi Zero, care must be taken as the usual gcc/linker available under the tag *arm-linux-gnueabihf* produces code that is compatible with armv7 in reality, not with armv6 for the Pi Zero. Quick solution would be to checkout the tools repo, from the official Raspberry Pi GitHub area, and set environment to point to it, eg:

```bash
git clone git@github.com:raspberrypi/tools.git raspberrypi_tools

cd project_folder
cargo clean
# assuming the checkout was one in the home directory.
export PATH="$HOME/raspberrypi_tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH"
cargo build --target arm-unknown-linux-gnueabihf
```
This will yield a viable binary that can then be copied onto the Pi Zero and executed successfully.
