//! Implementation of the MK66 System Integration Module

use core::mem;
use regs::sim::*;
use common::regs::FieldValue;

pub type Clock = (u32, FieldValue<u32>);

pub mod clocks {
    use sim::Clock;
    use regs::sim::*;
    pub const UART4: Clock = (1, SCGC1::UART4::True);
    pub const I2C2: Clock = (1, SCGC1::I2C2::True);
    pub const I2C3: Clock = (1, SCGC1::I2C3::True);

    pub const DAC1: Clock = (2, SCGC2::DAC1::True);
    pub const DAC0: Clock = (2, SCGC2::DAC0::True);
    pub const TPM2: Clock = (2, SCGC2::TPM2::True);
    pub const TPM1: Clock = (2, SCGC2::TPM1::True);
    pub const LPUART0: Clock = (2, SCGC2::LPUART0::True);
    pub const ENET: Clock = (2, SCGC2::ENET::True);

    pub const ADC1: Clock = (3, SCGC3::ADC1::True);
    pub const FTM3: Clock = (3, SCGC3::FTM3::True);
    pub const FTM2: Clock = (3, SCGC3::FTM2::True);
    pub const SDHC: Clock = (3, SCGC3::SDHC::True);
    pub const SPI2: Clock = (3, SCGC3::SPI2::True);
    pub const FLEXCAN1: Clock = (3, SCGC3::FLEXCAN1::True);
    pub const USBHSDCD: Clock = (3, SCGC3::USBHSDCD::True);
    pub const USBHSPHY: Clock = (3, SCGC3::USBHSPHY::True);
    pub const USBHS: Clock = (3, SCGC3::USBHS::True);
    pub const RNGA: Clock = (3, SCGC3::RNGA::True);

    pub const VREF: Clock = (4, SCGC4::VREF::True);
    pub const CMP: Clock = (4, SCGC4::CMP::True);
    pub const USBOTG: Clock = (4, SCGC4::USBOTG::True);
    pub const UART3: Clock = (4, SCGC4::UART3::True);
    pub const UART2: Clock = (4, SCGC4::UART2::True);
    pub const UART1: Clock = (4, SCGC4::UART1::True);
    pub const UART0: Clock = (4, SCGC4::UART0::True);
    pub const I2C1: Clock = (4, SCGC4::I2C1::True);
    pub const I2C0: Clock = (4, SCGC4::I2C0::True);
    pub const CMT: Clock = (4, SCGC4::CMT::True);
    pub const EWM: Clock = (4, SCGC4::EWM::True);

    pub const UART: [Clock; 5] = [UART0, UART1, UART2, UART3, UART4];

    pub const PORTE: Clock = (5, SCGC5::PORT::E);
    pub const PORTD: Clock = (5, SCGC5::PORT::D);
    pub const PORTC: Clock = (5, SCGC5::PORT::C);
    pub const PORTB: Clock = (5, SCGC5::PORT::B);
    pub const PORTA: Clock = (5, SCGC5::PORT::A);
    pub const PORTABCDE: Clock = (5, SCGC5::PORT::All);
    pub const TSI: Clock = (5, SCGC5::TSI::True);
    pub const LPTMR: Clock = (5, SCGC5::LPTMR::True);

    // DAC0,
    pub const RTC: Clock = (6, SCGC6::RTC::True);
    pub const ADC0: Clock = (6, SCGC6::ADC0::True);
    // FTM2,
    pub const FTM1: Clock = (6, SCGC6::FTM1::True);
    pub const FTM0: Clock = (6, SCGC6::FTM0::True);
    pub const PIT: Clock = (6, SCGC6::PIT::True);
    pub const PDB: Clock = (6, SCGC6::PDB::True);
    pub const USBDCD: Clock = (6, SCGC6::USBDCD::True);
    pub const CRC: Clock = (6, SCGC6::CRC::True);
    pub const I2S: Clock = (6, SCGC6::I2S::True);
    pub const SPI1: Clock = (6, SCGC6::SPI1::True);
    pub const SPI0: Clock = (6, SCGC6::SPI0::True);
    // RNGA,
    pub const FLEXCAN0: Clock = (6, SCGC6::FLEXCAN0::True);
    pub const DMAMUX: Clock = (6, SCGC6::DMAMUX::True);
    pub const FTF: Clock = (6, SCGC6::FTF::True);

    pub const SDRAMC: Clock = (7, SCGC7::SDRAMC::True);
    pub const MPU: Clock = (7, SCGC7::MPU::True);
    pub const DMA: Clock = (7, SCGC7::DMA::True);
    pub const FLEXBUS: Clock = (7, SCGC7::FLEXBUS::True);
}

pub fn enable_clock(clock: Clock) {
    let regs: &mut Registers = unsafe { mem::transmute(SIM) };

    match clock {
        (1, c) => regs.scgc1.modify(c),
        (2, c) => regs.scgc2.modify(c),
        (3, c) => regs.scgc3.modify(c),
        (4, c) => regs.scgc4.modify(c),
        (5, c) => regs.scgc5.modify(c),
        (6, c) => regs.scgc6.modify(c),
        (7, c) => regs.scgc7.modify(c),
        _ => ()
    };
}

pub fn set_dividers(core: u32, bus: u32, flash: u32) {
    let regs: &mut Registers = unsafe { mem::transmute(SIM) };

    regs.clkdiv1.modify(CLKDIV1::Core.val(core - 1) + 
                        CLKDIV1::Bus.val(bus - 1) + 
                        CLKDIV1::FlexBus.val(bus - 1) +
                        CLKDIV1::Flash.val(flash - 1));
}
