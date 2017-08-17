#![no_std]
#![feature(asm,const_fn,drop_types_in_const,lang_items,compiler_builtins_lib)]

extern crate capsules;
extern crate compiler_builtins;
extern crate kernel;

#[allow(dead_code)]
extern crate mk66;

// Test modules
mod blink;

// Set this function to run whatever test you desire. Test functions are named XXX_test by convention.
pub fn test() {
    blink::blink_test();

    loop {}
}

// Set this to true to make the kernel run the test instead of main.
pub const TEST: bool = false;
