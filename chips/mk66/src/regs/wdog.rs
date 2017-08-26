use common::regs::RW;

#[repr(C, packed)]
#[allow(dead_code)]
pub struct Registers {
    pub stctrlh: RW<u16>,
    pub stctrll: RW<u16>,
    pub tovalh: RW<u16>,
    pub tovall: RW<u16>,
    pub winh: RW<u16>,
    pub winl: RW<u16>,
    pub refresh: RW<u16>,
    pub unlock: RW<u16>,
    pub tmrouth: RW<u16>,
    pub tmroutl: RW<u16>,
    pub rstcnt: RW<u16>,
    pub presc: RW<u16>,
}

pub const WDOG: *mut Registers = 0x40052000 as *mut Registers;

bitfields![u16,
    STCTRLH [
        WAITEN 7,
        STOPEN 6,
        DBGEN 5,
        ALLOWUPDATE 4,
        WINEN 3,
        IRQSTEN 2,
        CLKSRC 1,
        WDOGEN 0
    ],
    REFRESH [
        KEY (0xFFFF, 0) [
            Key1 = 0xA602,
            Key2 = 0xB480
        ]
    ],
    UNLOCK [
        KEY (0xFFFF, 0) [
            Key1 = 0xC520,
            Key2 = 0xD928
        ]
    ]
];
