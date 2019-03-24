//! Implementation of the MK66 hardware watchdog timer

use core::mem;
use kernel::hil;

use kernel::common::registers::{register_bitfields, ReadWrite};

#[repr(C)]
pub struct Registers {
    pub stctrlh: ReadWrite<u16, StatusAndControlHigh::Register>,
    pub stctrll: ReadWrite<u16>,
    pub tovalh:  ReadWrite<u16>,
    pub tovall:  ReadWrite<u16>,
    pub winh:    ReadWrite<u16>,
    pub winl:    ReadWrite<u16>,
    pub refresh: ReadWrite<u16, Refresh::Register>,
    pub unlock:  ReadWrite<u16, Unlock::Register>,
    pub tmrouth: ReadWrite<u16>,
    pub tmroutl: ReadWrite<u16>,
    pub rstcnt:  ReadWrite<u16>,
    pub presc:   ReadWrite<u16>,
}

pub const WDOG: *mut Registers = 0x40052000 as *mut Registers;

register_bitfields![u16,
    StatusAndControlHigh [
        WAITEN 7,
        STOPEN 6,
        DBGEN 5,
        ALLOWUPDATE 4,
        WINEN 3,
        IRQSTEN 2,
        CLKSRC 1,
        WDOGEN 0
    ],
    Refresh [
        KEY OFFSET(0) NUMBITS(16) [
            Key1 = 0xA602,
            Key2 = 0xB480
        ]
    ],
    Unlock [
        KEY OFFSET(0) NUMBITS(16) [
            Key1 = 0xC520,
            Key2 = 0xD928
        ]
    ]
];

#[inline]
fn unlock() {
    let regs: &mut Registers = unsafe { mem::transmute(WDOG) };

    regs.unlock.write(Unlock::KEY::Key1);
    regs.unlock.write(Unlock::KEY::Key2);
    unsafe {
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
    }
}

#[allow(unused_variables)]
pub fn start(period: usize) {
    unimplemented!();
}

pub fn stop() {
    let regs: &mut Registers = unsafe { mem::transmute(WDOG) };

    // Must write the correct unlock sequence to the WDOG unlock register before reconfiguring
    // the module.
    unlock();

    // WDOG disabled in all power modes.
    // Allow future updates to the watchdog configuration.
    regs.stctrlh.modify(StatusAndControlHigh::ALLOWUPDATE::SET +
                        StatusAndControlHigh::WAITEN::CLEAR +
                        StatusAndControlHigh::STOPEN::CLEAR +
                        StatusAndControlHigh::DBGEN::CLEAR +
                        StatusAndControlHigh::WDOGEN::CLEAR);
}

pub fn tickle() {
    let regs: &mut Registers = unsafe { mem::transmute(WDOG) };
    regs.refresh.write(Refresh::KEY::Key1);
    regs.refresh.write(Refresh::KEY::Key2);
}

pub struct Wdog;
impl hil::watchdog::Watchdog for Wdog {
    fn start(&self, period: usize) {
        start(period);
    }

    fn stop(&self) {
        stop();
    }

    fn tickle(&self) {
        tickle();
    }
}
