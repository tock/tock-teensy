use common::regs::{RW, RO};

#[repr(C, packed)]
pub struct Registers {
    pub mcr: RW<u32>,
    _reserved0: [RO<u32>; 55],
    pub ltmr64h: RO<u32>,
    pub ltmr64l: RO<u32>,
    _reserved1: [RO<u32>; 2],
    pub timers: [TimerRegisters; 4]
}

#[repr(C, packed)]
pub struct TimerRegisters {
    pub ldval: RW<u32>, 
    pub cval: RO<u32>,
    pub tctrl: RW<u32>,
    pub tflg: RW<u32>
}

bitfields! [u32,
    MCR [
        MDIS 1,
        FRZ 0
    ],
    TCTRL [
        CHN 2,
        TIE 1,
        TEN 0
    ],
    TFLG [
        TIF 0
    ]
];

pub const PIT: *mut Registers = 0x4003_7000 as *mut Registers;
pub const TIMER_ADDRS: [*mut TimerRegisters; 4] = [0x4003_7100 as *mut TimerRegisters,
                                                   0x4003_7110 as *mut TimerRegisters,
                                                   0x4003_7120 as *mut TimerRegisters,
                                                   0x4003_7130 as *mut TimerRegisters];
