//! Implementation of the GPIO Controller
//! Resources:
//!     [1] Kinetis K66 Sub-Family Reference Manual
//!

use core::cell::Cell;
use core::sync::atomic::Ordering;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool};
use kernel::common::registers::{register_bitfields, ReadWrite, WriteOnly, ReadOnly};
use core::mem;
use kernel::hil;
use nvic::{self, NvicIdx};

// Register map for a single Port Control and Interrupt module
// [^1]: Section 12.5
#[repr(C)]
pub struct PortRegisters {
    pcr: [ReadWrite<u32, PinControl::Register>; 32],
    gpclr: WriteOnly<u32>,
    gpchr: WriteOnly<u32>,
    _reserved0: [ReadOnly<u32>; 6],
    isfr: ReadWrite<u32>,
    _reserved1: [ReadOnly<u32>; 7],
    dfer: ReadWrite<u32>,
    dfcr: ReadWrite<u32>,
    dfwr: ReadWrite<u32>,
}

register_bitfields! [ u32,
    PinControl [
        ISF OFFSET(24) NUMBITS(1) [],
        IRQC OFFSET(16) NUMBITS(4) [
            InterruptDisabled = 0,
            DmaRisingEdge = 1,
            DmaFallingEdge = 2,
            DmaEitherEdge = 3,
            InterruptLogicLow = 8,
            InterruptRisingEdge = 9,
            InterruptFallingEdge = 10,
            InterruptEitherEdge = 11,
            InterruptLogicHigh = 12
        ],
        LK OFFSET(15) NUMBITS(1) [],
        MUX OFFSET(8) NUMBITS(3) [],
        DSE OFFSET(6) NUMBITS(1) [],
        ODE OFFSET(5) NUMBITS(1) [],
        PFE OFFSET(4) NUMBITS(1) [],
        SRE OFFSET(2) NUMBITS(1) [],
        PE OFFSET(1) NUMBITS(1) [],
        PS OFFSET(0) NUMBITS(1) [
            PullDown = 0,
            PullUp = 1
        ]
    ]
];

// Register map for a single GPIO port--aliased to bitband region
// [^1]: Section 63.3.1
#[repr(C)]
pub struct GpioBitbandRegisters {
    output: [ReadWrite<u32>; 32],
    set: [ReadWrite<u32>; 32],
    clear: [ReadWrite<u32>; 32],
    toggle: [ReadWrite<u32>; 32],
    input: [ReadWrite<u32>; 32],
    direction: [ReadWrite<u32>; 32],
}

pub trait PinNum {
    const PIN: usize;
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

pub struct Port<'a> {
    regs: *mut PortRegisters,
    bitband: *mut GpioBitbandRegisters,
    clients: [Cell<Option<&'a dyn hil::gpio::Client>>; 32],
}

pub struct Pin<'a, P: PinNum> {
    gpio: Gpio<'a>,
    _pin: PhantomData<P>
}

pub struct Gpio<'a> {
    pin: usize,
    port: &'a Port<'a>,
    valid: AtomicBool
}

impl<'a> Port<'a> {
    const fn new(port: usize) -> Port<'a> {
        Port {
            regs: (PORT_BASE_ADDRESS + port*PORT_SIZE) as *mut PortRegisters,
            bitband: (GPIO_BASE_ADDRESS + port*GPIO_SIZE) as *mut GpioBitbandRegisters,
            clients: [Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None),
                      Cell::new(None), Cell::new(None)
                ],
        }
    }

    pub fn regs(&self) -> &mut PortRegisters {
        unsafe { mem::transmute(self.regs) }
    }

    pub fn handle_interrupt(&self) {
        let regs = self.regs();
        let mut fired = regs.isfr.get();

        // Writing a logic 1 to the interrupt status flag register clears the interrupt
        regs.isfr.set(!0);

        loop {
            let pin = fired.trailing_zeros() as usize;
            if pin < self.clients.len() {
                fired &= !(1 << pin);
                self.clients[pin].get().map(|client| {
                    client.fired();
                });
            } else {
                break;
            }
        }
    }
}

const PORT_BASE_ADDRESS: usize = 0x4004_9000;
const PORT_SIZE: usize = 0x1000;

const GPIO_BASE_ADDRESS: usize = 0x43FE_0000;
const GPIO_SIZE: usize = 0x800;

impl<'a, P: PinNum> Pin<'a, P> {
    const fn new(port: &'a Port<'a>) -> Pin<'a, P> {
        Pin {
            gpio: Gpio {
                pin: P::PIN,
                port: port,
                valid: AtomicBool::new(false),
            },
            _pin: PhantomData,
        }
    }

    pub fn claim_as_gpio(&mut self) -> &mut Gpio<'a> {
        let already_allocated = self.gpio.valid.swap(true, Ordering::Relaxed);
        if already_allocated {
            let port_name = match self.gpio.pin {
                0..=31 => "A",
                32..=63 => "B",
                64..=95 => "C",
                96..=127 => "D",
                _ => "E"
            };
            panic!("Requested GPIO pin P{}{} is already allocated.", port_name, self.gpio.index());
        }

        self.set_peripheral_function(PeripheralFunction::Alt1);

        &mut self.gpio
    }

    pub fn claim_as(&self, function: functions::Function<P>) {
        let already_allocated = self.gpio.valid.swap(true, Ordering::Relaxed);
        if already_allocated {
            return;
        }

        self.set_peripheral_function(function.val);
    }

    fn set_peripheral_function(&self, function: PeripheralFunction) {
        let port = self.gpio.port.regs();

        port.pcr[self.gpio.index()].modify(PinControl::MUX.val(function as u32));
    }

    pub fn release_claim(&self) {
        self.gpio.clear_client();
        self.set_peripheral_function(PeripheralFunction::Alt0);
        self.gpio.valid.swap(false, Ordering::Relaxed);
    }
}

impl<'a> Gpio<'a> {
    pub fn regs(&self) -> &mut GpioBitbandRegisters {
        unsafe { mem::transmute(self.port.bitband) }
    }

    fn index(&self) -> usize {
        (self.pin % 32) as usize
    }

    pub fn disable_output(&self) {
        self.regs().direction[self.index()].set(0);
    }

    pub fn enable_output(&self) {
        self.regs().direction[self.index()].set(1);
    }

    pub fn read(&self) -> bool {
        self.regs().input[self.index()].get() > 0
    }

    pub fn toggle(&self) {
        self.regs().toggle[self.index()].set(1);
    }

    pub fn set(&self) {
        self.regs().set[self.index()].set(1);
    }

    pub fn clear(&self) {
        self.regs().clear[self.index()].set(1);
    }

    pub fn set_floating_state(&self, mode: hil::gpio::FloatingState) {
        let config = match mode {
            hil::gpio::FloatingState::PullUp => PinControl::PE::SET + PinControl::PS::PullUp,
            hil::gpio::FloatingState::PullDown => PinControl::PE::SET + PinControl::PS::PullDown,
            hil::gpio::FloatingState::PullNone => PinControl::PE::CLEAR,
        };

        self.port.regs().pcr[self.index()].modify(config);
    }

    pub fn floating_state(&self) -> hil::gpio::FloatingState {
        let enable = self.port.regs().pcr[self.index()].read(PinControl::PE) == 1;
        let pullup = self.port.regs().pcr[self.index()].read(PinControl::PS) == 1;
        match (enable, pullup) {
            (false, _) => {
                hil::gpio::FloatingState::PullNone
            },
            (_, false) => {
                hil::gpio::FloatingState::PullDown
            }
            (_, true) => {
                hil::gpio::FloatingState::PullUp
            },
        }
    }

    pub fn set_interrupt_mode(&self, mode: hil::gpio::InterruptEdge) {
        let config = match mode {
            hil::gpio::InterruptEdge::RisingEdge => PinControl::IRQC::InterruptRisingEdge,
            hil::gpio::InterruptEdge::FallingEdge => PinControl::IRQC::InterruptFallingEdge,
            hil::gpio::InterruptEdge::EitherEdge => PinControl::IRQC::InterruptEitherEdge
        };

        self.port.regs().pcr[self.index()].modify(config);
    }

    fn clear_interrupt_status_flag(&self) {
        self.port.regs().pcr[self.index()].modify(PinControl::ISF::SET);
    }

    fn enable_interrupt(&self) {
        unsafe {
            match self.pin {
                0..=31 => nvic::enable(NvicIdx::PCMA),
                32..=63 => nvic::enable(NvicIdx::PCMB),
                64..=95 => nvic::enable(NvicIdx::PCMC),
                96..=127 => nvic::enable(NvicIdx::PCMD),
                _ => nvic::enable(NvicIdx::PCME)
            };
        }
    }

    fn disable_interrupt(&self) {
        self.clear_interrupt_status_flag();
        self.port.regs().pcr[self.index()].modify(PinControl::IRQC::InterruptDisabled);
    }

    pub fn clear_client(&self) {
        if !self.valid.load(Ordering::Relaxed) {
            return;
        }

        self.port.clients[self.index()].set(None);
    }

    pub fn set_client(&self, client: &'static dyn hil::gpio::Client) {
        if !self.valid.load(Ordering::Relaxed) {
            return;
        }

        self.port.clients[self.index()].set(Some(client));
    }

}

impl<'a> hil::gpio::Configure for Gpio<'a> {
    fn disable_input(&self) -> hil::gpio::Configuration {
        self.make_output()
    }

    fn disable_output(&self) -> hil::gpio::Configuration {
        self.make_input()
    }

    fn make_output(&self) -> hil::gpio::Configuration {
        self.enable_output();
        self.configuration()
    }

    fn make_input(&self) -> hil::gpio::Configuration {
        self.disable_output();
        self.configuration()
    }

    fn set_floating_state(&self, mode: hil::gpio::FloatingState) {
        Gpio::set_floating_state(self, mode);
    }

    fn floating_state(&self) -> hil::gpio::FloatingState {
        Gpio::floating_state(self)
    }

    fn deactivate_to_low_power(&self) {
        unimplemented!();
    }

    fn configuration(&self) -> hil::gpio::Configuration {
        match self.regs().direction[self.index()].get() == 1 {
            false => hil::gpio::Configuration::Output,
            true  => hil::gpio::Configuration::Input,
        }

    }
}

impl<'a> hil::gpio::Input for Gpio<'a> {
    fn read(&self) -> bool {
        self.read()
    }
}

impl<'a> hil::gpio::Output for Gpio<'a> {
    fn toggle(&self) -> bool {
        self.toggle();
        self.read()
    }

    fn set(&self) {
        self.set();
    }

    fn clear(&self) {
        self.clear();
    }
}

impl<'a> hil::gpio::Interrupt for Gpio<'a> {
    fn set_client(&self, client: &'static dyn hil::gpio::Client) {
        Gpio::set_client(self, client);
    }

    fn enable_interrupts(&self,  mode: hil::gpio::InterruptEdge) {
        self.set_interrupt_mode(mode);
        Gpio::enable_interrupt(self);
    }

    fn disable_interrupts(&self) {
        Gpio::disable_interrupt(self);
    }

    fn is_pending(&self) -> bool {
        false
    } 
}

macro_rules! pins {
    {$($port:ident [$($pintype:ident $pinname:ident $pinnum:expr),*]),*} => {
        $(
            $(
                pub struct $pintype;
                impl PinNum for $pintype {
                    const PIN: usize =  $pinnum;
                }
                pub static mut $pinname: Pin<$pintype> = Pin::new(unsafe { &$port });
            )*
        )*
    }
}

pub static mut PA: Port = Port::new(0);
pub static mut PB: Port = Port::new(1);
pub static mut PC: Port = Port::new(2);
pub static mut PD: Port = Port::new(3);
pub static mut PE: Port = Port::new(4);

pins! {
    PA [PinA00 PA00 0, PinA01 PA01 1, PinA02 PA02 2, PinA03 PA03 3,
        PinA04 PA04 4, PinA05 PA05 5, PinA06 PA06 6, PinA07 PA07 7,
        PinA08 PA08 8, PinA09 PA09 9, PinA10 PA10 10, PinA11 PA11 11,
        PinA12 PA12 12, PinA13 PA13 13, PinA14 PA14 14, PinA15 PA15 15,
        PinA16 PA16 16, PinA17 PA17 17, PinA18 PA18 18, PinA19 PA19 19,
        PinA20 PA20 20, PinA21 PA21 21, PinA22 PA22 22, PinA23 PA23 23,
        PinA24 PA24 24, PinA25 PA25 25, PinA26 PA26 26, PinA27 PA27 27,
        PinA28 PA28 28, PinA29 PA29 29, PinA30 PA30 30, PinA31 PA31 31],

    PB [PinB00 PB00 32, PinB01 PB01 33, PinB02 PB02 34, PinB03 PB03 35,
        PinB04 PB04 36, PinB05 PB05 37, PinB06 PB06 38, PinB07 PB07 39,
        PinB08 PB08 40, PinB09 PB09 41, PinB10 PB10 42, PinB11 PB11 43,
        PinB12 PB12 44, PinB13 PB13 45, PinB14 PB14 46, PinB15 PB15 47,
        PinB16 PB16 48, PinB17 PB17 49, PinB18 PB18 50, PinB19 PB19 51,
        PinB20 PB20 52, PinB21 PB21 53, PinB22 PB22 54, PinB23 PB23 55,
        PinB24 PB24 56, PinB25 PB25 57, PinB26 PB26 58, PinB27 PB27 59,
        PinB28 PB28 60, PinB29 PB29 61, PinB30 PB30 62, PinB31 PB31 63],

    PC [PinC00 PC00 64, PinC01 PC01 65, PinC02 PC02 66, PinC03 PC03 67,
        PinC04 PC04 68, PinC05 PC05 69, PinC06 PC06 70, PinC07 PC07 71,
        PinC08 PC08 72, PinC09 PC09 73, PinC10 PC10 74, PinC11 PC11 75,
        PinC12 PC12 76, PinC13 PC13 77, PinC14 PC14 78, PinC15 PC15 79,
        PinC16 PC16 80, PinC17 PC17 81, PinC18 PC18 82, PinC19 PC19 83,
        PinC20 PC20 84, PinC21 PC21 85, PinC22 PC22 86, PinC23 PC23 87,
        PinC24 PC24 88, PinC25 PC25 89, PinC26 PC26 90, PinC27 PC27 91,
        PinC28 PC28 92, PinC29 PC29 93, PinC30 PC30 94, PinC31 PC31 95],

    PD [PinD00 PD00 96, PinD01 PD01 97, PinD02 PD02 98, PinD03 PD03 99,
        PinD04 PD04 100, PinD05 PD05 101, PinD06 PD06 102, PinD07 PD07 103,
        PinD08 PD08 104, PinD09 PD09 105, PinD10 PD10 106, PinD11 PD11 107,
        PinD12 PD12 108, PinD13 PD13 109, PinD14 PD14 110, PinD15 PD15 111,
        PinD16 PD16 112, PinD17 PD17 113, PinD18 PD18 114, PinD19 PD19 115,
        PinD20 PD20 116, PinD21 PD21 117, PinD22 PD22 118, PinD23 PD23 119,
        PinD24 PD24 120, PinD25 PD25 121, PinD26 PD26 122, PinD27 PD27 123,
        PinD28 PD28 124, PinD29 PD29 125, PinD30 PD30 126, PinD31 PD31 127],

    PE [PinE00 PE00 128, PinE01 PE01 129, PinE02 PE02 130, PinE03 PE03 131,
        PinE04 PE04 132, PinE05 PE05 133, PinE06 PE06 134, PinE07 PE07 135,
        PinE08 PE08 136, PinE09 PE09 137, PinE10 PE10 138, PinE11 PE11 139,
        PinE12 PE12 140, PinE13 PE13 141, PinE14 PE14 142, PinE15 PE15 143,
        PinE16 PE16 144, PinE17 PE17 145, PinE18 PE18 146, PinE19 PE19 147,
        PinE20 PE20 148, PinE21 PE21 149, PinE22 PE22 150, PinE23 PE23 151,
        PinE24 PE24 152, PinE25 PE25 153, PinE26 PE26 154, PinE27 PE27 155,
        PinE28 PE28 156, PinE29 PE29 157, PinE30 PE30 158, PinE31 PE31 159]
}

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

    // SPI0
    pub const SPI0_MOSI: Function<PinC06> = Function::new(Alt2);
    pub const SPI0_MISO: Function<PinC07> = Function::new(Alt2);
    pub const SPI0_SCK: Function<PinA15> = Function::new(Alt2);
    pub const SPI0_CS0: Function<PinC04> = Function::new(Alt2);

    // SPI1
    pub const SPI1_MOSI: Function<PinD06> = Function::new(Alt7);
    pub const SPI1_SCK: Function<PinD05> = Function::new(Alt7);

    // The physical i2c ports
    // In most cases there is more than one bus per i2c
    // controller. Which are used is selected on a per-board
    // basis using pinmux settings.

    // Through the Magic of Schematics, on the schematics all these
    // pins have the same name: SDA0, SDA1. That's not as easy
    // when using identifiers. Here are the pin pairs for I2Cx.
    // These are defined in such a way that I2Cx_SDA0 is the one
    // most likely to be used. What I'd most like to do be able
    // to create a new symbol that matches the schematic, e.g.
    // I2C_SDA0, in the board file, which is where we define
    // the one we use.
    pub const I2C0_SDA0: Function<PinB03> = Function::new(Alt2);
    pub const I2C0_SCLK0: Function<PinB02> = Function::new(Alt2);
    pub const I2C0_SDA1: Function<PinB01> = Function::new(Alt2);
    pub const I2C0_SCLK1: Function<PinB00> = Function::new(Alt2);
    pub const I2C0_SDA2: Function<PinD03> = Function::new(Alt7);
    pub const I2C0_SCLK2: Function<PinD02> = Function::new(Alt7);
    pub const I2C0_SDA3: Function<PinD09> = Function::new(Alt2);
    pub const I2C0_SCLK3: Function<PinD08> = Function::new(Alt2);
    pub const I2C0_SDA4: Function<PinE25> = Function::new(Alt5);
    pub const I2C0_SCLK4: Function<PinE25> = Function::new(Alt5);

    // I2C1
    pub const I2C1_SDA0: Function<PinC11> = Function::new(Alt2);
    pub const I2C1_SCLK0: Function<PinC10> = Function::new(Alt2);
    pub const I2C1_SDA1: Function<PinE00> = Function::new(Alt6);
    pub const I2C1_SCLK1: Function<PinE01> = Function::new(Alt6);

    // I2C2
    // This one is right off the chip docs but seems wrong to me.
    pub const I2C2_SDA0: Function<PinA13> = Function::new(Alt5);
    pub const I2C2_SCLK0: Function<PinA14> = Function::new(Alt5);
    pub const I2C2_SDA1: Function<PinA11> = Function::new(Alt5);
    pub const I2C2_SCLK1: Function<PinA12> = Function::new(Alt5);

    // I2C3
    pub const I2C3_SDA0: Function<PinE10> = Function::new(Alt2);
    pub const I2C3_SCLK0: Function<PinE11> = Function::new(Alt2);
    pub const I2C3_SDA1: Function<PinA01> = Function::new(Alt4);
    pub const I2C3_SCLK1: Function<PinA02> = Function::new(Alt4);
}
