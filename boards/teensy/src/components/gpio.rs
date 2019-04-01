use capsules;
use components::{Component, ComponentWithDependency};
use kernel::capabilities;
use kernel::create_capability;
use kernel::static_init;
use mk66;



type PinHandle = &'static mk66::gpio::Gpio<'static>;

pub struct GpioComponent {
    pins: Option<&'static [PinHandle]>,
    board_kernel: &'static kernel::Kernel,
}

impl GpioComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> Self {
        GpioComponent {
            pins: None,
            board_kernel: board_kernel
        }
    }
}

impl Component for GpioComponent {
    type Output = &'static capsules::gpio::GPIO<'static, mk66::gpio::Gpio<'static>>;

    unsafe fn finalize(&mut self) -> Option<Self::Output> {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        if self.pins.is_none() {
            return None;
        }

        let gpio = static_init!(
                capsules::gpio::GPIO<'static, mk66::gpio::Gpio<'static>>,
                capsules::gpio::GPIO::new(self.pins.unwrap(), self.board_kernel.create_grant(&grant_cap))
            );

        for pin in self.pins.unwrap().iter() {
            pin.set_client(gpio);
        }

        Some(gpio)
    }
}

impl ComponentWithDependency<&'static [PinHandle]> for GpioComponent {
    fn dependency(&mut self, pins: &'static [PinHandle]) -> &mut Self {
        self.pins = Some(pins);

        self
    }
}

