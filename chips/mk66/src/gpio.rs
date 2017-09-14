//! Implementation of the GPIO Controller
//! Resources:
//!     [1] Kinetis K66 Sub-Family Reference Manual
//!


// let d1 = PA00.make_gpio();
// d1.enable_output();
// d1.set();
//
// PA00.set_function(functions::USART0_RX); // panics? or returns Err
// PA00.reclaim(); // puts in low power mode
//
// PA00.set_function(functions::USART0_RX); // ok
// PA00.reclaim();
//
// PA00.set_function(functions::USART1_RX); // won't compile
//

use core::cell::Cell;
use core::sync::atomic::Ordering;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use kernel::common::VolatileCell;
use core::mem;
use kernel::hil;

// Register map for a single Port Control and Interrupt module
// [^1]: Section 12.5
#[repr(C, packed)]
pub struct PortRegisters {
    cr: [VolatileCell<u32>; 32],
    gpclr: VolatileCell<u32>,
    gpchr: VolatileCell<u32>,
    isfr: VolatileCell<u32>,
    dfer: VolatileCell<u32>,
    dfcr: VolatileCell<u32>,
    dfwr: VolatileCell<u32>,
}

// Register map for a single GPIO port--aliased to bitband region
// [^1]: Section 63.3.1
#[repr(C, packed)]
pub struct GpioBitbandRegisters {
    output: [VolatileCell<u32>; 32],
    set: [VolatileCell<u32>; 32],
    clear: [VolatileCell<u32>; 32],
    toggle: [VolatileCell<u32>; 32],
    input: [VolatileCell<u32>; 32],
    direction: [VolatileCell<u32>; 32],
}

macro_rules! pins {
    ( $($name:ident => $pin:expr),* ) => {
        $(
            pub struct $name;
            impl PinNum for $name {
                const PIN: usize =  $pin;
            }
        )*
    }
}

pub trait PinNum {
    const PIN: usize;
}

pins! {
    PinA00 => 0, PinA01 => 1, PinA02 => 2, PinA03 => 3, PinA04 => 4, PinA05 => 5, PinA06 => 6, PinA07 => 7,
    PinA08 => 8, PinA09 => 9, PinA10 => 10, PinA11 => 11, PinA12 => 12, PinA13 => 13, PinA14 => 14, PinA15 => 15,
    PinA16 => 16, PinA17 => 17, PinA18 => 18, PinA19 => 19, PinA20 => 20, PinA21 => 21, PinA22 => 22, PinA23 => 23,
    PinA24 => 24, PinA25 => 25, PinA26 => 26, PinA27 => 27, PinA28 => 28, PinA29 => 29, PinA30 => 30, PinA31 => 31,

    PinB00 => 32, PinB01 => 33, PinB02 => 34, PinB03 => 35, PinB04 => 36, PinB05 => 37, PinB06 => 38, PinB07 => 39,
    PinB08 => 40, PinB09 => 41, PinB10 => 42, PinB11 => 43, PinB12 => 44, PinB13 => 45, PinB14 => 46, PinB15 => 47,
    PinB16 => 48, PinB17 => 49, PinB18 => 50, PinB19 => 51, PinB20 => 52, PinB21 => 53, PinB22 => 54, PinB23 => 55,
    PinB24 => 56, PinB25 => 57, PinB26 => 58, PinB27 => 59, PinB28 => 60, PinB29 => 61, PinB30 => 62, PinB31 => 63,

    PinC00 => 64, PinC01 => 65, PinC02 => 66, PinC03 => 67, PinC04 => 68, PinC05 => 69, PinC06 => 70, PinC07 => 71,
    PinC08 => 72, PinC09 => 73, PinC10 => 74, PinC11 => 75, PinC12 => 76, PinC13 => 77, PinC14 => 78, PinC15 => 79,
    PinC16 => 80, PinC17 => 81, PinC18 => 82, PinC19 => 83, PinC20 => 84, PinC21 => 85, PinC22 => 86, PinC23 => 87,
    PinC24 => 88, PinC25 => 89, PinC26 => 90, PinC27 => 91, PinC28 => 92, PinC29 => 93, PinC30 => 94, PinC31 => 95,

    PinD00 => 96, PinD01 => 97, PinD02 => 98, PinD03 => 99, PinD04 => 100, PinD05 => 101, PinD06 => 102, PinD07 => 103,
    PinD08 => 104, PinD09 => 105, PinD10 => 106, PinD11 => 107, PinD12 => 108, PinD13 => 109, PinD14 => 110, PinD15 => 111,
    PinD16 => 112, PinD17 => 113, PinD18 => 114, PinD19 => 115, PinD20 => 116, PinD21 => 117, PinD22 => 118, PinD23 => 119,
    PinD24 => 120, PinD25 => 121, PinD26 => 122, PinD27 => 123, PinD28 => 124, PinD29 => 125, PinD30 => 126, PinD31 => 127,

    PinE00 => 128, PinE01 => 129, PinE02 => 130, PinE03 => 131, PinE04 => 132, PinE05 => 133, PinE06 => 134, PinE07 => 135,
    PinE08 => 136, PinE09 => 137, PinE10 => 138, PinE11 => 139, PinE12 => 140, PinE13 => 141, PinE14 => 142, PinE15 => 143,
    PinE16 => 144, PinE17 => 145, PinE18 => 146, PinE19 => 147, PinE20 => 148, PinE21 => 149, PinE22 => 150, PinE23 => 151,
    PinE24 => 152, PinE25 => 153, PinE26 => 154, PinE27 => 155, PinE28 => 156, PinE29 => 157, PinE30 => 158, PinE31 => 159
}

pub enum PeripheralFunction {
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
    Alt6,
    Alt7
}

pub struct Port {
    port: *mut PortRegisters,
    pins: [&'static PinInterrupt; 32]
}

pub trait PinInterrupt {
    fn handle_interrupt(&self);
}

pub struct Pin<P: PinNum> {
    gpio: Gpio,
    port_regs: *mut PortRegisters,
    valid: AtomicBool,
    client: Cell<Option<&'static hil::gpio::Client>>,
    client_data: Cell<usize>,
    _pin: PhantomData<P>
}

pub struct Gpio {
    regs: *mut GpioBitbandRegisters,
    pin: usize,
}

impl Port {
    pub fn handle_interrupt(&self) {
        let port: &mut PortRegisters = unsafe { mem::transmute(self.port) };
        let mut fired = port.isfr.get();

        // Writing a logic 1 to the interrupt status flag register clears the interrupt
        port.isfr.set(!0);

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

impl<P: PinNum> PinInterrupt for Pin<P> {
    fn handle_interrupt(&self) {
        self.client.get().map(|client| client.fired(self.client_data.get()));
    }
}

const PORT_BASE_ADDRESS: usize = 0x4004_9000; 
const PORT_SIZE: usize = 0x1000;

const GPIO_BASE_ADDRESS: usize = 0x43FE_0000;
const GPIO_SIZE: usize = 0x800;

impl<P: PinNum> Pin<P> {
    const fn new() -> Pin<P> {
        Pin {
            gpio: Gpio {
                regs: (GPIO_BASE_ADDRESS + ((P::PIN / 32) * GPIO_SIZE)) as *mut GpioBitbandRegisters,
                pin: P::PIN % 32,
            },
            port_regs: (PORT_BASE_ADDRESS + ((P::PIN / 32) * PORT_SIZE)) as *mut PortRegisters,
            valid: ATOMIC_BOOL_INIT,
            client: Cell::new(None),
            client_data: Cell::new(0),
            _pin: PhantomData,
        }
    }

    pub fn make_gpio(&self) -> &Gpio {
        let already_allocated = self.valid.swap(true, Ordering::Relaxed);
        if already_allocated {
            panic!("Requested GPIO pin {} is already allocated.", self.gpio.pin);
        }

        self.set_peripheral_function(PeripheralFunction::Alt1);

        &self.gpio
    }

    pub fn set_function(&self, function: functions::Function<P>) {

        let already_allocated = self.valid.swap(true, Ordering::Relaxed);
        if already_allocated {
            return;
        }

        self.set_peripheral_function(function.val);
    }

    pub fn set_peripheral_function(&self, function: PeripheralFunction) {
        let port: &mut PortRegisters = unsafe { mem::transmute(self.port_regs) };

        const MUX_MASK: u32 = 0b111 << 8;
        let cr: u32 = port.cr[self.gpio.pin].get() & !MUX_MASK;

        port.cr[self.gpio.pin].set(cr | ((function as u32) << 8));
    }

    pub fn reclaim(&self) {
        self.client.set(None);
        self.client_data.set(0);
        self.set_peripheral_function(PeripheralFunction::Alt0);
        self.valid.swap(false, Ordering::Relaxed);
    }

    pub fn set_input_mode(&self, mode: hil::gpio::InputMode) {
        let port: &mut PortRegisters = unsafe { mem::transmute(self.port_regs) };
        let mode_bits = match mode {
            hil::gpio::InputMode::PullUp => 0b11,
            hil::gpio::InputMode::PullDown => 0b10,
            hil::gpio::InputMode::PullNone => 0b00,
        };

        const MODE_MASK: u32 = 0b11;
        let cr: u32 = port.cr[self.gpio.pin].get() & !MODE_MASK;

        port.cr[self.gpio.pin].set(cr | ((mode_bits & MODE_MASK)));
    }

    pub fn set_interrupt_mode(&self, _mode: hil::gpio::InterruptMode) {
    }

    pub fn set_client<C: hil::gpio::Client>(&self, client: &'static C) {
        if !self.valid.load(Ordering::Relaxed) {
            return;
        }

        self.client.set(Some(client));
    }

    pub fn set_client_data(&self, data: usize) {
        if !self.valid.load(Ordering::Relaxed) {
            return;
        }

        self.client_data.set(data);
    }
}

impl Gpio {
    pub fn disable_output(&self) {
        let gpio: &mut GpioBitbandRegisters = unsafe { mem::transmute(self.regs) };
        gpio.direction[self.pin].set(0);
    }

    pub fn enable_output(&self) {
        let gpio: &mut GpioBitbandRegisters = unsafe { mem::transmute(self.regs) };
        gpio.direction[self.pin].set(1);
    }

    pub fn read(&self) -> bool {
        let gpio: &mut GpioBitbandRegisters = unsafe { mem::transmute(self.regs) };
        gpio.input[self.pin].get() > 0
    }

    pub fn toggle(&self) {
        let gpio: &mut GpioBitbandRegisters = unsafe { mem::transmute(self.regs) };
        gpio.toggle[self.pin].set(1);
    }

    pub fn set(&self) {
        let gpio: &mut GpioBitbandRegisters = unsafe { mem::transmute(self.regs) };
        gpio.set[self.pin].set(1);
    }

    pub fn clear(&self) {
        let gpio: &mut GpioBitbandRegisters = unsafe { mem::transmute(self.regs) };
        gpio.clear[self.pin].set(1);
    }
}

impl<P: PinNum> hil::Controller for Pin<P> {
    type Config = functions::Function<P>;

    fn configure(&self, config: Self::Config) {
        self.set_function(config);
    }
}

impl<P: PinNum> hil::gpio::PinCtl for Pin<P> {
    fn set_input_mode(&self, mode: hil::gpio::InputMode) {
        Pin::set_input_mode(self, mode);
    }
}

impl<P: PinNum> hil::gpio::Pin for Pin<P> {
    fn disable(&self) {
        self.reclaim();
    }

    fn make_output(&self) {
        self.gpio.enable_output();
    }

    fn make_input(&self) {
        self.gpio.disable_output();
    }

    fn read(&self) -> bool {
        self.gpio.read()
    }

    fn toggle(&self) {
        self.gpio.toggle();
    }

    fn set(&self) {
        self.gpio.set();
    }

    fn clear(&self) {
        self.gpio.clear();
    }

    fn enable_interrupt(&self, _client_data: usize, _mode: hil::gpio::InterruptMode) {
    }

    fn disable_interrupt(&self) {
    }
}

pub static mut PA00: Pin<PinA00> = Pin::new();
pub static mut PA01: Pin<PinA01> = Pin::new();
pub static mut PA02: Pin<PinA02> = Pin::new();
pub static mut PA03: Pin<PinA03> = Pin::new();
pub static mut PA04: Pin<PinA04> = Pin::new();
pub static mut PA05: Pin<PinA05> = Pin::new();
pub static mut PA06: Pin<PinA06> = Pin::new();
pub static mut PA07: Pin<PinA07> = Pin::new();
pub static mut PA08: Pin<PinA08> = Pin::new();
pub static mut PA09: Pin<PinA09> = Pin::new();
pub static mut PA10: Pin<PinA10> = Pin::new();
pub static mut PA11: Pin<PinA11> = Pin::new();
pub static mut PA12: Pin<PinA12> = Pin::new();
pub static mut PA13: Pin<PinA13> = Pin::new();
pub static mut PA14: Pin<PinA14> = Pin::new();
pub static mut PA15: Pin<PinA15> = Pin::new();
pub static mut PA16: Pin<PinA16> = Pin::new();
pub static mut PA17: Pin<PinA17> = Pin::new();
pub static mut PA18: Pin<PinA18> = Pin::new();
pub static mut PA19: Pin<PinA19> = Pin::new();
pub static mut PA20: Pin<PinA20> = Pin::new();
pub static mut PA21: Pin<PinA21> = Pin::new();
pub static mut PA22: Pin<PinA22> = Pin::new();
pub static mut PA23: Pin<PinA23> = Pin::new();
pub static mut PA24: Pin<PinA24> = Pin::new();
pub static mut PA25: Pin<PinA25> = Pin::new();
pub static mut PA26: Pin<PinA26> = Pin::new();
pub static mut PA27: Pin<PinA27> = Pin::new();
pub static mut PA28: Pin<PinA28> = Pin::new();
pub static mut PA29: Pin<PinA29> = Pin::new();
pub static mut PA30: Pin<PinA30> = Pin::new();
pub static mut PA31: Pin<PinA31> = Pin::new();

pub static mut PB00: Pin<PinB00> = Pin::new();
pub static mut PB01: Pin<PinB01> = Pin::new();
pub static mut PB02: Pin<PinB02> = Pin::new();
pub static mut PB03: Pin<PinB03> = Pin::new();
pub static mut PB04: Pin<PinB04> = Pin::new();
pub static mut PB05: Pin<PinB05> = Pin::new();
pub static mut PB06: Pin<PinB06> = Pin::new();
pub static mut PB07: Pin<PinB07> = Pin::new();
pub static mut PB08: Pin<PinB08> = Pin::new();
pub static mut PB09: Pin<PinB09> = Pin::new();
pub static mut PB10: Pin<PinB10> = Pin::new();
pub static mut PB11: Pin<PinB11> = Pin::new();
pub static mut PB12: Pin<PinB12> = Pin::new();
pub static mut PB13: Pin<PinB13> = Pin::new();
pub static mut PB14: Pin<PinB14> = Pin::new();
pub static mut PB15: Pin<PinB15> = Pin::new();
pub static mut PB16: Pin<PinB16> = Pin::new();
pub static mut PB17: Pin<PinB17> = Pin::new();
pub static mut PB18: Pin<PinB18> = Pin::new();
pub static mut PB19: Pin<PinB19> = Pin::new();
pub static mut PB20: Pin<PinB20> = Pin::new();
pub static mut PB21: Pin<PinB21> = Pin::new();
pub static mut PB22: Pin<PinB22> = Pin::new();
pub static mut PB23: Pin<PinB23> = Pin::new();
pub static mut PB24: Pin<PinB24> = Pin::new();
pub static mut PB25: Pin<PinB25> = Pin::new();
pub static mut PB26: Pin<PinB26> = Pin::new();
pub static mut PB27: Pin<PinB27> = Pin::new();
pub static mut PB28: Pin<PinB28> = Pin::new();
pub static mut PB29: Pin<PinB29> = Pin::new();
pub static mut PB30: Pin<PinB30> = Pin::new();
pub static mut PB31: Pin<PinB31> = Pin::new();

pub static mut PC00: Pin<PinC00> = Pin::new();
pub static mut PC01: Pin<PinC01> = Pin::new();
pub static mut PC02: Pin<PinC02> = Pin::new();
pub static mut PC03: Pin<PinC03> = Pin::new();
pub static mut PC04: Pin<PinC04> = Pin::new();
pub static mut PC05: Pin<PinC05> = Pin::new();
pub static mut PC06: Pin<PinC06> = Pin::new();
pub static mut PC07: Pin<PinC07> = Pin::new();
pub static mut PC08: Pin<PinC08> = Pin::new();
pub static mut PC09: Pin<PinC09> = Pin::new();
pub static mut PC10: Pin<PinC10> = Pin::new();
pub static mut PC11: Pin<PinC11> = Pin::new();
pub static mut PC12: Pin<PinC12> = Pin::new();
pub static mut PC13: Pin<PinC13> = Pin::new();
pub static mut PC14: Pin<PinC14> = Pin::new();
pub static mut PC15: Pin<PinC15> = Pin::new();
pub static mut PC16: Pin<PinC16> = Pin::new();
pub static mut PC17: Pin<PinC17> = Pin::new();
pub static mut PC18: Pin<PinC18> = Pin::new();
pub static mut PC19: Pin<PinC19> = Pin::new();
pub static mut PC20: Pin<PinC20> = Pin::new();
pub static mut PC21: Pin<PinC21> = Pin::new();
pub static mut PC22: Pin<PinC22> = Pin::new();
pub static mut PC23: Pin<PinC23> = Pin::new();
pub static mut PC24: Pin<PinC24> = Pin::new();
pub static mut PC25: Pin<PinC25> = Pin::new();
pub static mut PC26: Pin<PinC26> = Pin::new();
pub static mut PC27: Pin<PinC27> = Pin::new();
pub static mut PC28: Pin<PinC28> = Pin::new();
pub static mut PC29: Pin<PinC29> = Pin::new();
pub static mut PC30: Pin<PinC30> = Pin::new();
pub static mut PC31: Pin<PinC31> = Pin::new();

pub static mut PD00: Pin<PinD00> = Pin::new();
pub static mut PD01: Pin<PinD01> = Pin::new();
pub static mut PD02: Pin<PinD02> = Pin::new();
pub static mut PD03: Pin<PinD03> = Pin::new();
pub static mut PD04: Pin<PinD04> = Pin::new();
pub static mut PD05: Pin<PinD05> = Pin::new();
pub static mut PD06: Pin<PinD06> = Pin::new();
pub static mut PD07: Pin<PinD07> = Pin::new();
pub static mut PD08: Pin<PinD08> = Pin::new();
pub static mut PD09: Pin<PinD09> = Pin::new();
pub static mut PD10: Pin<PinD10> = Pin::new();
pub static mut PD11: Pin<PinD11> = Pin::new();
pub static mut PD12: Pin<PinD12> = Pin::new();
pub static mut PD13: Pin<PinD13> = Pin::new();
pub static mut PD14: Pin<PinD14> = Pin::new();
pub static mut PD15: Pin<PinD15> = Pin::new();
pub static mut PD16: Pin<PinD16> = Pin::new();
pub static mut PD17: Pin<PinD17> = Pin::new();
pub static mut PD18: Pin<PinD18> = Pin::new();
pub static mut PD19: Pin<PinD19> = Pin::new();
pub static mut PD20: Pin<PinD20> = Pin::new();
pub static mut PD21: Pin<PinD21> = Pin::new();
pub static mut PD22: Pin<PinD22> = Pin::new();
pub static mut PD23: Pin<PinD23> = Pin::new();
pub static mut PD24: Pin<PinD24> = Pin::new();
pub static mut PD25: Pin<PinD25> = Pin::new();
pub static mut PD26: Pin<PinD26> = Pin::new();
pub static mut PD27: Pin<PinD27> = Pin::new();
pub static mut PD28: Pin<PinD28> = Pin::new();
pub static mut PD29: Pin<PinD29> = Pin::new();
pub static mut PD30: Pin<PinD30> = Pin::new();
pub static mut PD31: Pin<PinD31> = Pin::new();

pub static mut PE00: Pin<PinE00> = Pin::new();
pub static mut PE01: Pin<PinE01> = Pin::new();
pub static mut PE02: Pin<PinE02> = Pin::new();
pub static mut PE03: Pin<PinE03> = Pin::new();
pub static mut PE04: Pin<PinE04> = Pin::new();
pub static mut PE05: Pin<PinE05> = Pin::new();
pub static mut PE06: Pin<PinE06> = Pin::new();
pub static mut PE07: Pin<PinE07> = Pin::new();
pub static mut PE08: Pin<PinE08> = Pin::new();
pub static mut PE09: Pin<PinE09> = Pin::new();
pub static mut PE10: Pin<PinE10> = Pin::new();
pub static mut PE11: Pin<PinE11> = Pin::new();
pub static mut PE12: Pin<PinE12> = Pin::new();
pub static mut PE13: Pin<PinE13> = Pin::new();
pub static mut PE14: Pin<PinE14> = Pin::new();
pub static mut PE15: Pin<PinE15> = Pin::new();
pub static mut PE16: Pin<PinE16> = Pin::new();
pub static mut PE17: Pin<PinE17> = Pin::new();
pub static mut PE18: Pin<PinE18> = Pin::new();
pub static mut PE19: Pin<PinE19> = Pin::new();
pub static mut PE20: Pin<PinE20> = Pin::new();
pub static mut PE21: Pin<PinE21> = Pin::new();
pub static mut PE22: Pin<PinE22> = Pin::new();
pub static mut PE23: Pin<PinE23> = Pin::new();
pub static mut PE24: Pin<PinE24> = Pin::new();
pub static mut PE25: Pin<PinE25> = Pin::new();
pub static mut PE26: Pin<PinE26> = Pin::new();
pub static mut PE27: Pin<PinE27> = Pin::new();
pub static mut PE28: Pin<PinE28> = Pin::new();
pub static mut PE29: Pin<PinE29> = Pin::new();
pub static mut PE30: Pin<PinE30> = Pin::new();
pub static mut PE31: Pin<PinE31> = Pin::new();

pub static mut PA: Port = unsafe { 
    Port {
        pins: [&PA00, &PA01, &PA02, &PA03, &PA04, &PA05, &PA06, &PA07,
               &PA08, &PA09, &PA10, &PA11, &PA12, &PA13, &PA14, &PA15,
               &PA16, &PA17, &PA18, &PA19, &PA20, &PA21, &PA22, &PA23,
               &PA24, &PA25, &PA26, &PA27, &PA28, &PA29, &PA30, &PA31],
        port: PORT_BASE_ADDRESS as *mut PortRegisters
    }
};

pub static mut PB: Port = unsafe { 
    Port {
        pins: [&PB00, &PB01, &PB02, &PB03, &PB04, &PB05, &PB06, &PB07,
               &PB08, &PB09, &PB10, &PB11, &PB12, &PB13, &PB14, &PB15,
               &PB16, &PB17, &PB18, &PB19, &PB20, &PB21, &PB22, &PB23,
               &PB24, &PB25, &PB26, &PB27, &PB28, &PB29, &PB30, &PB31],
        port: (PORT_BASE_ADDRESS + PORT_SIZE) as *mut PortRegisters
    }
};

pub static mut PC: Port = unsafe { 
    Port {
        pins: [&PC00, &PC01, &PC02, &PC03, &PC04, &PC05, &PC06, &PC07,
               &PC08, &PC09, &PC10, &PC11, &PC12, &PC13, &PC14, &PC15,
               &PC16, &PC17, &PC18, &PC19, &PC20, &PC21, &PC22, &PC23,
               &PC24, &PC25, &PC26, &PC27, &PC28, &PC29, &PC30, &PC31],
        port: (PORT_BASE_ADDRESS + 2*PORT_SIZE) as *mut PortRegisters
    }
};

pub static mut PD: Port = unsafe { 
    Port {
        pins: [&PD00, &PD01, &PD02, &PD03, &PD04, &PD05, &PD06, &PD07,
               &PD08, &PD09, &PD10, &PD11, &PD12, &PD13, &PD14, &PD15,
               &PD16, &PD17, &PD18, &PD19, &PD20, &PD21, &PD22, &PD23,
               &PD24, &PD25, &PD26, &PD27, &PD28, &PD29, &PD30, &PD31],
        port: (PORT_BASE_ADDRESS + 3*PORT_SIZE) as *mut PortRegisters
    }
};

pub static mut PE: Port = unsafe { 
    Port {
        pins: [&PE00, &PE01, &PE02, &PE03, &PE04, &PE05, &PE06, &PE07,
               &PE08, &PE09, &PE10, &PE11, &PE12, &PE13, &PE14, &PE15,
               &PE16, &PE17, &PE18, &PE19, &PE20, &PE21, &PE22, &PE23,
               &PE24, &PE25, &PE26, &PE27, &PE28, &PE29, &PE30, &PE31],
        port: (PORT_BASE_ADDRESS + 4*PORT_SIZE) as *mut PortRegisters
    }
};

pub mod functions {
    use gpio::*;
    use core::marker::PhantomData;
    use gpio::PeripheralFunction::*;

    pub struct Function<P: PinNum> {
        _pin: PhantomData<P>,
        pub val: PeripheralFunction, 
    }

    impl<P: PinNum> Function<P> {
        const fn new(val: PeripheralFunction) -> Function<P> {
            Function {
                _pin: PhantomData,
                val: val
            }
        }
    }

    // Peripheral assignments
    // UART0: PB16, PB17
    pub const UART0_RX: Function<PinB16> = Function::new(Alt3);
    pub const UART0_TX: Function<PinB17> = Function::new(Alt3);

    // SPI1
    pub const SPI1_MOSI: Function<PinD06> = Function::new(Alt7);
    pub const SPI1_SCK: Function<PinD05> = Function::new(Alt7);
}
