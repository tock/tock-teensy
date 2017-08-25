#[allow(unused)]

use mk66::regs::{RW, Field};

struct Registers {
    c1: RW<u8>,
    c2: RW<u8>
}

// Some made up register fields, defined manually.
#[allow(non_snake_case)]
pub mod C1 {
    use mk66::regs::FieldMask;

    #[allow(non_upper_case_globals)]
    pub const CLKS: FieldMask<u8> = FieldMask::new(0b11, 6);

    #[allow(non_upper_case_globals)]
    pub const PRDIV: FieldMask<u8> = FieldMask::new(0b11, 4);

    #[allow(non_snake_case)]
    pub mod PRDIV {
        use mk66::regs::FieldValue;

        #[allow(non_upper_case_globals)]
        pub const Div32: FieldValue<u8> = FieldValue::<u8>::new(0b11, 4, 2);
    }
}

// Some made up register fields (expands to the same thing as the C1 module above).
bitfields! { u8,
    C2 [
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

        asm!{"nop"::::"volatile"};
        asm!{"nop"::::"volatile"};
        asm!{"nop"::::"volatile"};

        regs.c2.set(1 << 5);

        regs.c2.modify(C2::CLKS.val(3) +
                       C2::PRDIV.val(1));

        regs.c2.write(C2::CLKS.val(1) +
                      C2::PRDIV::Div32);

        while regs.c2.read(C2::CLKS) != 1 {}
    }
}
