use mk66;
use kernel;
use capsules::rng::SimpleRng;
use components::Component;

pub struct RngaComponent;

impl RngaComponent {
    pub fn new() -> Self {
        RngaComponent {}
    }
}

impl Component for RngaComponent {
    type Output = &'static SimpleRng<'static, mk66::rnga::Rnga<'static>>;

    unsafe fn finalize(&mut self) -> Option<Self::Output> {
        mk66::rnga::RNGA.init();
        
        let rng = static_init!(
                SimpleRng<'static, mk66::rnga::Rnga>,
                SimpleRng::new(&mk66::rnga::RNGA, kernel::Grant::create())
        );

        mk66::rnga::RNGA.set_client(rng);

        Some(rng)
    }
}
