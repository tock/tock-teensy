use mk66::rnga;
use kernel::hil::rng::{RNG, Client, Continue};
use tests::alarm;

struct RngTest;

impl Client for RngTest {
    fn randomness_available(&self, randomness: &mut Iterator<Item = u32>) -> Continue {
        match randomness.next() {
            Some(num) => {
                println!("Random number: {}", num);
                Continue::Done
            }
            None => Continue::More
        }
    }
}

static RNG: RngTest = RngTest;   

pub fn rng_test() {
    unsafe {
        rnga::RNGA.set_client(&RNG);
        
        alarm::loop_500ms(|| {  
            rnga::RNGA.get();
        });
    }
}
