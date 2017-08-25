use core::mem;
use regs::osc::*;

pub use self::CR::CAP::Value as OscCapacitance;

pub fn enable(cap: OscCapacitance) {
    let regs: &mut Registers = unsafe { mem::transmute(OSC) };

    // Set the capacitance.
    regs.cr.modify(CR::CAP.val(cap as u8));

    // Enable the oscillator.
    regs.cr.modify(CR::EREFSTEN::True);
}
