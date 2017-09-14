#![allow(unused)]

use mk66::{gpio, clock, pit};
use kernel::hil::time::{Time, Alarm, Frequency, Client};
use tests::blink;

static mut INTERVAL: u32 = 18_000_000;
static mut UP: bool = false;

static mut LAST_TIME: u32 = 0;

struct LedClient;
impl Client for LedClient {
    fn fired(&self) {
        unsafe {
            let now = pit::PIT.now();
            let gap = now - LAST_TIME;
            let wasted = gap - INTERVAL;
            blink::led_toggle();
            if INTERVAL > 18_000_000 || INTERVAL < 4_000_000 {
                UP = !UP;
            }
            INTERVAL = if UP {INTERVAL + 1_000_000} else { INTERVAL - 1_000_000 };
            LAST_TIME = now;
            pit::PIT.set_alarm(INTERVAL);
            debug!("Interval: {}, Time: {}, Gap: {}, Overhead: {}", INTERVAL, now, gap, wasted);
        }
    }
}

static LED: LedClient = LedClient;

pub fn alarm_test() {
    assert!(pit::PitFrequency::frequency() == 36_000_000,
            "Timer frequency does not match expected value!");

    unsafe {
        pit::PIT.set_client(&LED);
        pit::PIT.set_alarm(18_000_000);
    }
}
