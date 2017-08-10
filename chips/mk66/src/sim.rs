//! Implementation of the MK66 System Integration Module

use kernel::common::VolatileCell;

#[repr(C, packed)]
struct Registers {
    sopt2: VolatileCell<u32>,
    _reserved0: VolatileCell<u32>,
    sopt4: VolatileCell<u32>,
    sopt5: VolatileCell<u32>,
    _reserved1: VolatileCell<u32>,
    sopt7: VolatileCell<u32>,
    sopt8: VolatileCell<u32>,
    sopt9: VolatileCell<u32>,
    sdid: VolatileCell<u32>,
    scgc1: VolatileCell<u32>,
    scgc2: VolatileCell<u32>,
    scgc3: VolatileCell<u32>,
    scgc4: VolatileCell<u32>,
    scgc5: VolatileCell<u32>,
    scgc6: VolatileCell<u32>,
    scgc7: VolatileCell<u32>,
    clkdiv1: VolatileCell<u32>,
    clkdiv2: VolatileCell<u32>,
    fcfg1: VolatileCell<u32>,
    fcfg2: VolatileCell<u32>,
    uidh: VolatileCell<u32>,
    uidmh: VolatileCell<u32>,
    uidml: VolatileCell<u32>,
    uidl: VolatileCell<u32>,
    clkdiv3: VolatileCell<u32>,
    clkdiv4: VolatileCell<u32>,
}

const SIM: *mut Registers = 0x40048004 as *mut Registers;

pub enum Clock {
    Group1(Group1Clock),
    Group2(Group2Clock),
    Group3(Group3Clock),
    Group4(Group4Clock),
    Group5(Group5Clock),
    Group6(Group6Clock),
    Group7(Group7Clock),
}

pub enum Group1Clock {
    I2C2  = 1 << 6,
    I2C3  = 1 << 7,
    UART4 = 1 << 10,
}

pub enum Group2Clock {
    ENET = 1,
    LPUART0 = 1 << 4,
    TPM1 = 1 << 9,
    TPM2 = 1 << 10,
    DAC0 = 1 << 12,
    DAC1 = 1 << 13,
}

pub enum Group3Clock {
    RNGA = 1,
    USBHS = 1 << 1,
    USBHSPHY = 1 << 2,
    USBHSDCD = 1 << 3,
    FLEXCAN1 = 1 << 4,
    SPI2 = 1 << 12,
    SDHC = 1 << 17,
    FTM2 = 1 << 24,
    FTM3 = 1 << 25,
    ADC1 = 1 << 27,
}

pub enum Group4Clock {
    EWM = 1 << 1,
    CMT = 1 << 2,
    I2C0 = 1 << 6,
    I2C1 = 1 << 7,
    UART0 = 1 << 10,
    UART1 = 1 << 11,
    UART2 = 1 << 12,
    UART3 = 1 << 13,
    USBOTG = 1 << 18,
    CMP = 1 << 19,
    VREF = 1 << 20,
}

pub enum Group5Clock {
    LPTMR = 1,
    TSI = 1 << 5,
    PORTA = 1 << 9,
    PORTB = 1 << 10,
    PORTC = 1 << 11,
    PORTD = 1 << 12,
    PORTE = 1 << 13,

    // All gpio ports
    PORTABCDE = 0b11111 << 9,
}

pub enum Group6Clock {
    FTF = 1,
    DMAMUX = 1 << 1,
    FLEXCAN0 = 1 << 4,
    RNGA = 1 << 9,
    SPI0 = 1 << 12,
    SPI1 = 1 << 13,
    I2S = 1 << 15,
    CRC = 1 << 18,
    USBDCD = 1 << 21,
    PDB = 1 << 22,
    PIT = 1 << 23,
    FTM0 = 1 << 24,
    FTM1 = 1 << 25,
    FTM2 = 1 << 26,
    ADC0 = 1 << 27,
    RTC = 1 << 29,
    DAC0 = 1 << 31,
}

pub enum Group7Clock {
    FLEXBUS = 1,
    DMA = 1 << 1,
    MPU = 1 << 2,
    SDRAMC = 1 << 3,
}

pub mod clocks {
    use sim::Clock;
    use sim::Clock::*;
    use sim::Group1Clock;
    use sim::Group2Clock;
    use sim::Group3Clock;
    use sim::Group4Clock;
    use sim::Group5Clock;
    use sim::Group6Clock;
    use sim::Group7Clock;

    pub const I2C2: Clock = Group1(Group1Clock::I2C2);
    pub const I2C3: Clock = Group1(Group1Clock::I2C3);
    pub const UART4: Clock = Group1(Group1Clock::UART4);

    pub const ENET: Clock = Group2(Group2Clock::ENET);
    pub const LPUART0: Clock = Group2(Group2Clock::LPUART0);
    pub const TPM1: Clock = Group2(Group2Clock::TPM1);
    pub const TPM2: Clock = Group2(Group2Clock::TPM2);
    pub const DAC0: Clock = Group2(Group2Clock::DAC0);
    pub const DAC1: Clock = Group2(Group2Clock::DAC1);

    pub const RNGA: Clock = Group3(Group3Clock::RNGA);
    pub const USBHS: Clock = Group3(Group3Clock::USBHS);
    pub const USBHSPHY: Clock = Group3(Group3Clock::USBHSPHY);
    pub const USBHSDCD: Clock = Group3(Group3Clock::USBHSDCD);
    pub const FLEXCAN1: Clock = Group3(Group3Clock::FLEXCAN1);
    pub const SPI2: Clock = Group3(Group3Clock::SPI2);
    pub const SDHC: Clock = Group3(Group3Clock::SDHC);
    pub const FTM2: Clock = Group3(Group3Clock::FTM2);
    pub const FTM3: Clock = Group3(Group3Clock::FTM3);
    pub const ADC1: Clock = Group3(Group3Clock::ADC1);

    pub const EWM: Clock = Group4(Group4Clock::EWM);
    pub const CMT: Clock = Group4(Group4Clock::CMT);
    pub const I2C0: Clock = Group4(Group4Clock::I2C0);
    pub const I2C1: Clock = Group4(Group4Clock::I2C1);
    pub const UART0: Clock = Group4(Group4Clock::UART0);
    pub const UART1: Clock = Group4(Group4Clock::UART1);
    pub const UART2: Clock = Group4(Group4Clock::UART2);
    pub const UART3: Clock = Group4(Group4Clock::UART3);
    pub const USBOTG: Clock = Group4(Group4Clock::USBOTG);
    pub const CMP: Clock = Group4(Group4Clock::CMP);
    pub const VREF: Clock = Group4(Group4Clock::VREF);

    pub const LPTMR: Clock = Group5(Group5Clock::LPTMR);
    pub const TSI: Clock = Group5(Group5Clock::TSI);
    pub const PORTA: Clock = Group5(Group5Clock::PORTA);
    pub const PORTB: Clock = Group5(Group5Clock::PORTB);
    pub const PORTC: Clock = Group5(Group5Clock::PORTC);
    pub const PORTD: Clock = Group5(Group5Clock::PORTD);
    pub const PORTE: Clock = Group5(Group5Clock::PORTE);
    pub const PORTABCDE: Clock = Group5(Group5Clock::PORTABCDE);

    pub const FTF: Clock = Group6(Group6Clock::FTF);
    pub const DMAMUX: Clock = Group6(Group6Clock::DMAMUX);
    pub const FLEXCAN0: Clock = Group6(Group6Clock::FLEXCAN0);
    // pub const RNGA: Clock = Group6(Group6Clock::RNGA);
    pub const SPI0: Clock = Group6(Group6Clock::SPI0);
    pub const SPI1: Clock = Group6(Group6Clock::SPI1);
    pub const I2S: Clock = Group6(Group6Clock::I2S);
    pub const CRC: Clock = Group6(Group6Clock::CRC);
    pub const USBDCD: Clock = Group6(Group6Clock::USBDCD);
    pub const PDB: Clock = Group6(Group6Clock::PDB);
    pub const PIT: Clock = Group6(Group6Clock::PIT);
    pub const FTM0: Clock = Group6(Group6Clock::FTM0);
    pub const FTM1: Clock = Group6(Group6Clock::FTM1);
    // pub const FTM2: Clock = Group6(Group6Clock::FTM2);
    pub const ADC0: Clock = Group6(Group6Clock::ADC0);
    pub const RTC: Clock = Group6(Group6Clock::RTC);
    // pub const DAC0: Clock = Group6(Group6Clock::DAC0);

    pub const FLEXBUS: Clock = Group7(Group7Clock::FLEXBUS);
    pub const DMA: Clock = Group7(Group7Clock::DMA);
    pub const MPU: Clock = Group7(Group7Clock::MPU);
    pub const SDRAMC: Clock = Group7(Group7Clock::SDRAMC);
}


pub unsafe fn enable_clock(clock: Clock) {
    match clock {
        Clock::Group1(clock_mask) => {
            let scgc: u32 = (*SIM).scgc1.get();
            (*SIM).scgc1.set(scgc | (clock_mask as u32));
        },
        Clock::Group2(clock_mask) => {
            let scgc: u32 = (*SIM).scgc2.get();
            (*SIM).scgc2.set(scgc | (clock_mask as u32));
        },
        Clock::Group3(clock_mask) => {
            let scgc: u32 = (*SIM).scgc3.get();
            (*SIM).scgc3.set(scgc | (clock_mask as u32));
        },
        Clock::Group4(clock_mask) => {
            let scgc: u32 = (*SIM).scgc4.get();
            (*SIM).scgc4.set(scgc | (clock_mask as u32));
        },
        Clock::Group5(clock_mask) => {
            let scgc: u32 = (*SIM).scgc5.get();
            (*SIM).scgc5.set(scgc | (clock_mask as u32));
        },
        Clock::Group6(clock_mask) => {
            let scgc: u32 = (*SIM).scgc6.get();
            (*SIM).scgc6.set(scgc | (clock_mask as u32));
        },
        Clock::Group7(clock_mask) => {
            let scgc: u32 = (*SIM).scgc7.get();
            (*SIM).scgc7.set(scgc | (clock_mask as u32));
        },
    };
}
