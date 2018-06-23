//! Implementation of the Freescale MK66 interrupt controller

use core::intrinsics;
use kernel::common::cells::VolatileCell;

// TODO: This register format is common to all Cortex-M cores, so I think it
// should be moved to the cortexm crate
#[repr(C, packed)]
struct Nvic {
    iser: [VolatileCell<u32>; 7],
    _reserved0: [u32; 25],
    icer: [VolatileCell<u32>; 7],
    _reserved1: [u32; 25],
    ispr: [VolatileCell<u32>; 7],
    _reserved2: [u32; 25],
    icpr: [VolatileCell<u32>; 7],
}


pub const DMA0: u32 = 0;
pub const DMA1: u32 = 1;
pub const DMA2: u32 = 2;
pub const DMA3: u32 = 3;
pub const DMA4: u32 = 4;
pub const DMA5: u32 = 5;
pub const DMA6: u32 = 6;
pub const DMA7: u32 = 7;
pub const DMA8: u32 = 8;
pub const DMA9: u32 = 9;
pub const DMA10: u32 = 10;
pub const DMA11: u32 = 11;
pub const DMA12: u32 = 12;
pub const DMA13: u32 = 13;
pub const DMA14: u32 = 14;
pub const DMA15: u32 = 15;
pub const DMAERR: u32 = 16;
pub const MCM: u32 = 17;
pub const FLASHCC: u32 = 18;
pub const FLASHRC: u32 = 19;
pub const MODECTRL: u32 = 20;
pub const LLWU: u32 = 21;
pub const WDOG: u32 = 22;
pub const RNG: u32 = 23;
pub const I2C0: u32 = 24;
pub const I2C1: u32 = 25;
pub const SPI0: u32 = 26;
pub const SPI1: u32 = 27;
pub const I2S0_TX: u32 = 28;
pub const I2S0_RX: u32 = 29;
pub const _RESERVED0: u32 = 30;
pub const UART0: u32 = 31;
pub const UART0_ERR: u32 = 32;
pub const UART1: u32 = 33;
pub const UART1_ERR: u32 = 34;
pub const UART2: u32 = 35;
pub const UART2_ERR: u32 = 36;
pub const UART3: u32 = 37;
pub const UART3_ERR: u32 = 38;
pub const ADC0: u32 = 39;
pub const CMP0: u32 = 40;
pub const CMP1: u32 = 41;
pub const FTM0: u32 = 42;
pub const FTM1: u32 = 43;
pub const FTM2: u32 = 44;
pub const CMT: u32 = 45;
pub const RTC_ALARM: u32 = 46;
pub const RTC_SECONDS: u32 = 47;
pub const PIT0: u32 = 48;
pub const PIT1: u32 = 49;
pub const PIT2: u32 = 50;
pub const PIT3: u32 = 51;
pub const PDB: u32 = 52;
pub const USBFS_OTG: u32 = 53;
pub const USBFS_CHARGE: u32 = 54;
pub const _RESERVED1: u32 = 55;
pub const DAC0: u32 = 56;
pub const MCG: u32 = 57;
pub const LOWPOWERTIER: u32 = 58;
pub const PCMA: u32 = 59;
pub const PCMB: u32 = 60;
pub const PCMC: u32 = 61;
pub const PCMD: u32 = 62;
pub const PCME: u32 = 63;
pub const SOFTWARE: u32 = 64;
pub const SPI2: u32 = 65;
pub const UART4: u32 = 66;
pub const UART4_ERR: u32 = 67;
pub const _RESERVED2: u32 = 68;
pub const _RESERVED3: u32 = 69;
pub const CMP2: u32 = 70;
pub const FTM3: u32 = 71;
pub const DAC1: u32 = 72;
pub const ADC1: u32 = 73;
pub const I2C2: u32 = 74;
pub const CAN0_MSGBUF: u32 = 75;
pub const CAN0_BUSOFF: u32 = 76;
pub const CAN0_ERR: u32 = 77;
pub const CAN0_TX: u32 = 78;
pub const CAN0_RX: u32 = 79;
pub const CAN0_WKUP: u32 = 80;
pub const SDHC: u32 = 81;
pub const EMAC_TIMER: u32 = 82;
pub const EMAC_TX: u32 = 83;
pub const EMAC_RX: u32 = 84;
pub const EMAC_ERR: u32 = 85;
pub const LPUART0: u32 = 86;
pub const TSI0: u32 = 87;
pub const TPM1: u32 = 88;
pub const TPM2: u32 = 89;
pub const USBHS: u32 = 90;
pub const I2C3: u32 = 91;
pub const CMP3: u32 = 92;
pub const USBHS_OTG: u32 = 93;
pub const CAN1_MSBBUF: u32 = 94;
pub const CAN1_BUSOFF: u32 = 95;
pub const CAN1_ERR: u32 = 96;
pub const CAN1_TX: u32 = 97;
pub const CAN1_RX: u32 = 98;
pub const CAN1_WKUP: u32 = 99;


#[repr(C)]
#[derive(Copy,Clone)]
#[allow(non_camel_case_types)]
pub enum NvicIdx {
    DMA0,
    DMA1,
    DMA2,
    DMA3,
    DMA4,
    DMA5,
    DMA6,
    DMA7,
    DMA8,
    DMA9,
    DMA10,
    DMA11,
    DMA12,
    DMA13,
    DMA14,
    DMA15,
    DMAERR,
    MCM,
    FLASHCC,
    FLASHRC,
    MODECTRL,
    LLWU,
    WDOG,
    RNG,
    I2C0,
    I2C1,
    SPI0,
    SPI1,
    I2S0_TX,
    I2S0_RX,
    _RESERVED0,
    UART0,
    UART0_ERR,
    UART1,
    UART1_ERR,
    UART2,
    UART2_ERR,
    UART3,
    UART3_ERR,
    ADC0,
    CMP0,
    CMP1,
    FTM0,
    FTM1,
    FTM2,
    CMT,
    RTC_ALARM,
    RTC_SECONDS,
    PIT0,
    PIT1,
    PIT2,
    PIT3,
    PDB,
    USBFS_OTG,
    USBFS_CHARGE,
    _RESERVED1,
    DAC0,
    MCG,
    LOWPOWERTIMER,
    PCMA,
    PCMB,
    PCMC,
    PCMD,
    PCME,
    SOFTWARE,
    SPI2,
    UART4,
    UART4_ERR,
    _RESERVED2,
    _RESERVED3,
    CMP2,
    FTM3,
    DAC1,
    ADC1,
    I2C2,
    CAN0_MSGBUF,
    CAN0_BUSOFF,
    CAN0_ERR,
    CAN0_TX,
    CAN0_RX,
    CAN0_WKUP,
    SDHC,
    EMAC_TIMER,
    EMAC_TX,
    EMAC_RX,
    EMAC_ERR,
    LPUART0,
    TSI0,
    TPM1,
    TPM2,
    USBHS,
    I2C3,
    CMP3,
    USBHS_OTG,
    CAN1_MSBBUF,
    CAN1_BUSOFF,
    CAN1_ERR,
    CAN1_TX,
    CAN1_RX,
    CAN1_WKUP,
}

impl ::core::default::Default for NvicIdx {
    fn default() -> NvicIdx {
        NvicIdx::DMA0
    }
}

// Defined by ARM Cortex-M bus architecture
// TODO: since these functions/constants are common to all ARM Cortex-M cores, I
// think they should be moved to the cortexm crate.
const BASE_ADDRESS: usize = 0xe000e100;

pub unsafe fn enable(signal: NvicIdx) {
    let nvic: &mut Nvic = intrinsics::transmute(BASE_ADDRESS);
    let interrupt = signal as usize;

    nvic.iser[interrupt / 32].set(1 << (interrupt & 31));
}

pub unsafe fn disable(signal: NvicIdx) {
    let nvic: &mut Nvic = intrinsics::transmute(BASE_ADDRESS);
    let interrupt = signal as usize;

    nvic.icer[interrupt / 32].set(1 << (interrupt & 31));
}

pub unsafe fn clear_pending(signal: NvicIdx) {
    let nvic: &mut Nvic = intrinsics::transmute(BASE_ADDRESS);
    let interrupt = signal as usize;

    nvic.icpr[interrupt / 32].set(1 << (interrupt & 31));
}
