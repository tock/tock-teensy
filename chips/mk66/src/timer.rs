use regs::timer::*;
use core::mem;
use core::cell::Cell;
use kernel::hil::time::{Client, Time, Timer, Frequency};
use nvic;
use clock::peripheral_clock_hz;
use sim;

#[derive(Copy, Clone)]
pub enum TimerMode {
    Oneshot,
    Repeat
}

pub struct Pit<'a> {
    index: usize,
    mode: Cell<TimerMode>,
    regs: *mut TimerRegisters,
    pub client: Cell<Option<&'a Client>>
}

pub static mut PIT0: Pit<'static> = Pit::new(0);
pub static mut PIT1: Pit<'static> = Pit::new(1);
pub static mut PIT2: Pit<'static> = Pit::new(2);
pub static mut PIT3: Pit<'static> = Pit::new(3);

pub fn init() {
    sim::enable_clock(sim::clocks::PIT);
    unsafe {
        (*PIT).mcr.write(MCR::MDIS::False +
                         MCR::FRZ::True);
    }
}

impl<'a> Pit<'a> {
    pub const fn new(index: usize) -> Self {
        Pit {
            index: index,
            mode: Cell::new(TimerMode::Repeat),
            regs: TIMER_ADDRS[index],
            client: Cell::new(None)
        }
    }

    fn regs(&self) -> &mut TimerRegisters {
        unsafe { mem::transmute(self.regs) }
    }

    pub fn enable(&self) {
        self.regs().tctrl.modify(TCTRL::TEN::True);
    }

    pub fn is_enabled(&self) -> bool {
        self.regs().tctrl.is_set(TCTRL::TEN)
    }

    pub fn enable_interrupt(&self) {
        let irq = match self.index {
            0 => nvic::NvicIdx::PIT0,
            1 => nvic::NvicIdx::PIT1,
            2 => nvic::NvicIdx::PIT2,
            3 => nvic::NvicIdx::PIT3,
            _ => panic!("Invalid timer index: {}", self.index)
        };

        unsafe { nvic::enable(irq); }
        self.regs().tctrl.modify(TCTRL::TIE::True);
    }

    pub fn clear_pending(&self) {
        let irq = match self.index {
            0 => nvic::NvicIdx::PIT0,
            1 => nvic::NvicIdx::PIT1,
            2 => nvic::NvicIdx::PIT2,
            3 => nvic::NvicIdx::PIT3,
            _ => panic!("Invalid timer index: {}", self.index)
        };

        self.regs().tflg.modify(TFLG::TIF::True);
        unsafe {nvic::clear_pending(irq); }
    }

    pub fn timer_complete(&self) -> bool {
        self.regs().tflg.is_set(TFLG::TIF)
    }

    pub fn disable(&self) {
        self.regs().tctrl.modify(TCTRL::TEN::False);
    }

    pub fn disable_interrupt(&self) {
        self.regs().tctrl.modify(TCTRL::TIE::False);
    }

    pub fn chain(&self) {
        assert!(self.index != 0, "Timer 0 cannot be chained!");
        self.regs().tctrl.modify(TCTRL::CHN::True);
    }

    pub fn unchain(&self) {
        self.regs().tctrl.modify(TCTRL::CHN::False);
    }

    pub fn set_client(&self, client: &'a Client) {
        self.client.set(Some(client));
    }

    pub fn set_counter(&self, value: u32) {
        self.regs().ldval.set(value);
    }

    pub fn get_counter(&self) -> u32 {
        self.regs().cval.get()
    }

    pub fn set_mode(&self, mode: TimerMode) {
        self.mode.set(mode);
    }

    pub fn handle_interrupt(&self) {
        self.clear_pending(); 

        match self.mode.get() {
            TimerMode::Repeat => (),
            TimerMode::Oneshot => self.disable()
        };

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

impl<'a> Timer for Pit<'a> {
    fn oneshot(&self, interval: u32) {
        Time::disable(self);
        self.set_counter(interval);
        self.set_mode(TimerMode::Oneshot);
        self.enable_interrupt();
        self.enable();
    }

    fn repeat(&self, interval: u32) {
        Time::disable(self);
        self.set_counter(interval);
        self.set_mode(TimerMode::Repeat);
        self.enable_interrupt();
        self.enable();
    }
}

interrupt_handler!(timer0_handler, PIT0);
interrupt_handler!(timer1_handler, PIT1);
interrupt_handler!(timer2_handler, PIT2);
interrupt_handler!(timer3_handler, PIT3);
