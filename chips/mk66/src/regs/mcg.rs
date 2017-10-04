use common::regs::{RW, RO};

pub const MCG: *mut Registers = 0x4006_4000 as *mut Registers;

#[repr(C, packed)]
pub struct Registers {
    pub c1: RW<u8, Control1>,
    pub c2: RW<u8, Control2>,
    pub c3: RW<u8>,
    pub c4: RW<u8>,
    pub c5: RW<u8, Control5>,
    pub c6: RW<u8, Control6>,
    pub s: RO<u8, Status>,
    _reserved0: RO<u8>,
    pub sc: RW<u8>,
    _reserved1: RO<u8>,
    pub atcvh: RW<u8>,
    pub atcvl: RW<u8>,
    pub c7: RW<u8>,
    pub c8: RW<u8>,
    pub c9: RW<u8>,
    _reserved2: RO<u8>,
    pub c11: RW<u8>,
    pub c12: RW<u8>,
    pub s2: RO<u8>,
    pub t3: RW<u8>
}

bitfields![u8,
    C1 Control1 [
        CLKS (Mask(0b11), 6) [
            LockedLoop = 0,
            Internal = 1,
            External = 2
        ],
        FRDIV (Mask(0b11), 4) [
            Low1_High32 = 0,
            Low2_High64 = 1,
            Low4_High128 = 2,
            Low8_High256 = 3,
            Low16_High512 = 4,
            Low32_High1024 = 5,
            Low64_High1280 = 6,
            Low128_High1536 = 7
        ],
        IREFS 2 [
            External = 0,
            SlowInternal = 1
        ],
        IRCLKEN 1 [
            Inactive = 0,
            Active = 1
        ],
        IREFSTEN 0 [
            IrefDisabledInStop = 0,
            IrefEnabledInStop = 1
        ]
    ],

    C2 Control2 [
        LOCKRE0 7 [],
        FCFTRIM 6 [],
        RANGE (Mask(0b11), 4) [
            Low = 0,
            High = 1,
            VeryHigh = 2
        ],
        HGO 3 [
            LowPower = 0,
            HighGain = 1
        ],
        EREFS 2 [
            External = 0,
            Oscillator = 1
        ],
        LP 1 [],
        IRCS 0 [
            SlowInternal = 0,
            FastInternal = 1
        ]
    ],

    C5 Control5 [
        PLLCLKEN 6 [],
        PLLSTEN 5 [],
        PRDIV (Mask(0b111), 0) [
            Div1 = 0, Div2 = 1, Div3 = 2, Div4 = 3,
            Div5 = 4, Div6 = 5, Div7 = 6, Div8 = 7
        ]
    ],

    C6 Control6 [
        LOLIE0 7 [],
        PLLS 6 [
            Fll = 0,
            PllcsOutput = 1
        ],
        CME0 5 [],
        VDIV (Mask(0b11111), 0) [
            Mul16 = 0, Mul17 = 1, Mul18 = 2, Mul19 = 3,
            Mul20 = 4, Mul21 = 5, Mul22 = 6, Mul23 = 7,
            Mul24 = 8, Mul25 = 9, Mul26 = 10, Mul27 = 11,
            Mul28 = 12, Mul29 = 13, Mul30 = 14, Mul31 = 15,
            Mul32 = 16, Mul33 = 17, Mul34 = 18, Mul35 = 19,
            Mul36 = 20, Mul37 = 21, Mul38 = 22, Mul39 = 23,
            Mul40 = 24, Mul41 = 25, Mul42 = 26, Mul43 = 27,
            Mul44 = 28, Mul45 = 29, Mul46 = 30, Mul47 = 31
        ]
    ],

    S Status [
        LOLS0 7 [],
        LOCK0 6 [],
        PLLST 5 [
            Fll = 0,
            PllcsOutput = 1
        ],
        IREFST 4 [
            External = 0,
            Internal = 1
        ],
        CLKST (Mask(0b11), 2) [
            Fll = 0,
            Internal = 1,
            External = 2,
            Pll = 3
        ],
        OSCINIT0 1 [],
        IRCST 0 [
            Slow = 0,
            Fast = 1
        ]
    ]
];
