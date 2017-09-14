use tests::blink;

pub fn print_test() {
    loop {
        println!("Hello World!");
        blink::delay();
        blink::led_toggle();
    }
}

pub fn panic_test() {
    panic!("This is a kernel panic.");
}

pub fn debug_test() {
    loop {
        debug!("This is a debug message from the kernel.");
        blink::delay();
        blink::led_toggle();
    }
}
