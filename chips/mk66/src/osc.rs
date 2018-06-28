use core::mem;
use regs::osc::*;

pub use self::Control::CAP::Value as OscCapacitance;

pub fn enable(osc: ::mcg::Xtal) {
    let regs: &mut Registers = unsafe { mem::transmute(OSC) };

    // Set the capacitance.
    regs.cr.modify(Control::CAP.val(osc.load as u8));

    // Enable the oscillator.
    regs.cr.modify(Control::EREFSTEN::SET);
}
