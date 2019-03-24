use core::mem;

use kernel::common::registers::{register_bitfields, ReadWrite};
pub const OSC: *mut OscRegisters = 0x4006_5000 as *mut OscRegisters;

pub use self::Control::CAP::Value as OscCapacitance;

#[repr(C)]
pub struct OscRegisters {
    pub cr: ReadWrite<u8, Control::Register>,
    pub div: ReadWrite<u8, Divider::Register>
}

register_bitfields![u8,
    Control [
        ERCLKEN OFFSET(7) NUMBITS(1) [],
        EREFSTEN OFFSET(5) NUMBITS(1) [],
        CAP OFFSET(0) NUMBITS(4) [
            Load_0pF = 0b0000,
            Load_2pF = 0b1000,
            Load_4pF = 0b0100,
            Load_6pF = 0b1100,
            Load_8pF = 0b0010,
            Load_10pF = 0b1010,
            Load_12pF = 0b0110,
            Load_14pF = 0b1110,
            Load_16pF = 0b0001,
            Load_18pF = 0b1001,
            Load_20pF = 0b0101,
            Load_22pF = 0b1101,
            Load_24pF = 0b0011,
            Load_26pF = 0b1011,
            Load_28pF = 0b0111,
            Load_30pF = 0b1111
        ]
    ],
    Divider [
        ERPS OFFSET(6) NUMBITS(2) [
            Div1 = 0,
            Div2 = 1,
            Div4 = 2,
            Div8 = 3
        ]
    ]
];

pub fn enable(osc: ::mcg::Xtal) {
    let regs: &mut OscRegisters = unsafe { mem::transmute(OSC) };

    // Set the capacitance.
    regs.cr.modify(Control::CAP.val(osc.load as u8));

    // Enable the oscillator.
    regs.cr.modify(Control::EREFSTEN::SET);
}
