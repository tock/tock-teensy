// Test modules
#[allow(dead_code)]
mod blink;

#[allow(dead_code)]
mod print;

#[allow(dead_code)]
mod alarm;

#[allow(dead_code)]
mod spi;

#[allow(dead_code)]
mod rng;

// Set this function to run whatever test you desire. Test functions are named XXX_test by convention.
pub fn test() {
    spi::spi_test();
}

// Set this to true to make the kernel run the test instead of main.
pub const TEST: bool = false;
