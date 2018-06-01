#![allow(unused)]
//! Implementation of the MK66 I2C

use core::cell::Cell;
use kernel::common::take_cell::TakeCell;
use kernel::hil;
use kernel::hil::i2c;
use core::mem;
//use nvic;
use regs::i2c::*;
//use clock;

pub struct I2C {
    index: usize,
    registers: *mut Registers,
    //client: Cell<Option<&'static i2c::Client>>,
    buffer: TakeCell<'static, [u8]>,
    rx_len: Cell<usize>,
    rx_index: Cell<usize>
}

pub static mut I2C0: I2C = I2C::new(0);
pub static mut I2C1: I2C = I2C::new(1);
pub static mut I2C2: I2C = I2C::new(2);
pub static mut I2C3: I2C = I2C::new(3);


impl I2C {
    pub const fn new(index: usize) -> I2C {
        I2C {
            index: index,
            registers: I2C_BASE_ADDRS[index],
            //client: Cell::new(None),
            buffer: TakeCell::empty(),
            rx_len: Cell::new(0),
            rx_index: Cell::new(0),
        }
    }

    pub fn handle_interrupt(&self) {
        let _regs: &mut Registers = unsafe { mem::transmute(self.registers) };
        // Read byte from data register; reading S1 and D clears interrupt
    }

    pub fn handle_error(&self) {
        // TODO: implement
    }

    fn enable_clock(&self) {
        use sim::{clocks, Clock};
        match self.index {
            0 => clocks::I2C0.enable(),
            1 => clocks::I2C1.enable(),
            2 => clocks::I2C2.enable(),
            3 => clocks::I2C3.enable(),
            _ => unreachable!()
        };
    }

}


/// Implementation of kernel::hil::I2C
impl hil::i2c::I2CMaster for I2C {
    fn enable(&self){
    }
    fn disable(&self){
    }
    fn write_read(&self, addr: u8, data: &'static mut [u8], write_len: u8, read_len: u8){
    }
    fn write(&self, addr: u8, data: &'static mut [u8], len: u8){
    }
    fn read(&self, addr: u8, buffer: &'static mut [u8], len: u8){
    }

}
/*
interrupt_handler!(i2c0_handler, I2C0);
interrupt_handler!(i2c1_handler, I2C1);
interrupt_handler!(i2c2_handler, I2C2);
interrupt_handler!(i2c3_handler, I2C3);
interrupt_handler!(i2c0_err_handler, I2C0);
interrupt_handler!(i2c1_err_handler, I2C1);
interrupt_handler!(i2c2_err_handler, I2C2);
interrupt_handler!(i2c3_err_handler, I2C3);
*/
