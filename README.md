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
