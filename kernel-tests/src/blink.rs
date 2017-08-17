
unsafe fn delay() {
    for _ in 1..2000_000 {
        asm!("nop" :::: "volatile");
    }
}

pub fn blink_test() {
    use mk66::gpio;

    unsafe {
        let led = gpio::PC05.make_gpio();
        led.enable_output();
        loop {
            delay();
            led.toggle();
        }
    }
}

