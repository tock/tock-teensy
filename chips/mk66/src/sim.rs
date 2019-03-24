//! Implementation of the MK66 System Integration Module

use core::mem;

use kernel::common::registers::{register_bitfields, FieldValue,ReadWrite, ReadOnly};

#[repr(C)]
pub struct SimRegisters {
    pub sopt2: ReadWrite<u32>,
    _reserved0: ReadWrite<u32>,
    pub sopt4: ReadWrite<u32>,
    pub sopt5: ReadWrite<u32>,
    _reserved1: ReadWrite<u32>,
    pub sopt7: ReadWrite<u32>,
    pub sopt8: ReadWrite<u32>,
    pub sopt9: ReadWrite<u32>,
    pub sdid: ReadOnly<u32>,
    pub scgc1: ReadWrite<u32, SystemClockGatingControl1::Register>,
    pub scgc2: ReadWrite<u32, SystemClockGatingControl2::Register>,
    pub scgc3: ReadWrite<u32, SystemClockGatingControl3::Register>,
    pub scgc4: ReadWrite<u32, SystemClockGatingControl4::Register>,
    pub scgc5: ReadWrite<u32, SystemClockGatingControl5::Register>,
    pub scgc6: ReadWrite<u32, SystemClockGatingControl6::Register>,
    pub scgc7: ReadWrite<u32, SystemClockGatingControl7::Register>,
    pub clkdiv1: ReadWrite<u32, ClockDivider1::Register>,
    pub clkdiv2: ReadWrite<u32>,
    pub fcfg1: ReadOnly<u32>,
    pub fcfg2: ReadOnly<u32>,
    pub uidh: ReadOnly<u32>,
    pub uidmh: ReadOnly<u32>,
    pub uidml: ReadOnly<u32>,
    pub uidl: ReadOnly<u32>,
    pub clkdiv3: ReadWrite<u32>,
    pub clkdiv4: ReadWrite<u32>,
}

pub const SIM: *mut SimRegisters = 0x40048004 as *mut SimRegisters;

register_bitfields![u32,
    SystemClockGatingControl1 [
        UART4 10,
        I2C3 7,
        I2C2 6
    ],
    SystemClockGatingControl2 [
        DAC1 13,
        DAC0 12,
        TPM2 10,
        TPM1 9,
        LPUART0 4,
        ENET 0
    ],
    SystemClockGatingControl3 [
        ADC1 27,
        FTM3 25,
        FTM2 24,
        SDHC 17,
        SPI2 12,
        FLEXCAN1 4,
        USBHSDCD 3,
        USBHSPHY 2,
        USBHS 1,
        RNGA 0
    ],
    SystemClockGatingControl4 [
        VREF 20,
        CMP 19,
        USBOTG 18,
        UART3 13,
        UART2 12,
        UART1 11,
        UART0 10,
        I2C1 7,
        I2C0 6,
        CMT 2,
        EWM 1
    ],
    SystemClockGatingControl5 [
        PORT OFFSET(9) NUMBITS(5) [
            All = 0b11111,
            A = 0b1,
            B = 0b10,
            C = 0b100,
            D = 0b1000,
            E = 0b10000
        ],
        TSI OFFSET(5) NUMBITS(1) [],
        LPTMR OFFSET(0) NUMBITS(1) []
    ],

    SystemClockGatingControl6 [
        DAC0 0,
        RTC 29,
        ADC0 27,
        FTM2 26,
        FTM1 25,
        FTM0 24,
        PIT 23,
        PDB 22,
        USBDCD 21,
        CRC 18,
        I2S 15,
        SPI1 13,
        SPI0 12,
        RNGA 9,
        FLEXCAN0 4,
        DMAMUX 1,
        FTF 0
    ],
    SystemClockGatingControl7 [
        SDRAMC 3,
        MPU 2,
        DMA 1,
        FLEXBUS 0
    ],
    ClockDivider1 [
        Core OFFSET(28) NUMBITS(4) [],
        Bus OFFSET(24) NUMBITS(4) [],
        FlexBus OFFSET(20) NUMBITS(4) [],
        Flash OFFSET(16) NUMBITS(4) []
    ]
];

pub type Clock1 = FieldValue<u32, SystemClockGatingControl1::Register>;
pub type Clock2 = FieldValue<u32, SystemClockGatingControl2::Register>;
pub type Clock3 = FieldValue<u32, SystemClockGatingControl3::Register>;
pub type Clock4 = FieldValue<u32, SystemClockGatingControl4::Register>;
pub type Clock5 = FieldValue<u32, SystemClockGatingControl5::Register>;
pub type Clock6 = FieldValue<u32, SystemClockGatingControl6::Register>;
pub type Clock7 = FieldValue<u32, SystemClockGatingControl7::Register>;

pub trait Clock {
    fn enable(self);
}

impl Clock for Clock1 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc1.modify(self);
    }
}

impl Clock for Clock2 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc2.modify(self);
    }
}

impl Clock for Clock3 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc3.modify(self);
    }
}

impl Clock for Clock4 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc4.modify(self);
    }
}

impl Clock for Clock5 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc5.modify(self);
    }
}

impl Clock for Clock6 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc6.modify(self);
    }
}

impl Clock for Clock7 {
    fn enable(self) {
        let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };
        regs.scgc7.modify(self);
    }
}

pub const UART4: Clock1 = self::SystemClockGatingControl1::UART4::SET;

pub mod clocks {
    use sim::{Clock1, Clock2, Clock3, Clock4, Clock5, Clock6, Clock7};
    use sim::{SystemClockGatingControl1,SystemClockGatingControl2};
    use sim::{SystemClockGatingControl3,SystemClockGatingControl4};
    use sim::{SystemClockGatingControl5,SystemClockGatingControl6};
    use sim::{SystemClockGatingControl7};

    pub const UART4: Clock1 = SystemClockGatingControl1::UART4::SET;
    pub const I2C2: Clock1 = SystemClockGatingControl1::I2C2::SET;
    pub const I2C3: Clock1 = SystemClockGatingControl1::I2C3::SET;

    pub const DAC1: Clock2 = SystemClockGatingControl2::DAC1::SET;
    pub const DAC0: Clock2 = SystemClockGatingControl2::DAC0::SET;
    pub const TPM2: Clock2 = SystemClockGatingControl2::TPM2::SET;
    pub const TPM1: Clock2 = SystemClockGatingControl2::TPM1::SET;
    pub const LPUART0: Clock2 = SystemClockGatingControl2::LPUART0::SET;
    pub const ENET: Clock2 = SystemClockGatingControl2::ENET::SET;

    pub const ADC1: Clock3 = SystemClockGatingControl3::ADC1::SET;
    pub const FTM3: Clock3 = SystemClockGatingControl3::FTM3::SET;
    pub const FTM2: Clock3 = SystemClockGatingControl3::FTM2::SET;
    pub const SDHC: Clock3 = SystemClockGatingControl3::SDHC::SET;
    pub const SPI2: Clock3 = SystemClockGatingControl3::SPI2::SET;
    pub const FLEXCAN1: Clock3 = SystemClockGatingControl3::FLEXCAN1::SET;
    pub const USBHSDCD: Clock3 = SystemClockGatingControl3::USBHSDCD::SET;
    pub const USBHSPHY: Clock3 = SystemClockGatingControl3::USBHSPHY::SET;
    pub const USBHS: Clock3 = SystemClockGatingControl3::USBHS::SET;
    pub const RNGA: Clock3 = SystemClockGatingControl3::RNGA::SET;

    pub const VREF: Clock4 = SystemClockGatingControl4::VREF::SET;
    pub const CMP: Clock4 = SystemClockGatingControl4::CMP::SET;
    pub const USBOTG: Clock4 = SystemClockGatingControl4::USBOTG::SET;
    pub const UART3: Clock4 = SystemClockGatingControl4::UART3::SET;
    pub const UART2: Clock4 = SystemClockGatingControl4::UART2::SET;
    pub const UART1: Clock4 = SystemClockGatingControl4::UART1::SET;
    pub const UART0: Clock4 = SystemClockGatingControl4::UART0::SET;
    pub const I2C1: Clock4 = SystemClockGatingControl4::I2C1::SET;
    pub const I2C0: Clock4 = SystemClockGatingControl4::I2C0::SET;
    pub const CMT: Clock4 = SystemClockGatingControl4::CMT::SET;
    pub const EWM: Clock4 = SystemClockGatingControl4::EWM::SET;

    pub const PORTE: Clock5 = SystemClockGatingControl5::PORT::E;
    pub const PORTD: Clock5 = SystemClockGatingControl5::PORT::D;
    pub const PORTC: Clock5 = SystemClockGatingControl5::PORT::C;
    pub const PORTB: Clock5 = SystemClockGatingControl5::PORT::B;
    pub const PORTA: Clock5 = SystemClockGatingControl5::PORT::A;
    pub const PORTABCDE: Clock5 = SystemClockGatingControl5::PORT::All;
    pub const TSI: Clock5 = SystemClockGatingControl5::TSI::SET;
    pub const LPTMR: Clock5 = SystemClockGatingControl5::LPTMR::SET;

    // DAC0,
    pub const RTC: Clock6 = SystemClockGatingControl6::RTC::SET;
    pub const ADC0: Clock6 = SystemClockGatingControl6::ADC0::SET;
    // FTM2,
    pub const FTM1: Clock6 = SystemClockGatingControl6::FTM1::SET;
    pub const FTM0: Clock6 = SystemClockGatingControl6::FTM0::SET;
    pub const PIT: Clock6 = SystemClockGatingControl6::PIT::SET;
    pub const PDB: Clock6 = SystemClockGatingControl6::PDB::SET;
    pub const USBDCD: Clock6 = SystemClockGatingControl6::USBDCD::SET;
    pub const CRC: Clock6 = SystemClockGatingControl6::CRC::SET;
    pub const I2S: Clock6 = SystemClockGatingControl6::I2S::SET;
    pub const SPI1: Clock6 = SystemClockGatingControl6::SPI1::SET;
    pub const SPI0: Clock6 = SystemClockGatingControl6::SPI0::SET;

    // RNGA,
    pub const FLEXCAN0: Clock6 = SystemClockGatingControl6::FLEXCAN0::SET;
    pub const DMAMUX: Clock6 = SystemClockGatingControl6::DMAMUX::SET;
    pub const FTF: Clock6 = SystemClockGatingControl6::FTF::SET;

    pub const SDRAMC: Clock7 = SystemClockGatingControl7::SDRAMC::SET;
    pub const MPU: Clock7 = SystemClockGatingControl7::MPU::SET;
    pub const DMA: Clock7 = SystemClockGatingControl7::DMA::SET;
    pub const FLEXBUS: Clock7 = SystemClockGatingControl7::FLEXBUS::SET;
}

pub fn set_dividers(core: u32, bus: u32, flash: u32) {
    let regs: &mut SimRegisters = unsafe { mem::transmute(SIM) };

    regs.clkdiv1.modify(ClockDivider1::Core.val(core - 1) +
                        ClockDivider1::Bus.val(bus - 1) +
                        ClockDivider1::FlexBus.val(bus - 1) +
                        ClockDivider1::Flash.val(flash - 1));
}
