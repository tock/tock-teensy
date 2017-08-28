#![allow(unused)]

use mk66::{gpio, clock, timer};
use kernel::hil::time::{Time, Timer, Frequency, Client};
use tests::blink;

static mut INTERVAL: u32 = 18_000_000;
static mut UP: bool = false;

struct LedOneshotClient;
impl Client for LedOneshotClient {
    fn fired(&self) {
        unsafe {
            blink::led_toggle();
            if INTERVAL > 18_000_000 || INTERVAL < 10_000 {
                UP = !UP;
            }
            INTERVAL = if UP {INTERVAL + 1_000_000} else { INTERVAL - 1_000_000 };
            println!("Oneshot: {}", INTERVAL);
            timer::PIT0.oneshot(INTERVAL);
        }
    }
}

struct LedRepeatClient;
impl Client for LedRepeatClient {
    fn fired(&self) {
        unsafe {
            blink::led_toggle();
            println!("Repeat");
        }
    }
}

static ONESHOT: LedOneshotClient = LedOneshotClient;
static REPEAT: LedRepeatClient = LedRepeatClient;

pub fn timer_oneshot_test() {
    clock::configure(72);
    assert!(timer::PitFrequency::frequency() == 36_000_000,
            "Timer frequency does not match expected value!");

    unsafe {
        timer::PIT0.set_client(&ONESHOT);
        timer::PIT0.oneshot(18_000_000);
    }
}

pub fn timer_repeat_test() {
    clock::configure(72);
    assert!(timer::PitFrequency::frequency() == 36_000_000,
            "Timer frequency does not match expected value!");

    unsafe {
        timer::PIT1.set_client(&REPEAT);
        timer::PIT1.repeat(18_000_000);
    }
}
