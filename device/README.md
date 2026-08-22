# **ParrotProject: Device**

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


### Docker
Build the qemu instance for linux container:
```
docker run --rm -it -v "$(pwd)":/app -w /app rust:1.85-bookworm bash -c "
apt-get update && apt-get install -y build-essential pkg-config ninja-build libglib2.0-dev libpixman-1-dev libfdt-dev python3 python3-pip &&
cd qemu-custom && rm -rf build_linux && mkdir build_linux && cd build_linux &&
../configure --target-list=arm-softmmu && ninja
"
```

The result of the build is available at `/qemu_custom/build_linux`. 