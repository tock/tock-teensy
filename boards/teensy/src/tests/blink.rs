#![allow(unused)]

use mk66::{gpio, clock};

pub fn delay() {
    unsafe {
        for _ in 1..2000_000 {
            asm!("nop" :::: "volatile");
        }
    }
}

pub fn blink_test() {
    unsafe {
        let led = gpio::PC05.make_gpio();
        led.enable_output();
        loop {
            delay();
            led.toggle();
        }
    }
}

fn led_on() {
    unsafe {
        let led = gpio::PC05.make_gpio();
        led.enable_output();
        led.set();
        loop {}
    }
}

pub fn led_toggle() {
    unsafe {
        gpio::PC05.reclaim();
        let led = gpio::PC05.make_gpio();
        led.enable_output();
        led.toggle();
    }
}

pub fn fast_blink_test() {
    clock::configure(120);

    blink_test();
}
