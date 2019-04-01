use capsules::alarm::AlarmDriver;
use kernel;
use kernel::capabilities;
use components::Component;
use kernel::create_capability;
use kernel::static_init;
use mk66;

pub struct AlarmComponent {
    board_kernel: &'static kernel::Kernel,
}

impl AlarmComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> Self {
        AlarmComponent {
            board_kernel: board_kernel
        }
    }
}

impl Component for AlarmComponent {
    type Output = &'static AlarmDriver<'static, mk66::pit::Pit<'static>>;

    unsafe fn finalize(&mut self) -> Option<Self::Output> {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        mk66::pit::PIT.init();

        let alarm = static_init!(
                AlarmDriver<'static, mk66::pit::Pit>,
                AlarmDriver::new(&mk66::pit::PIT,
                                self.board_kernel.create_grant(&grant_cap))
            );
        mk66::pit::PIT.set_client(alarm);
        Some(alarm)
    }
}
