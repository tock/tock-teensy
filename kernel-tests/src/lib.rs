#![no_std]
#![feature(asm,const_fn,drop_types_in_const,lang_items,compiler_builtins_lib)]

extern crate capsules;
extern crate compiler_builtins;
extern crate kernel;

#[allow(dead_code)]
extern crate mk66;

#[macro_use]
extern crate common;

// Test modules
mod blink;
mod registers;

// Set this function to run whatever test you desire. Test functions are named XXX_test by convention.
pub fn test() {
    registers::register_test();
}

// Set this to true to make the kernel run the test instead of main.
pub const TEST: bool = true;
