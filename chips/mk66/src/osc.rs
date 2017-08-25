use kernel::common::VolatileCell;
use core::mem;

#[allow(dead_code)]
pub struct Registers {
    cr: VolatileCell<u8>,
    div: VolatileCell<u8>
}

pub struct Osc {
    regs: *mut Registers
}

pub const OSC_BASE_ADDRESS: *mut Registers = 0x4006_5000 as *mut Registers;
pub static mut OSC: Osc = Osc::new();

impl Osc {
    const fn new() -> Osc {
        Osc { 
            regs: OSC_BASE_ADDRESS
        }
    }

    pub fn enable(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };

        // Crystal capacitance; must be even and <= 30 pF
        const CAPACITANCE: u8 = 10; // pF

        // The capacitance bits are all flipped in the control register
        let mut cr: u8 = (CAPACITANCE & 0b00010) << 2 |
                         (CAPACITANCE & 0b00100) << 0 |
                         (CAPACITANCE & 0b01000) >> 2 |
                         (CAPACITANCE & 0b10000) >> 4;

        // Enable the oscillator
        cr |= 1 << 7; 

        regs.cr.set(cr);
    }
}
