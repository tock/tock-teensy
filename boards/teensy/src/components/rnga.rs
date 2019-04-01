use capsules::rng;
use components::Component;
use kernel;
use kernel::{capabilities, create_capability, static_init};
use kernel::hil::entropy::Entropy32;
use kernel::hil::rng::Rng;
use mk66;

pub struct RngaComponent {
        board_kernel: &'static kernel::Kernel,

}

impl RngaComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> Self {
        RngaComponent {board_kernel: board_kernel}
    }
}

impl Component for RngaComponent {
    type Output = &'static rng::RngDriver<'static>;

    unsafe fn finalize(&mut self) -> Option<Self::Output> {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        let entropy_to_random = static_init!(
            rng::Entropy32ToRandom<'static>,
            rng::Entropy32ToRandom::new(&mk66::rnga::ENTROPY)
        );
        let rng = static_init!(
            rng::RngDriver<'static>,
            rng::RngDriver::new(
                entropy_to_random,
                self.board_kernel.create_grant(&grant_cap)
            )
        );
        mk66::rnga::ENTROPY.init();
        mk66::rnga::ENTROPY.set_client(entropy_to_random);        
        entropy_to_random.set_client(rng);

        Some(rng)
    }
}
