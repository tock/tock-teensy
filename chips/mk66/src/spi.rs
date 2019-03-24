use kernel::hil::spi::*;
use kernel::ReturnCode;
use kernel::common::cells::TakeCell;
use core::cell::Cell;
use core::mem;
use clock;
use nvic::{self, NvicIdx};

use kernel::common::registers::{register_bitfields, ReadWrite, ReadOnly};
  
#[repr(C)]
pub struct Registers {
    pub mcr: ReadWrite<u32, ModuleConfiguration::Register>,
    _reserved0: ReadOnly<u32>,
    pub tcr: ReadWrite<u32, TransferCount::Register>,
    pub ctar0: ReadWrite<u32, ClockAndTransferAttributes::Register>,
    pub ctar1: ReadWrite<u32, ClockAndTransferAttributes::Register>,
    _reserved1: [ReadOnly<u32>; 6],
    pub sr: ReadWrite<u32, Status::Register>,
    pub rser: ReadWrite<u32, RequestSelectAndEnable::Register>,
    pub pushr_data: ReadWrite<u8>,
    _reserved2: ReadWrite<u8>,
    pub pushr_cmd: ReadWrite<u16, TxFifoPushCommand::Register>,
    pub popr: ReadOnly<u32>,
    pub txfifo: [ReadOnly<u32>; 4],
    _reserved3: [ReadOnly<u32>; 12],
    pub rxfifo: [ReadOnly<u32>; 4]
}

pub const SPI_ADDRS: [*mut Registers; 3] = [0x4002_C000 as *mut Registers,
                                            0x4002_D000 as *mut Registers,
                                            0x400A_C000 as *mut Registers];

register_bitfields![ u32,
    ModuleConfiguration [
        MSTR OFFSET(31) NUMBITS(1) [
            Master = 1,
            Slave = 0
        ],
        CONT_SCKE OFFSET(30) NUMBITS(1) [],
        FRZ OFFSET(27) NUMBITS(1) [],
        MTFE OFFSET(26) NUMBITS(1) [],
        PCSSE OFFSET(25) NUMBITS(1) [
            PeripheralChipSelect = 0,
            ActiveLowStrobe = 1
        ],
        ROOE OFFSET(24) NUMBITS(1) [
            IgnoreOverflow = 0,
            ShiftOverflow = 1
        ],
        PCSIS OFFSET(16) NUMBITS(6) [
            AllInactiveHigh = 0x3F,
            AllInactiveLow = 0x0
        ],
        DOZE OFFSET(15) NUMBITS(1) [],
        MDIS OFFSET(14) NUMBITS(1) [],
        DIS_TXF OFFSET(13) NUMBITS(1) [],
        DIS_RXF OFFSET(12) NUMBITS(1) [],
        CLR_TXF OFFSET(11) NUMBITS(1) [],
        CLR_RXF OFFSET(10) NUMBITS(1) [],
        SMPL_PT OFFSET(8) NUMBITS(2) [
            ZeroCycles = 0,
            OneCycle = 1,
            TwoCycles = 2
        ],
        HALT OFFSET(0) NUMBITS(1) []
    ],

    TransferCount [
        SPI_TCNT OFFSET(16) NUMBITS(16) []
    ],

    ClockAndTransferAttributes [
        DBR OFFSET(31) NUMBITS(1) [],
        FMSZ OFFSET(27) NUMBITS(4) [],
        CPOL OFFSET(26) NUMBITS(1) [
            IdleLow = 0,
            IdleHigh = 1
        ],
        CPHA OFFSET(25) NUMBITS(1) [
            SampleLeading = 0,
            SampleTrailing = 1
        ],
        LSBFE OFFSET(24) NUMBITS(1) [
            MsbFirst = 0,
            LsbFirst = 1
        ],
        PCSSCK OFFSET(22) NUMBITS(2) [
            Prescaler1 = 0,
            Prescaler3 = 1,
            Prescaler5 = 2,
            Prescaler7 = 3
        ],
        PASC OFFSET(20) NUMBITS(2) [
            Delay1 = 0,
            Delay3 = 1,
            Delay5 = 2,
            Delay7 = 3
        ],
        PDT OFFSET(18) NUMBITS(2) [
            Delay1 = 0,
            Delay3 = 1,
            Delay5 = 2,
            Delay7 = 3
        ],
        PBR OFFSET(16) NUMBITS(2) [
            BaudRatePrescaler2 = 0,
            BaudRatePrescaler3 = 1,
            BaudRatePrescaler5 = 2,
            BaudRatePrescaler7 = 3
        ],
        CSSCK OFFSET(12) NUMBITS(4) [
            DelayScaler2 = 0x0,
            DelayScaler4 = 0x1,
            DelayScaler8 = 0x2,
            DelayScaler16 = 0x3,
            DelayScaler32 = 0x4,
            DelayScaler64 = 0x5,
            DelayScaler128 = 0x6,
            DelayScaler256 = 0x7,
            DelayScaler512 = 0x8,
            DelayScaler1024 = 0x9,
            DelayScaler2048 = 0xA,
            DelayScaler4096 = 0xB,
            DelayScaler8192 = 0xC,
            DelayScaler16384 = 0xD,
            DelayScaler32768 = 0xE,
            DelayScaler65536 = 0xF
        ],
        ASC OFFSET(8) NUMBITS(4) [],
        DT OFFSET(4) NUMBITS(4) [],
        BR OFFSET(0) NUMBITS(4) [
            BaudRateScaler2 = 0x0,
            BaudRateScaler4 = 0x1,
            BaudRateScaler8 = 0x2,
            BaudRateScaler16 = 0x3,
            BaudRateScaler32 = 0x4,
            BaudRateScaler64 = 0x5,
            BaudRateScaler128 = 0x6,
            BaudRateScaler256 = 0x7,
            BaudRateScaler512 = 0x8,
            BaudRateScaler1024 = 0x9,
            BaudRateScaler2048 = 0xA,
            BaudRateScaler4096 = 0xB,
            BaudRateScaler8192 = 0xC,
            BaudRateScaler16384 = 0xD,
            BaudRateScaler32768 = 0xE,
            BaudRateScaler65536 = 0xF
        ]
    ],

    ClockAndTransferAttributesSlave [
        FMSZ OFFSET(27) NUMBITS(4) [],
        CPOL OFFSET(26) NUMBITS(1) [
            IdleLow = 0,
            IdleHigh = 1
        ],
        CPHA OFFSET(25) NUMBITS(1) [
            SampleLeading = 0,
            SampleTrailing = 1
        ]
    ],

    Status [
        TCF OFFSET(31) NUMBITS(1) [],
        TXRS OFFSET(30) NUMBITS(1) [],
        EOQF OFFSET(28) NUMBITS(1) [],
        TFUF OFFSET(27) NUMBITS(1) [],
        TFFF OFFSET(25) NUMBITS(1) [],
        RFOF OFFSET(19) NUMBITS(1) [],
        RFDF OFFSET(17) NUMBITS(1) [],
        TXCTR OFFSET(12) NUMBITS(4) [],
        TXNXTPTR OFFSET(8) NUMBITS(4) [],
        RXCTR OFFSET(4) NUMBITS(4) [],
        POPNXTPTR OFFSET(0) NUMBITS(4) []
    ],

    RequestSelectAndEnable [
        TCF_RE OFFSET(31) NUMBITS(1) [],
        EOQF_RE OFFSET(28) NUMBITS(1) [],
        TFUF_RE OFFSET(27) NUMBITS(1) [],
        TFFF_RE OFFSET(25) NUMBITS(1) [],
        TFFF_DIRS OFFSET(24) NUMBITS(1) [
            Interrupt = 0,
            Dma = 1
        ],
        RFOF_RE OFFSET(19) NUMBITS(1) [],
        RFDF_RE OFFSET(17) NUMBITS(1) [],
        RFDF_DIRS OFFSET(16) NUMBITS(1) [
            Interrupt = 0,
            Dma = 1
        ]
    ]
];

register_bitfields![u16,
    TxFifoPushCommand [
        CONT OFFSET(15) NUMBITS(1) [
            ChipSelectInactiveBetweenTxfers = 0,
            ChipSelectAssertedBetweenTxfers = 1
        ],
        CTAS OFFSET(12) NUMBITS(3) [
            Ctar0 = 0,
            Ctar1 = 1
        ],
        EOQ OFFSET(11) NUMBITS(1) [],
        CTCNT OFFSET(10) NUMBITS(1) [],
        PCS OFFSET(0) NUMBITS(6) []
    ]
];

pub enum SpiRole {
    Master,
    Slave
}

pub struct Spi<'a> {
    regs: *mut Registers,
    client: Cell<Option<&'a SpiMasterClient>>,
    index: usize,
    chip_select_settings: [Cell<u32>; 6],
    write: TakeCell<'static, [u8]>,
    read: TakeCell<'static, [u8]>,
    transfer_len: Cell<usize>,
}

pub static mut SPI0: Spi<'static> = Spi::new(0);
pub static mut SPI1: Spi<'static> = Spi::new(1);
pub static mut SPI2: Spi<'static> = Spi::new(2);

impl<'a> Spi<'a> {
    pub const fn new(index: usize) -> Spi<'a> {
        Spi {
            regs: SPI_ADDRS[index],
            client: Cell::new(None),
            index: index,
            chip_select_settings: [Cell::new(0),
                                   Cell::new(0),
                                   Cell::new(0),
                                   Cell::new(0),
                                   Cell::new(0),
                                   Cell::new(0)],
            write: TakeCell::empty(),
            read: TakeCell::empty(),
            transfer_len: Cell::new(0),
        }
    }

    fn regs(&self) -> &mut Registers {
        unsafe { mem::transmute(self.regs) }
    }

    pub fn enable(&self) {
        self.regs().mcr.modify(ModuleConfiguration::MDIS::CLEAR);
    }

    pub fn disable(&self) {
        self.regs().mcr.modify(ModuleConfiguration::MDIS::SET);
    }

    pub fn is_running(&self) -> bool {
        self.regs().sr.is_set(Status::TXRS)
    }

    pub fn halt(&self) {
        self.regs().mcr.modify(ModuleConfiguration::HALT::SET);
        while self.is_running() {}
    }

    pub fn resume(&self) {
        self.regs().mcr.modify(ModuleConfiguration::HALT::CLEAR);
    }

    fn enable_clock(&self) {
        use sim::{clocks, Clock};
        match self.index {
            0 => clocks::SPI0.enable(),
            1 => clocks::SPI1.enable(),
            2 => clocks::SPI2.enable(),
            _ => unreachable!()
        };
    }

    fn set_client(&self, client: &'a SpiMasterClient) {
        self.client.set(Some(client));
    }

    fn set_role(&self, role: SpiRole) {
        self.halt();
        match role {
            SpiRole::Master => {
                self.regs().mcr.modify(ModuleConfiguration::MSTR::Master);
            },
            SpiRole::Slave => {
                self.regs().mcr.modify(ModuleConfiguration::MSTR::Slave);
            }
        }
        self.resume();
    }

    fn set_polarity(&self, polarity: ClockPolarity) {
        let cpol = match polarity {
            ClockPolarity::IdleHigh => ClockAndTransferAttributes::CPOL::IdleHigh,
            ClockPolarity::IdleLow => ClockAndTransferAttributes::CPOL::IdleLow
        };
        self.halt();
        self.regs().ctar0.modify(cpol);
        self.resume();
    }

    fn get_polarity(&self) -> ClockPolarity {
        if self.regs().ctar0.matches_all(ClockAndTransferAttributes::CPOL::IdleHigh) {
            ClockPolarity::IdleHigh
        } else {
            ClockPolarity::IdleLow
        }
    }

    fn set_phase(&self, phase: ClockPhase) {
        let cpha = match phase {
            ClockPhase::SampleLeading => ClockAndTransferAttributes::CPHA::SampleLeading,
            ClockPhase::SampleTrailing => ClockAndTransferAttributes::CPHA::SampleTrailing
        };
        self.halt();
        self.regs().ctar0.modify(cpha);
        self.resume();
    }

    fn get_phase(&self) -> ClockPhase {
        if self.regs().ctar0.matches_all(ClockAndTransferAttributes::CPHA::SampleLeading) {
            ClockPhase::SampleLeading
        } else {
            ClockPhase::SampleTrailing
        }
    }

    pub fn set_data_order(&self, order: DataOrder) {
        let order = match order {
            DataOrder::LSBFirst => ClockAndTransferAttributes::LSBFE::LsbFirst,
            DataOrder::MSBFirst => ClockAndTransferAttributes::LSBFE::MsbFirst
        };
        self.halt();
        self.regs().ctar0.modify(order);
        self.resume();
    }

    pub fn get_data_order(&self) -> DataOrder {
        if self.regs().ctar0.matches_all(ClockAndTransferAttributes::LSBFE::LsbFirst) {
            DataOrder::LSBFirst
        } else {
            DataOrder::MSBFirst
        }
    }

    fn fifo_depth(&self) -> u32 {
        // SPI0 has a FIFO with 4 entries, all others have a 1 entry "FIFO".
        match self.index {
            0 => 4,
            1 | 2 => 1,
            _ => unreachable!()
        }
    }

    fn num_chip_selects(&self) -> u32 {
        match self.index {
            0 => 6,
            1 => 4,
            2 => 2,
            _ => unreachable!()
        }
    }

    fn flush_tx_fifo(&self) {
        self.halt();
        self.regs().mcr.modify(ModuleConfiguration::CLR_TXF::SET);
        self.resume();
    }

    fn flush_rx_fifo(&self) {
        self.halt();
        self.regs().mcr.modify(ModuleConfiguration::CLR_RXF::SET);
        self.resume();
    }

    fn tx_fifo_ready(&self) -> bool {
        !(self.regs().sr.read(Status::TXCTR) >= self.fifo_depth())
    }

    fn rx_fifo_ready(&self) -> bool {
        self.regs().sr.read(Status::RXCTR) > 0
    }

    fn baud_rate(dbl: u32, prescaler: u32, scaler: u32) -> u32 {
        (clock::bus_clock_hz() * (1 + dbl)) / (prescaler * scaler)
    }

    fn set_baud_rate(&self, rate: u32) -> u32 {
        let prescalers: [u32; 4] = [ 2, 3, 5, 7 ];
        let scalers: [u32; 16] = [2, 4, 6, 8,
                                  1<<4, 1<<5, 1<<6, 1<<7,
                                  1<<8, 1<<9, 1<<10, 1<<11,
                                  1<<12, 1<<13, 1<<14, 1<<15];
        let dbls: [u32; 2] = [0, 1];

        let mut rate_diff = rate;
        let mut prescaler = 0;
        let mut scaler = 0;
        let mut dbl = 0;

        // Since there are only 128 unique settings, just iterate over possible
        // configurations until we find the best match. If baud rate can be
        // matched exactly, this loop will terminate early.
        for d in 0..dbls.len() { // 0 is preferred for DBL, as it affects duty cycle
            for p in 0..prescalers.len() {
                for s in 0..scalers.len() {
                    let curr_rate = Spi::baud_rate(dbls[d],
                                                   prescalers[p],
                                                   scalers[s]);

                    // Determine the distance from the target baud rate.
                    let curr_diff = if curr_rate > rate { curr_rate - rate }
                                    else { rate - curr_rate };

                    // If we've improved the best configuration, use it.
                    if curr_diff < rate_diff {
                        rate_diff = curr_diff;
                        scaler = s;
                        prescaler = p;
                        dbl = d;
                    }

                    // Terminate if we've found an exact match.
                    if rate_diff == 0 { break }
                }
            }
        }

        self.halt();
        self.regs().ctar0.modify(ClockAndTransferAttributes::DBR.val(dbl as u32) +
                                 ClockAndTransferAttributes::PBR.val(prescaler as u32) +
                                 ClockAndTransferAttributes::BR.val(scaler as u32));
        self.resume();

        Spi::baud_rate(dbls[dbl], prescalers[prescaler], scalers[scaler])
    }

    fn get_baud_rate(&self) -> u32 {
        let prescaler = match self.regs().ctar0.read(ClockAndTransferAttributes::PBR) {
            0 => 2,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => panic!("Impossible value for baud rate field!")
        };

        let scaler = match self.regs().ctar0.read(ClockAndTransferAttributes::BR) {
            0 => 2,
            1 => 4,
            2 => 6,
            s @ _ => 1 << s
        };

        let dbl = self.regs().ctar0.read(ClockAndTransferAttributes::DBR);

        Spi::baud_rate(dbl, prescaler, scaler)
    }

    pub fn transfer_count(&self) -> u32 {
        self.regs().sr.read(Status::TXCTR)
    }

    pub fn start_of_queue(&self) {
        self.regs().pushr_cmd.modify(TxFifoPushCommand::EOQ::CLEAR);
    }

    fn end_of_queue(&self) {
        self.regs().pushr_cmd.modify(TxFifoPushCommand::EOQ::SET);
    }

    fn configure_timing(&self) {
        self.halt();
        // Set maximum delay after transfer.
        self.regs().ctar0.modify(ClockAndTransferAttributes::DT.val(0x0) + ClockAndTransferAttributes::PDT::Delay7);
        self.resume();
    }

    fn set_frame_size(&self, size: u32) {
        if size > 16 || size < 4 { return }

        self.halt();
        self.regs().ctar0.modify(ClockAndTransferAttributes::FMSZ.val(size - 1));
        self.resume();
    }

    fn enable_interrupt(&self) {
        let idx = match self.index {
            0 => NvicIdx::SPI0,
            1 => NvicIdx::SPI1,
            2 => NvicIdx::SPI2,
            _ => unreachable!()
        };

        self.halt();
        unsafe {
            nvic::enable(idx);
        }
        self.regs().rser.modify(RequestSelectAndEnable::EOQF_RE::SET);
        self.resume();
    }

    pub fn handle_interrupt(&self) {
        // TODO: Determine why the extra interrupt is called

        // End of transfer
        if self.regs().sr.is_set(Status::EOQF) {
            self.regs().sr.modify(Status::EOQF::SET);

            self.client.get().map(|client| {
                match self.write.take() {
                    Some(wbuf) => client.read_write_done(wbuf, self.read.take(), self.transfer_len.get()),
                    None => ()
                };
            });
        }
    }
}

impl<'a> SpiMaster for Spi<'a> {
    type ChipSelect = u32;

    fn set_client(&self, client: &'static SpiMasterClient) {
        Spi::set_client(self, client);
    }

    fn init(&self) {
        // Section 57.6.2
        self.enable_clock();
        self.flush_rx_fifo();
        self.flush_tx_fifo();
        self.set_role(SpiRole::Master);
        self.enable_interrupt();
        self.enable();

        self.set_frame_size(8);
        self.configure_timing();
        self.regs().mcr.modify(ModuleConfiguration::PCSIS::AllInactiveHigh);
        self.regs().pushr_cmd.modify(TxFifoPushCommand::PCS.val(0));
    }

    fn is_busy(&self) -> bool {
        self.is_running()
    }

    /// Perform an asynchronous read/write operation, whose
    /// completion is signaled by invoking SpiMasterClient on
    /// the initialized client. write_buffer must be Some,
    /// read_buffer may be None. If read_buffer is Some, the
    /// length of the operation is the minimum of the size of
    /// the two buffers.
    fn read_write_bytes(&self,
                        write_buffer: &'static mut [u8],
                        read_buffer: Option<&'static mut [u8]>,
                        len: usize)
                        -> ReturnCode {

        self.start_of_queue();
        if let Some(rbuf) = read_buffer {
            for i in 0..len {
                while !self.tx_fifo_ready() {}

                if i == len - 1 {
                    self.end_of_queue();
                }

                self.regs().pushr_data.set(write_buffer[i]);

                // TODO: this is pretty hacky
                while !self.rx_fifo_ready() {}
                rbuf[i] = self.regs().popr.get() as u8;
            }

            self.read.put(Some(rbuf));
        } else {
            for i in 0..len {
                while !self.tx_fifo_ready() {}

                if i == len - 1 {
                    self.end_of_queue();
                }

                self.regs().pushr_data.set(write_buffer[i]);
            }
            self.read.put(None);
        }

        self.write.put(Some(write_buffer));
        self.transfer_len.set(len);

        ReturnCode::SUCCESS
    }

    fn write_byte(&self, _val: u8) {
        unimplemented!();
    }

    fn read_byte(&self) -> u8 {
        unimplemented!();
    }

    fn read_write_byte(&self, _val: u8) -> u8 {
        unimplemented!();
    }

    /// Tell the SPI peripheral what to use as a chip select pin.
    /// The type of the argument is based on what makes sense for the
    /// peripheral when this trait is implemented.
    fn specify_chip_select(&self, cs: Self::ChipSelect) {
        if cs >= self.num_chip_selects() {
            return;
        }

        // The PCS field is one-hot (the way this interface uses it).
        let pcs = self.regs().pushr_cmd.read(TxFifoPushCommand::PCS);
        let old_cs = match pcs {
            0 | 0b000001 => 0,
            0b000010 => 1,
            0b000100 => 2,
            0b001000 => 3,
            0b010000 => 4,
            0b100000 => 5,
            _ => panic!("Unexpected PCS: {:?}", pcs),
        };

        let new_cs = cs as usize;

        // Swap in the new configuration.
        self.halt();
        self.chip_select_settings[old_cs].set(self.regs().ctar0.get());
        self.regs().ctar0.set(self.chip_select_settings[new_cs].get());
        self.resume();
        self.regs().pushr_cmd.modify(TxFifoPushCommand::PCS.val(1 << new_cs));
    }

    /// Returns the actual rate set
    fn set_rate(&self, rate: u32) -> u32 {
        self.set_baud_rate(rate)
    }

    fn get_rate(&self) -> u32 {
        self.get_baud_rate()
    }

    fn set_clock(&self, polarity: ClockPolarity) {
        self.set_polarity(polarity);
    }

    fn get_clock(&self) -> ClockPolarity {
        self.get_polarity()
    }

    fn set_phase(&self, phase: ClockPhase) {
        Spi::set_phase(self, phase);
    }

    fn get_phase(&self) -> ClockPhase {
        Spi::get_phase(self)
    }

    // These two functions determine what happens to the chip
    // select line between transfers. If hold_low() is called,
    // then the chip select line is held low after transfers
    // complete. If release_low() is called, then the chip select
    // line is brought high after a transfer completes. A "transfer"
    // is any of the read/read_write calls. These functions
    // allow an application to manually control when the
    // CS line is high or low, such that it can issue multi-byte
    // requests with single byte operations.
    fn hold_low(&self) {
        self.regs().pushr_cmd.modify(TxFifoPushCommand::CONT::ChipSelectInactiveBetweenTxfers);
    }

    fn release_low(&self) {
        self.regs().pushr_cmd.modify(TxFifoPushCommand::CONT::ChipSelectAssertedBetweenTxfers);
    }
}
