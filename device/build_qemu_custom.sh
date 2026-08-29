cd ./qemu-custom
rm -rf build
mkdir build
cd build
../configure --target-list=arm-softmmu
ninja