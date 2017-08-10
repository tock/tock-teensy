//! Implementation of the GPIO Controller
//! Resources:
//!     [1] Kinetis K66 Sub-Family Reference Manual

use self::Pin::*;
use core::cell::Cell;
use kernel::common::VolatileCell;
use core::ops::{Index, IndexMut};
use core::mem;
use kernel::hil;
use nvic;
use nvic::NvicIdx::*;

// Register map for a single Port Control and Interrupt module
// [^1]: Section 12.5
#[repr(C, packed)]
struct PortRegisters {
    cr: [VolatileCell<u32>; 32],
    gpclr: VolatileCell<u32>,
    gpchr: VolatileCell<u32>,
    isfr: VolatileCell<u32>,
    dfer: VolatileCell<u32>,
    dfcr: VolatileCell<u32>,
    dfwr: VolatileCell<u32>,
}

// Register map for a single GPIO port
// [^1]: Section 63.3.1
#[repr(C, packed)]
struct GpioRegisters {
    output: VolatileCell<u32>,
    set: VolatileCell<u32>,
    clear: VolatileCell<u32>,
    toggle: VolatileCell<u32>,
    input: VolatileCell<u32>,
    direction: VolatileCell<u32>,
}

pub struct Port<P: PortName> {
    registers: *mut PortRegisters,
    pins: [Pin; 32]
}

impl<P: PortName> Port<P> {
    const fn new() -> Port<P> {
        Port {
            registers: match P::val() {
                PortName::A => PORTA_BASE_ADDRESS as *mut PortRegisters,
                PortName::B => PORTB_BASE_ADDRESS as *mut PortRegisters,
                PortName::C => PORTC_BASE_ADDRESS as *mut PortRegisters,
                PortName::D => PORTD_BASE_ADDRESS as *mut PortRegisters,
                PortName::E => PORTE_BASE_ADDRESS as *mut PortRegisters,
            },
            pins: [Pin<P>::new(0),
                   Pin<P>::new(1),
                   Pin<P>::new(2),
                   Pin<P>::new(3),
                   Pin<P>::new(4),
                   Pin<P>::new(5),
                   Pin<P>::new(6),
                   Pin<P>::new(7),
                   Pin<P>::new(8),
                   Pin<P>::new(9),
                   Pin<P>::new(10),
                   Pin<P>::new(11),
                   Pin<P>::new(12),
                   Pin<P>::new(13),
                   Pin<P>::new(14),
                   Pin<P>::new(15),
                   Pin<P>::new(16),
                   Pin<P>::new(17),
                   Pin<P>::new(18),
                   Pin<P>::new(19),
                   Pin<P>::new(20),
                   Pin<P>::new(21),
                   Pin<P>::new(22),
                   Pin<P>::new(23),
                   Pin<P>::new(24),
                   Pin<P>::new(25),
                   Pin<P>::new(26),
                   Pin<P>::new(27),
                   Pin<P>::new(28),
                   Pin<P>::new(29),
                   Pin<P>::new(30),
                   Pin<P>::new(31)],
        }
    }

    pub fn handle_interrupt(&self) {
        let port_control: &mut PortControlRegisters = unsafe { mem::transmute(self.port_control) };
        let mut fired = port_control.isfr.get();

        // Writing a logic 1 to the interrupt status flag register clears the interrupt
        port_control.isfr.set(!0);

        loop {
            let pin = fired.trailing_zeros() as usize;
            if pin < self.pins.len() {
                fired &= !(1 << pin);
                self.pins[pin].handle_interrupt();
            } else {
                break;
            }
        }
    }
}

pub struct Pin<P: PortName, N: PinNum> {
    port: *mut PortRegisters,
    valid: AtomicBool,
    _portname: PhantomData<P>,
    _pin: PhantomData<N>
}

pub struct PeripheralPin {
    client_data: ()
}

pub struct GPIOPin {
    client_data: ()
}

impl<P: PortName, N: PinNum> Pin<P, N> {
    pub fn to_peripheral_function<F: FunctionNum>(&mut self, f: PF) -> PeripheralPin
        where PF: Function<P, N, F> {

        let was_valid = self.valid.swap(false, Ordering::Relaxed);
        if !was_valid { panic!("Doubly allocated pin!"); }

        // Set the peripheral function
        
        PeripheralPin {
            client_data: (),
        }
    }
}

