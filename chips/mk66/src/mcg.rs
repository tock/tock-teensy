//! Implementation of the Multipurpose Clock Generator
//!

use common::regs::{Field, RW, RO};
use ::core::mem;

const MCG: *mut Registers = 0x4006_4000 as *mut Registers;

#[allow(dead_code)]
struct Registers {
    c1: RW<u8>,
    c2: RW<u8>,
    c3: RW<u8>,
    c4: RW<u8>,
    c5: RW<u8>,
    c6: RW<u8>,
    s: RO<u8>,
    _reserved0: RO<u8>,
    sc: RW<u8>,
    _reserved1: RO<u8>,
    atcvh: RW<u8>,
    atcvl: RW<u8>,
    c7: RW<u8>,
    c8: RW<u8>,
    c9: RW<u8>,
    _reserved2: RO<u8>,
    c11: RW<u8>,
    c12: RW<u8>,
    s2: RO<u8>,
    t3: RW<u8>
}

bitfields![u8,
    C1 [
        CLKS (0b11, 6) [
            LockedLoop = 0,
            Internal = 1,
            External = 2
        ],
        FRDIV (0b11, 4) [
            Low1_High32 = 0,
            Low2_High64 = 1,
            Low4_High128 = 2,
            Low8_High256 = 3,
            Low16_High512 = 4,
            Low32_High1024 = 5,
            Low64_High1280 = 6,
            Low128_High1536 = 7
        ],
        IREFS (1, 2) [
            External = 0,
            SlowInternal = 1
        ],
        IRCLKEN (1, 1) [
            Inactive = 0,
            Active = 1
        ],
        IREFSTEN (1, 0) [
            IrefDisabledInStop = 0,
            IrefEnabledInStop = 1
        ]
    ],

    C2 [
        LOCKRE0 (1, 7) [],
        FCFTRIM (1, 6) [],
        RANGE (0b11, 4) [
            Low = 0,
            High = 1,
            VeryHigh = 2
        ],
        HGO (1, 3) [
            LowPower = 0,
            HighGain = 1
        ],
        EREFS (1, 2) [
            External = 0,
            Oscillator = 1
        ],
        LP (1, 1) [],
        IRCS (1, 0) [
            SlowInternal = 0,
            FastInternal = 1
        ]
    ],

    C5 [
        PLLCLKEN (1, 6) [],
        PLLSTEN (1, 5) [],
        PRDIV (0b111, 0) [
            Div1 = 0, Div2 = 1, Div3 = 2, Div4 = 3,
            Div5 = 4, Div6 = 5, Div7 = 6, Div8 = 7
        ]
    ],

    C6 [
        LOLIE0 (1, 7) [],
        PLLS (1, 6) [
            Fll = 0,
            PllcsOutput = 1
        ],
        CME0 (1, 5) [],
        VDIV (0b11111, 0) [
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

    S [
        LOLS0 (1, 7) [],
        LOCK0 (1, 6) [],
        PLLST (1, 5) [
            Fll = 0,
            PllcsOutput = 1
        ],
        IREFST (1, 4) [
            External = 0,
            Internal = 1
        ],
        CLKST (0b11, 2) [
            Fll = 0,
            Internal = 1,
            External = 2,
            Pll = 3
        ],
        OSCINIT0 (1, 1) [],
        IRCST (1, 0) [
            Slow = 0,
            Fast = 1
        ]
    ]
];

pub use self::C1::CLKS::Value as OscSource;
pub use self::C1::FRDIV::Value as Frdiv;
pub use self::C2::RANGE::Value as OscRange;

pub enum State {
    Fei(Fei),
    Fee,
    Fbi,
    Fbe(Fbe),
    Pbe(Pbe),
    Pee,
    Blpi,
    Blpe,
    Stop,
}

#[derive(Copy,Clone)]
pub struct Fei;

#[derive(Copy,Clone)]
pub struct Fbe;

#[derive(Copy,Clone)]
pub struct Pbe;

pub fn state() -> State {
    let mcg: &mut Registers = unsafe { mem::transmute(MCG) };

    let clks: OscSource = match mcg.c1.read(C1::CLKS) {
        1 => OscSource::Internal,
        2 => OscSource::External,
        _ => OscSource::LockedLoop
    };

    let irefs = mcg.c1.is_set(C1::IREFS);
    let plls = mcg.c6.is_set(C6::PLLS);
    let lp = mcg.c2.is_set(C2::LP);

    match (clks, irefs, plls, lp) {
        (OscSource::LockedLoop, true, false, _) => State::Fei(Fei {}),
        (OscSource::LockedLoop, false, false, _) => State::Fee,
        (OscSource::Internal, true, false, false) => State::Fbi,
        (OscSource::External, false, false, false) => State::Fbe(Fbe {}),
        (OscSource::LockedLoop, false, true, _) => State::Pee,
        (OscSource::External, false, true, false) => State::Pbe(Pbe {}),
        (OscSource::Internal, true, false, true) => State::Blpi,
        (OscSource::External, false, _, true) => State::Blpe,
        _ => State::Fei(Fei {})
    }
}

// Source: https://branan.github.io/teensy/2017/01/28/uart.html
impl Fei {
    pub fn enable_xtal(self, range: OscRange) {
        let mcg: &mut Registers = unsafe { mem::transmute(MCG) };
        mcg.c2.modify(C2::RANGE.val(range as u8) +
                      C2::EREFS::True);

        while !mcg.s.is_set(S::OSCINIT0) {}
    }

    pub fn use_external(self, divide: Frdiv) -> Fbe {
        let mcg: &mut Registers = unsafe { mem::transmute(MCG) };

        mcg.c1.write(C1::CLKS::External +
                     C1::FRDIV.val(divide as u8) +
                     C1::IREFS::False);

        while !mcg.s.matches(S::IREFST::False + 
                             S::CLKST::External) {}

        Fbe { }
    }
}

impl Fbe {
    pub fn enable_pll(self, multiplier: u8, divider: u8) -> Pbe {
        let mcg: &mut Registers = unsafe { mem::transmute(MCG) };

        if multiplier < 16 || multiplier > 47 {
            panic!("Invalid PLL VCO divide factor: {}", multiplier);
        }
        if divider < 1 || divider > 8 {
            panic!("Invalid PLL reference divide factor: {}", divider);
        }

        mcg.c5.modify(C5::PRDIV.val(divider - 1));

        mcg.c6.modify(C6::VDIV.val(multiplier - 16) +
                      C6::PLLS::True);

        // Wait for PLL to be selected and stable PLL lock
        while !mcg.s.matches(S::PLLST::True + S::LOCK0::True) {}

        Pbe { }
    }
}

impl Pbe {
    pub fn use_pll(self) {
        let mcg: &mut Registers = unsafe { mem::transmute(MCG) };

        mcg.c1.modify(C1::CLKS::LockedLoop);

        while !mcg.s.matches(S::CLKST::Pll) {}
    }
}
