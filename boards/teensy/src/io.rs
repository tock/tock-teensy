use core::fmt::*;

#[cfg(not(test))]
#[no_mangle]
#[allow(unused_variables)]
#[lang="panic_fmt"]
pub unsafe extern "C" fn panic_fmt(args: Arguments, file: &'static str, line: u32) -> ! {
    loop {
        asm!("nop");
    }
}
