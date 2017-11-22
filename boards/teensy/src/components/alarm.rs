use mk66;
use kernel;
use components::Component;
use capsules::alarm::AlarmDriver;

pub struct AlarmComponent;

impl AlarmComponent {
    pub fn new() -> Self {
        AlarmComponent {}
    }
}

impl Component for AlarmComponent {
    type Output = &'static AlarmDriver<'static, mk66::pit::Pit<'static>>;

    unsafe fn finalize(&mut self) -> Option<Self::Output> {
        mk66::pit::PIT.init();

        let alarm = static_init!(
                AlarmDriver<'static, mk66::pit::Pit>,
                AlarmDriver::new(&mk66::pit::PIT,
                                 kernel::Grant::create())
            );
        mk66::pit::PIT.set_client(alarm);
        Some(alarm)
    }
}
