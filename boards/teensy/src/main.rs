#![no_std]
#![no_main]
#![feature(asm,const_fn,lang_items)]

extern crate capsules;

extern crate kernel;

#[allow(dead_code)]
extern crate mk66;

#[macro_use]
pub mod io;

#[allow(dead_code)]
mod tests;

#[allow(dead_code)]
mod spi;

#[allow(dead_code)]
mod components;

//pub mod xconsole;

#[allow(dead_code)]
mod pins;

use components::*;
use kernel::{create_capability, static_init};
use kernel::capabilities;

const NUM_PROCS: usize = 1;

// Total memory allocated to the processes
#[link_section = ".app_memory"]
static mut APP_MEMORY: [u8; 1 << 17] = [0; 1 << 17];

// How the kernel responds when a process faults
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

static mut PROCESSES: [Option<&'static kernel::procs::ProcessType>; NUM_PROCS] = [None; NUM_PROCS];

#[allow(unused)]
struct Teensy {
    //xconsole: <XConsoleComponent as Component>::Output,
    gpio: <GpioComponent as Component>::Output,
    led: <LedComponent as Component>::Output,
    alarm: <AlarmComponent as Component>::Output,
    spi: <VirtualSpiComponent as Component>::Output,
    rng: <RngaComponent as Component>::Output,
    ipc: kernel::ipc::IPC,
}

impl kernel::Platform for Teensy {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
        where F: FnOnce(Option<&kernel::Driver>) -> R
    {
        match driver_num {
            //xconsole::DRIVER_NUM => f(Some(self.xconsole)),
            capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),

            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            spi::DRIVER_NUM => f(Some(self.spi)),

            capsules::led::DRIVER_NUM => f(Some(self.led)),

            capsules::rng::DRIVER_NUM => f(Some(self.rng)),

            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
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
    // Disable the watchdog.
    mk66::wdog::stop();

    // Relocate the text and data segments.
    mk66::init();

    // Configure the system clock.
    mk66::clock::configure(120);

    // Enable the Port Control and Interrupt clocks.
    use mk66::sim::Clock;
    mk66::sim::clocks::PORTABCDE.enable();

    let process_mgmt_cap = create_capability!(capabilities::ProcessManagementCapability);
    let main_cap = create_capability!(capabilities::MainLoopCapability);
    let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));

    
    let (gpio_pins, led_pins) = pins::configure_all_pins();
    let gpio = GpioComponent::new(board_kernel)
                             .dependency(gpio_pins)
                             .finalize().unwrap();
    let led = LedComponent::new()
                           .dependency(led_pins)
                           .finalize().unwrap();
    let spi = VirtualSpiComponent::new().finalize().unwrap();
    let alarm = AlarmComponent::new(board_kernel).finalize().unwrap();
    //let xconsole = XConsoleComponent::new().finalize().unwrap();
    let rng = RngaComponent::new(board_kernel).finalize().unwrap();

    
    let teensy = Teensy {
        //xconsole: xconsole,
        gpio: gpio,
        led: led,
        alarm: alarm,
        spi: spi,
        rng: rng,
        ipc: kernel::ipc::IPC::new(board_kernel, &grant_cap),
    };

    let chip = static_init!(mk66::chip::MK66, mk66::chip::MK66::new());

    if tests::TEST {
        tests::test();
    }



    extern "C" {
        /// Beginning of the ROM region containing the app images.
        static _sapps: u8;
    }
    
    kernel::procs::load_processes(
        board_kernel,
        chip,
        &_sapps as *const u8,
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
        &process_mgmt_cap,
    );
    
    board_kernel.kernel_loop(&teensy, chip, Some(&teensy.ipc), &main_cap);
}

