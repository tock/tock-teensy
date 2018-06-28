//! Implementation of the MK66 hardware watchdog timer

use core::mem;
use kernel::hil;
use regs::wdog::*;

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
