# SSD1306 Raspberry Pi Examples
Simple example to show displays driven by the SSD1306 chip.

# Compiling
If using a Raspberry Pi Zero compiling will take a while. That's why we have set up a cross-compilation environment. Cross-compile means we compile the code in our local machine but we execute it in the Raspberry Pi.

## Requirements
For the cross compilation we need to have the toolchain for the Raspberry Pi Zero available:

### Musl Tool Chain
This is the default tool chain for this repo, download from [musl.cc](https://musl.cc/)

### Raspberry Pi Tools
```bash
# Download the tool chain from the Raspberry Pi tool repository.
# Takes about 8 mins.
git clone git@github.com:raspberrypi/tools.git raspberrypi_tools

# Add the ARM target to the Rust environment.
export PATH="$(pwd)/raspberrypi_tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH"
```

## Running the compilation
After installing the requirements we are ready to compile/copy to the pi via:
```bash
# Use graphics-spi for the spi example
cargo clean; cargo build --example graphics-i2c --release

# Transfer binary to your Pi
# Fill in target with target description for your tool chain.
scp target/<TARGET>/release/examples/graphics-i2c pi@<IP-ADDRESS>:graphics-i2c-xcomp
```
