use mk66::clock;
use tests::blink;

pub fn print_test() {
    clock::configure(36);

    loop {
        println!("Hello World!");
        blink::delay();
        blink::led_toggle();
    }
}

pub fn panic_test() {
    clock::configure(72);
    panic!("This is a drill.");
}
