#![no_std]
#![no_main]
#![feature(asm,const_fn,drop_types_in_const,lang_items,compiler_builtins_lib)]

extern crate capsules;
extern crate compiler_builtins;
extern crate kernel;

#[allow(dead_code)]
extern crate mk66;

#[macro_use]
pub mod io;

#[allow(unused)]
struct Teensy {
    ipc: kernel::ipc::IPC,
}

impl kernel::Platform for Teensy {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
        where F: FnOnce(Option<&kernel::Driver>) -> R
    {
        match driver_num {
            _ => f(None),
        }
    }
}

#[link_section = ".flashconfig"]
#[no_mangle]
pub static FLASH_CONFIG_BYTES: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF,
];

#[no_mangle]
pub unsafe fn reset_handler() {
    use mk66::wdog;
    use mk66::gpio;
    use mk66::sim;
    use kernel::hil::Controller;

    // Disable the watchdog
    wdog::WDOG.stop();

    // Enable the Port Control and Interrupt clocks
    sim::enable_clock(sim::clocks::PORTABCDE);

    mk66::init();

    // Turn on the LED
    let led = gpio::PC05.make_gpio();
    led.enable_output();
    led.set();

    // Example configuration for the pin functions
    gpio::PB16.configure(gpio::functions::UART0_RX);
    gpio::PB17.configure(gpio::functions::UART0_TX);

    let teensy = Teensy {
        ipc: kernel::ipc::IPC::new(),
    };

    let mut chip = mk66::chip::MK66::new();

    kernel::main(&teensy, &mut chip, load_processes(), &teensy.ipc);
}


unsafe fn load_processes() -> &'static mut [Option<kernel::Process<'static>>] {
    extern "C" {
        /// Beginning of the ROM region containing the app images.
        static _sapps: u8;
    }

    const NUM_PROCS: usize = 2;

    // Total memory allocated to the processes
    #[link_section = ".app_memory"]
    static mut APP_MEMORY: [u8; 16384] = [0; 16384];

    // How the kernel responds when a process faults
    const FAULT_RESPONSE: kernel::process::FaultResponse = kernel::process::FaultResponse::Panic;

    static mut PROCESSES: [Option<kernel::Process<'static>>; NUM_PROCS] = [None, None];

    // Create the processes and allocate the app memory among them
    let mut apps_in_flash_ptr = &_sapps as *const u8;
    let mut app_memory_ptr = APP_MEMORY.as_mut_ptr();
    let mut app_memory_size = APP_MEMORY.len();
    for i in 0..NUM_PROCS {
        let (process, flash_offset, memory_offset) = kernel::Process::create(apps_in_flash_ptr,
                                                                             app_memory_ptr,
                                                                             app_memory_size,
                                                                             FAULT_RESPONSE);
        if process.is_none() {
            break;
        }

        PROCESSES[i] = process;
        apps_in_flash_ptr = apps_in_flash_ptr.offset(flash_offset as isize);
        app_memory_ptr = app_memory_ptr.offset(memory_offset as isize);
        app_memory_size -= memory_offset;
    }

    &mut PROCESSES
}
