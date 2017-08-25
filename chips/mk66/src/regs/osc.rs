use common::regs::RW;

pub const OSC: *mut Registers = 0x4006_5000 as *mut Registers;

#[allow(dead_code)]
#[repr(C, packed)]
pub struct Registers {
    pub cr: RW<u8>,
    pub div: RW<u8>
}

bitfields![u8,
    CR [
        ERCLKEN (1, 7) [],
        EREFSTEN (1, 5) [],
        CAP (0b1111, 0) [
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
    DIV [
        ERPS (0b11, 6) [
            Div1 = 0,
            Div2 = 1,
            Div4 = 2,
            Div8 = 3
        ]
    ]
];