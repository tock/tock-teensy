#![allow(unused)]

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

fn led_on() {
    use mk66::gpio;
    unsafe {
        let led = gpio::PC05.make_gpio();
        led.enable_output();
        led.set();
        loop {}
    }
}

pub fn fast_blink_test() {
    use mk66::osc;
    use mk66::mcg;
    use mk66::sim;

    unsafe { 
        osc::enable(osc::OscCapacitance::Load_10pF); 
        sim::set_dividers(1, 2, 3);
    }

    if let mcg::State::Fei(fei) = mcg::state() {
        led_on();
        fei.enable_xtal(mcg::OscRange::VeryHigh);
        let fbe = fei.use_external(mcg::Frdiv::Low16_High512);

        let pbe = fbe.enable_pll(27, 6);

        pbe.use_pll();
    } else {
    }

    blink_test();
}
