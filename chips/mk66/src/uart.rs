//! Implementation of the MK66 UART Peripheral

use core::cell::Cell;
use kernel::hil;
use kernel::hil::uart;
use core::mem;
use nvic;
use regs::uart::*;

pub struct UART {
    registers: *mut Registers,
    client: Cell<Option<&'static uart::Client>>,
}

pub static mut UART0: UART = UART::new(UART_BASE_ADDRS[0]);
pub static mut UART1: UART = UART::new(UART_BASE_ADDRS[1]);
pub static mut UART2: UART = UART::new(UART_BASE_ADDRS[2]);
pub static mut UART3: UART = UART::new(UART_BASE_ADDRS[3]);
pub static mut UART4: UART = UART::new(UART_BASE_ADDRS[4]);

pub const UART_MODULE_CLOCK: u32 = 10_200_000;

impl UART {
    pub const fn new(base_addr: *mut Registers) -> UART {
        UART {
            registers: base_addr,
            client: Cell::new(None),
        }
    }

    pub fn handle_interrupt(&self) {
        // TODO: implement
    }

    pub fn handle_error(&self) {
        // TODO: implement
    }
}


/// Implementation of kernel::hil::UART
impl hil::uart::UART for UART {
    fn set_client(&self, client: &'static hil::uart::Client) {
        self.client.set(Some(client));
    }

    fn init(&self, params: uart::UARTParams) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        let parity = match params.parity {
            hil::uart::Parity::None => (C1::PE::False, C1::PT::Even),
            hil::uart::Parity::Even => (C1::PE::True, C1::PT::Even),
            hil::uart::Parity::Odd => (C1::PE::True, C1::PT::Odd)
        };

        let stop_bits = match params.stop_bits {
            hil::uart::StopBits::One => BDH::SBNS::One, 
            hil::uart::StopBits::Two => BDH::SBNS::Two
        };

        // Baud rate generation.
        let baud_rate: u32 = (UART_MODULE_CLOCK >> 4) / params.baud_rate;

        // This basic procedure outlined in section 59.9.3.
        // Set control register 1, which configures the parity.
        regs.c1.write(parity.0 + 
                      parity.1 + 
                      C1::LOOPS::False + 
                      C1::UARTSWAI::False +
                      C1::RSRC::False +
                      C1::M::EightBit +
                      C1::WAKE::Idle +
                      C1::ILT::AfterStart);

        // Set the baud rate and stop bits.
        regs.bdh.modify(stop_bits +
                        BDH::SBR.val((baud_rate >> 8) as u8));
        
        regs.bdl.set(baud_rate as u8);

        // Enable the transmitter and receiver.
        regs.c2.write(C2::RE::True + C2::TE::True);
    }

    #[allow(unused_variables)]
    fn transmit(&self, tx_data: &'static mut [u8], tx_len: usize) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        // This basic procedure outlined in section 59.9.3.
        for i in 0..tx_len {
            while !regs.s1.is_set(S1::TRDE) {}
            regs.d.set(tx_data[i]);
        }
    }

    #[allow(unused_variables)]
    fn receive(&self, rx_buffer: &'static mut [u8], rx_len: usize) {
        // TODO: implement
    }
}

interrupt_handler!(uart0_handler, UART0);
interrupt_handler!(uart1_handler, UART1);
interrupt_handler!(uart2_handler, UART2);
interrupt_handler!(uart3_handler, UART3);
interrupt_handler!(uart4_handler, UART4);
interrupt_handler!(uart0_err_handler, UART0);
interrupt_handler!(uart1_err_handler, UART1);
interrupt_handler!(uart2_err_handler, UART2);
interrupt_handler!(uart3_err_handler, UART3);
interrupt_handler!(uart4_err_handler, UART4);
