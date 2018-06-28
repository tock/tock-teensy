use kernel::common::regs::ReadWrite;

#[repr(C)]
pub struct Registers {
    pub stctrlh: ReadWrite<u16, StatusAndControlHigh::Register>,
    pub stctrll: ReadWrite<u16>,
    pub tovalh:  ReadWrite<u16>,
    pub tovall:  ReadWrite<u16>,
    pub winh:    ReadWrite<u16>,
    pub winl:    ReadWrite<u16>,
    pub refresh: ReadWrite<u16, Refresh::Register>,
    pub unlock:  ReadWrite<u16, Unlock::Register>,
    pub tmrouth: ReadWrite<u16>,
    pub tmroutl: ReadWrite<u16>,
    pub rstcnt:  ReadWrite<u16>,
    pub presc:   ReadWrite<u16>,
}

pub const WDOG: *mut Registers = 0x40052000 as *mut Registers;

register_bitfields![u16,
    StatusAndControlHigh [
        WAITEN 7,
        STOPEN 6,
        DBGEN 5,
        ALLOWUPDATE 4,
        WINEN 3,
        IRQSTEN 2,
        CLKSRC 1,
        WDOGEN 0
    ],
    Refresh [
        KEY OFFSET(0) NUMBITS(16) [
            Key1 = 0xA602,
            Key2 = 0xB480
        ]
    ],
    Unlock [
        KEY OFFSET(0) NUMBITS(16) [
            Key1 = 0xC520,
            Key2 = 0xD928
        ]
    ]
];
