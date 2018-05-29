# TockOS port for Teensy 3.6

This repository is an experimental port of the Tock embedded operating system to
the Teensy 3.6.

## Installation

This repository depends on the standard Tock distribution as a git submodule 
in the `tock/` folder. To download this repo, use: 

```
git clone --recursive https://github.com/shaneleonard/tock-teensy.git
```

(note the `--recursive` flag, which is needed to download the `tock/` 
dependency).


## Compiling

To compile the kernel, simply run `make` from the top-level directory. You must
have the prerequiste build tools installed, as detailed in the
[Tock getting started guide](https://github.com/helena-project/tock/blob/master/doc/Getting_Started.md).

## Programming the Teensy

Connect the Teensy via USB to your computer, and run `make program` from the
root directory. You should see a prompt telling you to press the reset button on
your board. Once you press the button, `teensy-loader-cli` will flash the kernel
onto the board using the Teensy's builtin HalfKay bootloader.

## Blink

The `boards/teensy/src/tests` directory contains tests which can be run instead of running
the normal kernel main loop. To run `blink` from the kernel, edit
`tests/mod.rs` to the following:

```rust
// Set this function to run whatever test you desire. Test functions are named XXX_test by convention.
pub fn test() {
    blink::blink_test();
}

// Set this to true to make the kernel run the test instead of main.
pub const TEST: bool = true;
```

Then run `make program` and the kernel will be compiled and flashed to your
Teensy. You should see the orange LED blinking!

To get a blink with UART console output on TX0, run `print::print_test()` instead.

## Packages you need

You'll need the ARM cross compiler on many systems:

```
sudo apt-get install gcc-arm-none-eabi
```
