use common::regs::{RW, RO};

#[repr(C, packed)]
pub struct Registers {
    pub mcr: RW<u32, ModuleControl>,
    _reserved0: [RO<u32>; 55],
    pub ltmr64h: RO<u32>,
    pub ltmr64l: RO<u32>,
    _reserved1: [RO<u32>; 2],
    pub timers: [PitRegisters; 4]
}

#[repr(C, packed)]
pub struct PitRegisters {
    pub ldval: RW<u32>, 
    pub cval: RO<u32>,
    pub tctrl: RW<u32, TimerControl>,
    pub tflg: RW<u32, TimerFlag>
}

bitfields! [u32,
    MCR ModuleControl [
        MDIS 1,
        FRZ 0
    ],
    TCTRL TimerControl [
        CHN 2,
        TIE 1,
        TEN 0
    ],
    TFLG TimerFlag [
        TIF 0
    ]
];

pub const PIT_BASE: *mut Registers = 0x4003_7000 as *mut Registers;
pub const PIT_ADDRS: [*mut PitRegisters; 4] = [0x4003_7100 as *mut PitRegisters,
                                               0x4003_7110 as *mut PitRegisters,
                                               0x4003_7120 as *mut PitRegisters,
                                               0x4003_7130 as *mut PitRegisters];
