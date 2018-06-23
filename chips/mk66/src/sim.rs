//! Implementation of the MK66 System Integration Module

use core::mem;
use regs::sim::*;
use kernel::common::regs::FieldValue;

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
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc1.modify(self);
    }
}

impl Clock for Clock2 {
    fn enable(self) {
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc2.modify(self);
    }
}

impl Clock for Clock3 {
    fn enable(self) {
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc3.modify(self);
    }
}

impl Clock for Clock4 {
    fn enable(self) {
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc4.modify(self);
    }
}

impl Clock for Clock5 {
    fn enable(self) {
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc5.modify(self);
    }
}

impl Clock for Clock6 {
    fn enable(self) {
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc6.modify(self);
    }
}

impl Clock for Clock7 {
    fn enable(self) {
        let regs: &mut Registers = unsafe { mem::transmute(SIM) };
        regs.scgc7.modify(self);
    }
}

pub mod clocks {
    use sim::{Clock1, Clock2, Clock3, Clock4, Clock5, Clock6, Clock7};
    use regs::sim::*;

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
    let regs: &mut Registers = unsafe { mem::transmute(SIM) };

    regs.clkdiv1.modify(ClockDivider1::Core.val(core - 1) +
                        ClockDivider1::Bus.val(bus - 1) +
                        ClockDivider1::FlexBus.val(bus - 1) +
                        ClockDivider1::Flash.val(flash - 1));
}
