#[allow(unused)]

use common::regs::RW;

struct Registers {
    c1: RW<u8>,
}

// Some made up register fields.
bitfields! { u8,
    C1 [
        CLKS  (0b11, 6) [],
        PRDIV (0b11, 4) [
            Div32 = 2
        ]
    ]
}

static mut BASE: *mut Registers = 0x2000_0000 as *mut Registers;

#[inline(never)]
pub fn register_test() {
    unsafe {
        let regs: &mut Registers = ::core::mem::transmute(BASE);

        regs.c1.set(1 << 5);

        regs.c1.modify(C1::CLKS.val(3) +
                       C1::PRDIV.val(1));

        regs.c1.write(C1::CLKS.val(1) +
                      C1::PRDIV::Div32);

        while regs.c1.read(C1::CLKS) != 1 {}
    }
}
