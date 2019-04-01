use mk66::rnga;
use kernel::ReturnCode;
use kernel::hil::entropy::{Entropy32, Client32, Continue};
use tests::alarm;

struct RngTest;

impl Client32 for RngTest {
    fn entropy_available(&self, entropy: &mut Iterator<Item = u32>,
                         error: ReturnCode) -> Continue {
        match error {
            ReturnCode::SUCCESS => {
                match entropy.next() {
                    Some(num) => {
                        println!("Entropy: {}", num);
                        Continue::Done
                    }
                    None => Continue::More
                }
            },
            _ => Continue::Done
        }
    }
}

static RNG: RngTest = RngTest;   
pub fn rng_test() {
    unsafe {
        rnga::ENTROPY.set_client(&RNG);
        
        alarm::loop_500ms(|| {  
            rnga::ENTROPY.get();
        });
    }
}
