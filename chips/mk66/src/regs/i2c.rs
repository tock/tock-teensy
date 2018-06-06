use common::regs::{ReadWrite};
#[repr(C, packed)]
pub struct Registers {
    pub  a1: ReadWrite<u8, I2CAddress>,
    pub  f: ReadWrite<u8, MultiplierFactor>,
    pub  c1: ReadWrite<u8, ControlRegister1>,
    pub  s: ReadWrite<u8, StatusRegister>,
    pub  d: ReadWrite<u8>,
    pub  c2: ReadWrite<u8, ControlRegister2>,
    pub  flt: ReadWrite<u8, FltRegister>,
    pub  ra: ReadWrite<u8, RangeAddress>,
    pub  smb: ReadWrite<u8, SMBusCSR>,
    pub  a2: ReadWrite<u8, I2CAddress>,
    pub  slth: ReadWrite<u8>,
    pub  sltl: ReadWrite<u8>
}

pub const I2C_BASE_ADDRS: [*mut Registers; 4] = [0x40066000 as *mut Registers,
                                                 0x40067000 as *mut Registers,
                                                 0x400E6000 as *mut Registers,
                                                 0x400E7000 as *mut Registers];

bitfields! {u8,
            A1 I2CAddress [
                ADR (1, Mask(0b1111111)),
                res 0
            ],
            F MultiplierFactor [
                Factor (6, Mask(0b11)),
                //f1 = 00,
                //                    f2 = 01,
                  //  f4 = 02,
                   // res = 03
                Rate (0, Mask(0b111111))
            ],
            // These names are kinda awkward, but they match the docs, so ...
            C1 ControlRegister1 [
                IICEN 7 [],
                IICIE 6[],
                MST (5, Mask(0b1)) [
                    Slave=0,
                    Master=1
                ],
                TX 4 [],
                TXAK 3 [],
                RSTAR 2[],
                WUEN 1 [],
                DMAEN 0 []
            ],
            Status StatusRegister [
                TCF 7,
                IAAS 6,
                BUSY 5,
                ARBL 4,
                RAM 3,
                SRW 2,
                IICIF 1,
                RXAK 0
            ],
            C2 ControlRegister2 [
                GCAEN 7,
                ADEXT 6,
                HDRS 5,
                SBRC 4,
                RMEN 3,
                AD10 (0, Mask(0b111))
            ],
            FLT FltRegister [
                SHEN 7,
                STOPF 6,
                SSIE 5,
                STARTF 4,
                FLT (0, Mask(0b1111))
            ],
            RA RangeAddress [
                RAD(1, Mask(0b1111111)),
                res 0
            ],
            SMB SMBusCSR [
                FACK 7,
                ALERTEN 6,
                SIICAEN 5,
                TCKSEL 4,
                SLTF 3,
                SHTF1 2,
                SHTF2 1,
                SHTF2IE 0
            ]
}
