use regs::pit::*;
use core::mem;
use core::cell::Cell;
use kernel::hil::time::{Client, Time, Alarm, Frequency};
use nvic;
use clock::peripheral_clock_hz;
use sim;

pub static mut PIT: Pit<'static> = Pit::new();

pub struct Pit<'a> {
    pub client: Cell<Option<&'a Client>>
}

impl<'a> Pit<'a> {
    pub const fn new() -> Self {
        Pit {
            client: Cell::new(None)
        }
    }

    pub fn init(&self) {
        sim::enable_clock(sim::clocks::PIT);
        self.regs().mcr.write(MCR::MDIS::False +
                              MCR::FRZ::True);

        // Configure the lifetime timer.
        self.pit(0).ldval.set(0xFFFF_FFFF);
        self.pit(1).ldval.set(0xFFFF_FFFF);
        self.pit(1).tctrl.modify(TCTRL::CHN::True);

        // Enable the lifetime timer.
        self.pit(1).tctrl.modify(TCTRL::TEN::True);
        self.pit(0).tctrl.modify(TCTRL::TEN::True);
    }

    fn regs(&self) -> &mut Registers {
        unsafe { mem::transmute(PIT_BASE) }
    }

    fn pit(&self, index: usize) -> &mut PitRegisters {
        unsafe { mem::transmute(PIT_ADDRS[index])}
    }

    pub fn enable(&self) {
        self.pit(2).tctrl.modify(TCTRL::TEN::True);
    }

    pub fn is_enabled(&self) -> bool {
        self.pit(2).tctrl.is_set(TCTRL::TEN)
    }

    pub fn enable_interrupt(&self) {
        unsafe { nvic::enable(nvic::NvicIdx::PIT2); }
        self.pit(2).tctrl.modify(TCTRL::TIE::True);
    }

    pub fn set_counter(&self, value: u32) {
        self.pit(2).ldval.set(value);
    }

    pub fn get_counter(&self)  -> u32 {
        self.pit(2).ldval.get()
    }

    pub fn clear_pending(&self) {
        self.pit(2).tflg.modify(TFLG::TIF::True);
        unsafe { nvic::clear_pending(nvic::NvicIdx::PIT2); }
    }

    pub fn disable(&self) {
        self.pit(2).tctrl.modify(TCTRL::TEN::False);
    }

    pub fn disable_interrupt(&self) {
        self.pit(2).tctrl.modify(TCTRL::TIE::False);
    }

    pub fn set_client(&self, client: &'a Client) {
        self.client.set(Some(client));
    }

    pub fn handle_interrupt(&self) {
        self.disable();
        self.disable_interrupt();
        self.clear_pending();
        self.client.get().map(|client| { client.fired(); });
    }
}

pub struct PitFrequency;
impl Frequency for PitFrequency {
    fn frequency() -> u32 {
        peripheral_clock_hz()
    }
}

impl<'a> Time for Pit<'a> {
    type Frequency = PitFrequency;
    fn disable(&self) {
        Pit::disable(self);
        self.disable_interrupt();
        self.clear_pending();
    }

    fn is_armed(&self) -> bool {
        self.is_enabled()
    }
}

impl<'a> Alarm for Pit<'a> {
    fn now(&self) -> u32 {
        self.regs().ltmr64h.get();
        ::core::u32::MAX - self.regs().ltmr64l.get()
    }

    fn set_alarm(&self, ticks: u32) {
        Time::disable(self);
        self.set_counter(ticks);
        self.enable_interrupt();
        self.enable();
    }

    fn get_alarm(&self) -> u32 {
        self.get_counter()
    }
}

interrupt_handler!(pit2_handler, PIT2);
