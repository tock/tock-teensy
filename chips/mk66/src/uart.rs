//! Implementation of the MK66 UART Peripheral

use core::cell::Cell;
use kernel::common::cells::OptionalCell;
use kernel::common::cells::TakeCell;
use kernel::hil;
use kernel::hil::uart;
use kernel::ReturnCode;

use core::mem;
use nvic;
use clock;

use kernel::common::registers::{register_bitfields, ReadWrite, ReadOnly};

#[repr(C)]
pub struct Registers {
    pub bdh: ReadWrite<u8, BaudRateHigh::Register>,
    pub bdl: ReadWrite<u8>,
    pub c1: ReadWrite<u8, Control1::Register>,
    pub c2: ReadWrite<u8, Control2::Register>,
    pub s1: ReadOnly<u8, Status1::Register>,
    pub s2: ReadWrite<u8, Status2::Register>,
    pub c3: ReadWrite<u8, Control3::Register>,
    pub d: ReadWrite<u8>,
    pub ma1: ReadWrite<u8>,
    pub ma2: ReadWrite<u8>,
    pub c4: ReadWrite<u8, Control4::Register>,
    pub c5: ReadWrite<u8, Control5::Register>,
    pub ed: ReadOnly<u8>,
    pub modem: ReadWrite<u8>,
    pub ir: ReadWrite<u8>, // 0x0E
    _reserved0: ReadWrite<u8>,
    pub pfifo: ReadWrite<u8>, // 0x10
    pub cfifo: ReadWrite<u8>,
    pub sfifo: ReadWrite<u8>,
    pub twfifo: ReadWrite<u8>,
    pub tcfifo: ReadOnly<u8>,
    pub rwfifo: ReadWrite<u8>,
    pub rcfifo: ReadOnly<u8>, // 0x16
    _reserved1: ReadWrite<u8>,
    pub c7816: ReadWrite<u8>, // 0x18
    pub ie7816: ReadWrite<u8>,
    pub is7816: ReadWrite<u8>,
    pub wp7816: ReadWrite<u8>,
    pub wn7816: ReadWrite<u8>,
    pub wf7816: ReadWrite<u8>,
    pub et7816: ReadWrite<u8>,
    pub tl7816: ReadWrite<u8>, // 0x1F
    _reserved2: [ReadWrite<u8>; 26],
    pub ap7816a_t0: ReadWrite<u8>, // 0x3A
    pub ap7816b_t0: ReadWrite<u8>,
    pub wp7816a_t0_t1: ReadWrite<u8>,
    pub wp7816b_t0_t1: ReadWrite<u8>,
    pub wgp7816_t1: ReadWrite<u8>,
    pub wp7816c_t1: ReadWrite<u8>,
}


pub const UART_BASE_ADDRS: [*mut Registers; 5] = [0x4006A000 as *mut Registers,
                                                  0x4006B000 as *mut Registers,
                                                  0x4006C000 as *mut Registers,
                                                  0x4006D000 as *mut Registers,
                                                  0x400EA000 as *mut Registers];

register_bitfields! {u8,
    BaudRateHigh [
        LBKDIE OFFSET(7) NUMBITS(1) [],
        RXEDGIE OFFSET(6) NUMBITS(1) [],
        SBNS OFFSET(5) NUMBITS(1) [
            One = 0,
            Two = 1
        ],
        SBR OFFSET(0) NUMBITS(5) []
    ],
    Control1 [
        LOOPS OFFSET(7) NUMBITS(1) [],
        UARTSWAI OFFSET(6) NUMBITS(1) [],
        RSRC OFFSET(5) NUMBITS(1) [],
        M OFFSET(4) NUMBITS(1) [
            EightBit = 0,
            NineBit = 1
        ],
        WAKE OFFSET(3) NUMBITS(1) [
            Idle = 0,
            AddressMark = 1
        ],
        ILT OFFSET(2) NUMBITS(1) [
            AfterStart = 0,
            AfterStop = 1
        ],
        PE OFFSET(1) NUMBITS(1) [],
        PT OFFSET(0) NUMBITS(1) [
            Even = 0,
            Odd = 1
        ]
    ],
    Control2 [
        TIE 7,
        TCIE 6,
        RIE 5,
        ILIE 4,
        TE 3,
        RE 2,
        RWU 1,
        SBK 0
    ],
    Status1 [
        TRDE 7,
        TC 6,
        RDRF 5,
        IDLE 4,
        OR 3,
        NF 2,
        FE 1,
        PF 0
    ],
    Status2 [
        LBKDIF 7,
        RXEDGIF 6,
        MSBF 5,
        RXINV 4,
        RWUID 3,
        BRK13 2,
        LBKDE 1,
        RAF 0
    ],
    Control3 [
        R8 7,
        T8 6,
        TXDIR 5,
        TXINV 4,
        ORIE 3,
        NEIE 2,
        FEIE 1,
        PEIE 0
    ],
    Control4 [
        MAEN1 OFFSET(7) NUMBITS(1) [],
        MAEN2 OFFSET(6) NUMBITS(1) [],
        M10 OFFSET(5) NUMBITS(1) [],
        BRFA OFFSET(0) NUMBITS(5) []
    ],
    Control5 [
        TDMAS 7,
        RDMAS 5
    ]
}


pub struct Uart<'a> {
    index: usize,
    registers: *mut Registers,
    receive_client: OptionalCell<&'a dyn uart::ReceiveClient>,
    transmit_client: OptionalCell<&'a dyn uart::TransmitClient>,
    rx_buffer: TakeCell<'static, [u8]>,
    rx_len: Cell<usize>,
    rx_index: Cell<usize>
}

pub static mut UART0: Uart = Uart::new(0);
pub static mut UART1: Uart = Uart::new(1);
pub static mut UART2: Uart = Uart::new(2);
pub static mut UART3: Uart = Uart::new(3);
pub static mut UART4: Uart = Uart::new(4);

impl<'a> Uart<'a> {
    const fn new(index: usize) -> Uart<'a> {
        Uart {
            index: index,
            registers: UART_BASE_ADDRS[index],
            receive_client: OptionalCell::empty(),
            transmit_client: OptionalCell::empty(),
            rx_buffer: TakeCell::empty(),
            rx_len: Cell::new(0),
            rx_index: Cell::new(0),
        }
    }

    pub fn handle_interrupt(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };
        // Read byte from data register; reading S1 and D clears interrupt
        if regs.s1.is_set(Status1::RDRF) {
            let datum: u8 = regs.d.get();

            // Put byte into buffer, trigger callback if buffer full
            let mut done = false;
            let mut index = self.rx_index.get();
            self.rx_buffer.map( |buf| {
                buf[index] = datum;
                index = index + 1;
                if index >= self.rx_len.get() {
                    done = true;
                }
                self.rx_index.set(index);
            });
            if done {
                self.receive_client.map(|client| {
                    match self.rx_buffer.take() {
                        Some(buf) => client.received_buffer(buf, index, ReturnCode::SUCCESS, uart::Error::None),
                        None => ()
                    }
                });
            }
        }
    }

    pub fn handle_error(&self) {
        // TODO: implement
    }

    fn set_parity(&self, parity: hil::uart::Parity) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        let (pe, pt) = match parity {
            hil::uart::Parity::None => (Control1::PE::CLEAR, Control1::PT::Even),
            hil::uart::Parity::Even => (Control1::PE::SET, Control1::PT::Even),
            hil::uart::Parity::Odd => (Control1::PE::SET, Control1::PT::Odd)
        };

        // This basic procedure outlined in section 59.9.3.
        // Set control register 1, which configures the parity.
        regs.c1.write(pe + pt +
                      Control1::LOOPS::CLEAR +
                      Control1::UARTSWAI::CLEAR +
                      Control1::RSRC::CLEAR +
                      Control1::M::EightBit +
                      Control1::WAKE::Idle +
                      Control1::ILT::AfterStop);
    }

    fn set_stop_bits(&self, stop_bits: hil::uart::StopBits) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        let stop_bits = match stop_bits {
            hil::uart::StopBits::One => BaudRateHigh::SBNS::One,
            hil::uart::StopBits::Two => BaudRateHigh::SBNS::Two
        };

        regs.bdh.modify(stop_bits);
    }

    fn set_baud_rate(&self, baud_rate: u32) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        // Baud rate generation. Note that UART0 and UART1 are sourced from the core clock, not the
        // bus clock.
        let uart_clock = match self.index {
            0 | 1 => clock::core_clock_hz(),
            _ => clock::peripheral_clock_hz()
        };

        let baud_counter: u32 = (uart_clock >> 4) / baud_rate;

        // Set the baud rate.
        regs.c4.modify(Control4::BRFA.val(0));
        regs.bdh.modify(BaudRateHigh::SBR.val((baud_counter >> 8) as u8));
        regs.bdl.set(baud_counter as u8);
    }

    pub fn enable_rx(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };
        regs.c1.modify(Control1::ILT::SET); // Idle after stop bit
        regs.c2.modify(Control2::RE::SET);  // Enable UART reception
    }

    pub fn enable_rx_interrupts(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };
        regs.rwfifo.set(1);               // Issue interrupt on each byte
        regs.c5.modify(Control5::RDMAS::CLEAR); // Issue interrupt on RX data

        match self.index {
            0 => unsafe {nvic::enable(nvic::NvicIdx::UART0)},
            1 => unsafe {nvic::enable(nvic::NvicIdx::UART1)},
            2 => unsafe {nvic::enable(nvic::NvicIdx::UART2)},
            3 => unsafe {nvic::enable(nvic::NvicIdx::UART3)},
            4 => unsafe {nvic::enable(nvic::NvicIdx::UART4)},
            _ => unreachable!()
        };
        regs.c2.modify(Control2::RIE::SET);     // Enable interrupts
    }

    pub fn enable_tx(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };
        regs.c2.modify(Control2::TE::SET);
    }

    fn enable_clock(&self) {
        use sim::{clocks, Clock};
        match self.index {
            0 => clocks::UART0.enable(),
            1 => clocks::UART1.enable(),
            2 => clocks::UART2.enable(),
            3 => clocks::UART3.enable(),
            4 => clocks::UART4.enable(),
            _ => unreachable!()
        };
    }

    pub fn send_byte(&self, byte: u8) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        while !regs.s1.is_set(Status1::TRDE) {}
        regs.d.set(byte);
    }

    pub fn tx_ready(&self) -> bool {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };
        regs.s1.is_set(Status1::TC)
    }
}


/// Implementation of kernel::hil::UART
impl<'a> hil::uart::Transmit<'a> for Uart<'a> {
    fn set_transmit_client(&self, client: &'a dyn hil::uart::TransmitClient) {
        self.transmit_client.replace(client);
    }

    fn transmit_buffer(&self, tx_data: &'static mut [u8], tx_len: usize) -> (ReturnCode, Option<&'static mut [u8]>) { 
        // This basic procedure outlined in section 59.9.3.
        for i in 0..tx_len {
            self.send_byte(tx_data[i]);
        }

        while !self.tx_ready() {}

        self.transmit_client.map(move |client|
            client.transmitted_buffer(tx_data, tx_len, ReturnCode::SUCCESS)
        );
        (ReturnCode::SUCCESS, None)
    }

    fn transmit_word(&self, _tx_data: u32) -> ReturnCode {
        ReturnCode::FAIL
    }

    fn transmit_abort(&self) -> ReturnCode {
        // Because Transmit is blocking, this succeeds, there will be
        // no callback.
        ReturnCode::SUCCESS
    }
}

impl<'a> hil::uart::Configure for Uart<'a> {
    fn configure(&self, params: hil::uart::Parameters) -> ReturnCode {
        self.enable_clock();

        self.set_parity(params.parity);
        self.set_stop_bits(params.stop_bits);
        self.set_baud_rate(params.baud_rate);

        self.enable_rx();
        self.enable_rx_interrupts();
        self.enable_tx();
        ReturnCode::SUCCESS
    }
}

impl<'a> hil::uart::Receive<'a> for Uart<'a> {
    fn receive_buffer(&self, rx_buffer: &'static mut [u8], rx_len: usize) -> (ReturnCode, Option<&'static mut [u8]>) {
        let length = rx_len;
        if rx_len > rx_buffer.len() {
            return (ReturnCode::ESIZE, Some(rx_buffer));
        }
        self.rx_buffer.replace(rx_buffer);
        self.rx_len.set(length);
        self.rx_index.set(0);
        (ReturnCode::SUCCESS, None)
    }

    fn receive_abort(&self) -> ReturnCode {
	ReturnCode::FAIL
    }
    
    fn receive_word(&self) -> ReturnCode {
        ReturnCode::FAIL
    }
 
    fn set_receive_client(&self, client: &'a dyn uart::ReceiveClient) {
        self.receive_client.replace(client);
    }
}
