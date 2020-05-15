use capsules;
use components::{Component, ComponentWithDependency};
use kernel::static_init;
use mk66;

type PinHandle = &'static mk66::gpio::Gpio<'static>;

pub struct LedComponent {
    leds: Option<&'static [(&'static mk66::gpio::Gpio<'static>, kernel::hil::gpio::ActivationMode)]>
}

impl LedComponent {
    pub fn new(leds: &'static [(PinHandle, kernel::hil::gpio::ActivationMode)]) -> Self {
        LedComponent {
            leds: Some(leds)
        }
    }
}

impl Component for LedComponent {
    type Output = &'static capsules::led::LED<'static, mk66::gpio::Gpio<'static>>;

    unsafe fn finalize(&mut self) -> Option<Self::Output> {
        if self.leds.is_none() {
            return None;
        }

        let leds = static_init!(
                capsules::led::LED<'static, mk66::gpio::Gpio<'static>>,
                capsules::led::LED::new(self.leds.unwrap())
            );

        Some(leds)
    }
}

