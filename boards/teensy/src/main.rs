#![no_std]
#![no_main]
#![feature(asm,const_fn,drop_types_in_const,lang_items,compiler_builtins_lib)]

extern crate capsules;
extern crate compiler_builtins;

#[macro_use(debug, static_init)]
extern crate kernel;

#[macro_use]
extern crate common;

#[allow(dead_code)]
extern crate mk66;

#[macro_use]
pub mod io;

#[allow(dead_code)]
mod tests;

use capsules::timer::TimerDriver;
use capsules::spi::Spi;
use capsules::console::Console;
use capsules::virtual_spi::{VirtualSpiMasterDevice, MuxSpiMaster};
use kernel::hil::spi::SpiMaster;
use kernel::hil::uart::UART;


#[allow(unused)]
struct Teensy {
    console: &'static Console<'static, mk66::uart::Uart>,
    timer: &'static TimerDriver<'static, mk66::pit::Pit<'static>>,
    spi: &'static Spi<'static, VirtualSpiMasterDevice<'static, mk66::spi::Spi<'static>>>,
    ipc: kernel::ipc::IPC,
}

impl kernel::Platform for Teensy {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
        where F: FnOnce(Option<&kernel::Driver>) -> R
    {
        match driver_num {
            0 => f(Some(self.console)),
            // // 1 => f(Some(self.gpio)),

            3 => f(Some(self.timer)),
            4 => f(Some(self.spi)),

            // 8 => f(Some(self.led)),

            0xff => f(Some(&self.ipc)),
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

pub unsafe fn set_pin_primary_functions() {
    use mk66::gpio::*;
    PB17.set_function(functions::UART0_TX);
    PB16.set_function(functions::UART0_RX);

    PD05.set_function(functions::SPI1_SCK);
    PD06.set_function(functions::SPI1_MOSI);
}

#[no_mangle]
pub unsafe fn reset_handler() {
    use mk66::{clock, wdog, sim, pit, spi, uart};

    // Disable the watchdog.
    wdog::stop();

    // Relocate the text and data segments.
    mk66::init();

    // Configure the system clock.
    clock::configure(120);

    // Enable the Port Control and Interrupt clocks.
    sim::enable_clock(sim::clocks::PORTABCDE);

    pit::PIT.init();
    spi::SPI1.init();

    set_pin_primary_functions();

    let console = static_init!(
            Console<uart::Uart>,
            Console::new(&uart::UART0,
                         115200,
                         &mut capsules::console::WRITE_BUF,
                         kernel::Container::create())
        );
    uart::UART0.set_client(console);
    console.initialize();

    let kc = static_init!(
            capsules::console::App,
            capsules::console::App::default()
        );
    kernel::debug::assign_console_driver(Some(console), kc);

    let timer = static_init!(
            TimerDriver<'static, mk66::pit::Pit>,
            TimerDriver::new(&pit::PIT,
                             kernel::Container::create())
        );

    let mux_spi = static_init!(
            MuxSpiMaster<'static, spi::Spi<'static>>,
            MuxSpiMaster::new(&spi::SPI1)
        );

    spi::SPI1.set_client(mux_spi);

    let virtual_spi = static_init!(
            VirtualSpiMasterDevice<'static, spi::Spi<'static>>,
            VirtualSpiMasterDevice::new(mux_spi, 0)
        );

    let spi = static_init!(
            capsules::spi::Spi<'static, VirtualSpiMasterDevice<'static, spi::Spi<'static>>>,
            capsules::spi::Spi::new(virtual_spi)
        );

    static mut SPI_READ_BUF: [u8; 64] = [0; 64];
    static mut SPI_WRITE_BUF: [u8; 64] = [0; 64];
    spi.config_buffers(&mut SPI_READ_BUF, &mut SPI_WRITE_BUF);
    virtual_spi.set_client(spi);

    let teensy = Teensy {
        console: console,
        timer: timer,
        spi: spi,
        ipc: kernel::ipc::IPC::new(),
    };

    let mut chip = mk66::chip::MK66::new();

    if tests::TEST {
        tests::test();
    }

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
