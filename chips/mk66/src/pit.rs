use core::mem;
use core::cell::Cell;
use kernel::hil::time::{Client, Time, Alarm, Frequency};
use nvic;
use clock::peripheral_clock_hz;

use kernel::common::registers::{register_bitfields, ReadWrite, ReadOnly};
  
#[repr(C)]
pub struct Registers {
    pub mcr: ReadWrite<u32, ModuleControl::Register>,
    _reserved0: [ReadOnly<u32>; 55],
    pub ltmr64h: ReadOnly<u32>,
    pub ltmr64l: ReadOnly<u32>,
    _reserved1: [ReadOnly<u32>; 2],
    pub timers: [PitRegisters; 4]
}

#[repr(C)]
pub struct PitRegisters {
    pub ldval: ReadWrite<u32>,
    pub cval: ReadOnly<u32>,
    pub tctrl: ReadWrite<u32, TimerControl::Register>,
    pub tflg: ReadWrite<u32, TimerFlag::Register>
}

register_bitfields! [u32,
    ModuleControl [
        MDIS 1,
        FRZ 0
    ],
    TimerControl [
        CHN 2,
        TIE 1,
        TEN 0
    ],
    TimerFlag [
        TIF 0
    ]
];

pub const PIT_BASE: *mut Registers = 0x4003_7000 as *mut Registers;
pub const PIT_ADDRS: [*mut PitRegisters; 4] = [0x4003_7100 as *mut PitRegisters,
                                               0x4003_7110 as *mut PitRegisters,
                                               0x4003_7120 as *mut PitRegisters,
                                               0x4003_7130 as *mut PitRegisters];

pub static mut PIT: Pit<'static> = Pit::new();

pub struct Pit<'a> {
    pub client: Cell<Option<&'a Client>>,
    alarm: Cell<u32>
}

impl<'a> Pit<'a> {
    pub const fn new() -> Self {
        Pit {
            client: Cell::new(None),
            alarm: Cell::new(0)
        }
    }

    pub fn init(&self) {
        use sim::{clocks, Clock};

        clocks::PIT.enable();
        self.regs().mcr.write(ModuleControl::MDIS::CLEAR +
                              ModuleControl::FRZ::SET);

        // Configure the lifetime timer.
        self.pit(0).ldval.set(0xFFFF_FFFF);
        self.pit(1).ldval.set(0xFFFF_FFFF);
        self.pit(1).tctrl.modify(TimerControl::CHN::SET);

        // Enable the lifetime timer.
        self.pit(1).tctrl.modify(TimerControl::TEN::SET);
        self.pit(0).tctrl.modify(TimerControl::TEN::SET);
    }

    fn regs(&self) -> &mut Registers {
        unsafe { mem::transmute(PIT_BASE) }
    }

    fn pit(&self, index: usize) -> &mut PitRegisters {
        unsafe { mem::transmute(PIT_ADDRS[index])}
    }

    pub fn enable(&self) {
        self.pit(2).tctrl.modify(TimerControl::TEN::SET);
    }

    pub fn is_enabled(&self) -> bool {
        self.pit(2).tctrl.is_set(TimerControl::TEN)
    }

    pub fn enable_interrupt(&self) {
        unsafe { nvic::enable(nvic::NvicIdx::PIT2); }
        self.pit(2).tctrl.modify(TimerControl::TIE::SET);
    }

    pub fn set_counter(&self, value: u32) {
        self.pit(2).ldval.set(value);
    }

    pub fn get_counter(&self)  -> u32 {
        self.pit(2).ldval.get()
    }

    pub fn clear_pending(&self) {
        self.pit(2).tflg.modify(TimerFlag::TIF::SET);
        unsafe { nvic::clear_pending(nvic::NvicIdx::PIT2); }
    }

    pub fn disable(&self) {
        self.pit(2).tctrl.modify(TimerControl::TEN::CLEAR);
    }

    pub fn disable_interrupt(&self) {
        self.pit(2).tctrl.modify(TimerControl::TIE::CLEAR);
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
        self.alarm.set(ticks.wrapping_sub(self.now()));
        self.set_counter(self.alarm.get());
        self.enable_interrupt();
        self.enable();
    }

    fn get_alarm(&self) -> u32 {
        self.alarm.get()
    }
}
