#![crate_name = "mk66"]
#![crate_type = "rlib"]
#![feature(asm,core_intrinsics,concat_idents,const_fn)]
#![no_std]

#[allow(unused_extern_crates)]
extern crate cortexm4;

#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;

extern crate sha2;
extern crate twofish;
extern crate block_cipher_trait;

pub mod chip;
pub mod nvic;
pub mod wdog;
pub mod gpio;
pub mod sim;
pub mod mcg;
pub mod osc;
pub mod uart;
pub mod clock;
pub mod pit;
pub mod spi;
pub mod mpu;

#[allow(while_true)]
pub mod rnga;

use cortexm4::{generic_isr, svc_handler, hard_fault_handler, systick_handler};

// TODO: Should this be moved to the cortexm crate?
unsafe extern "C" fn unhandled_interrupt() {
    let mut interrupt_number: u32;

    // IPSR[8:0] holds the currently active interrupt
    asm!(
        "mrs    r0, ipsr                    "
        : "={r0}"(interrupt_number)
        :
        : "r0"
        :
        );

    interrupt_number = interrupt_number & 0x1ff;

    panic!("Unhandled Interrupt. ISR {} is active.", interrupt_number);
}

extern "C" {
    // _estack is not really a function, but it makes the types work.
    // You should never actually invoke it!!
    fn _estack();

    // Defined by platform
    fn reset_handler();

    static mut _szero: u32;
    static mut _ezero: u32;
    static mut _etext: u32;
    static mut _srelocate: u32;
    static mut _erelocate: u32;
}

// Cortex-M core interrupt vectors
#[link_section=".vectors"]
// no_mangle ensures that the symbol is kept until the final binary
#[no_mangle]
pub static BASE_VECTORS: [unsafe extern fn(); 16] = [
    _estack, reset_handler,
    unhandled_interrupt, // NMI
    hard_fault_handler, // Hard Fault
    unhandled_interrupt, // MemManage
    unhandled_interrupt, // BusFault
    unhandled_interrupt, // UsageFault
    unhandled_interrupt, unhandled_interrupt, unhandled_interrupt,
    unhandled_interrupt,
    svc_handler, // SVC
    unhandled_interrupt, // DebugMon
    unhandled_interrupt,
    unhandled_interrupt, // PendSV
    systick_handler // SysTick
];

#[link_section=".vectors"]
// no_mangle ensures that the symbol is kept until the final binary
#[no_mangle]
pub static IRQS: [unsafe extern "C" fn(); 100] = [generic_isr; 100];

pub unsafe fn init() {
    // TODO: Enable the FPU (SCB_CPACR) and LMEM_PCCCR.

    // Relocate data segment.
    // Assumes data starts right after text segment as specified by the linker
    // file.
    let mut pdest = &mut _srelocate as *mut u32;
    let pend = &mut _erelocate as *mut u32;
    let mut psrc = &_etext as *const u32;

    if psrc != pdest {
        while (pdest as *const u32) < pend {
            *pdest = *psrc;
            pdest = pdest.offset(1);
            psrc = psrc.offset(1);
        }
    }

    // Clear the zero segment (BSS)
    let pzero = &_ezero as *const u32;
    pdest = &mut _szero as *mut u32;

    while (pdest as *const u32) < pzero {
        *pdest = 0;
        pdest = pdest.offset(1);
    }
}

