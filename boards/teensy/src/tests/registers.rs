#[allow(unused)]

use kernel::common::regs::ReadWrite;

struct Registers {
    c1: ReadWrite<u8, Control::Register>,
}

// Some made up register fields.
register_bitfields! { u8,
    Control [
        CLKS  OFFSET(6) NUMBITS(3) [],
        PRDIV OFFSET(4) NUMBITS(3) [
            Div32 = 2
        ]
    ]
}

const BASE: *mut Registers = 0x2000_0000 as *mut Registers;

#[inline(never)]
pub fn register_test() {
    unsafe {
        let regs: &mut Registers = ::core::mem::transmute(BASE);

        regs.c1.set(1 << 5);

        regs.c1.modify(Control::CLKS.val(3) +
                       Control::PRDIV.val(1));

        regs.c1.write(Control::CLKS.val(1) +
                      Control::PRDIV::Div32);

        while regs.c1.read(Control::CLKS) != 1 {}
    }
}
