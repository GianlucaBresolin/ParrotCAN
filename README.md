# **ParrotProject**

### **Submodule: QEMU-Custom**
QEMU-Custom is a custom version of QEMU with a `virtual-can-controller` device
added. This dependency is treated as a git submodule, so after cloning the
repository, run the following command to initialize the submodule:

```
git submodule update --init --recursive
```
---
### **Building and Running QEMU**

Run the following command to build and run a QEMU instance of the ParrotProject:

```
cargo build
cargo run
```
--- 
This project was developed as part of the *Cyberphysical and IoT Systems* course
at the University of Padua.
