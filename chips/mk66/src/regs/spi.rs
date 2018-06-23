use kernel::common::regs::{ReadWrite, ReadOnly};

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
