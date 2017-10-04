use common::regs::{RW, RO};

#[repr(C, packed)]
pub struct Registers {
    pub mcr: RW<u32, ModuleConfiguration>,
    _reserved0: RO<u32>,
    pub tcr: RW<u32, TransferCount>,
    pub ctar0: RW<u32, ClockAndTransferAttributes>,
    pub ctar1: RW<u32, ClockAndTransferAttributes>,
    _reserved1: [RO<u32>; 6],
    pub sr: RW<u32, Status>,
    pub rser: RW<u32, RequestSelectAndEnable>,
    pub pushr_data: RW<u8>,
    _reserved2: RW<u8>,
    pub pushr_cmd: RW<u16, TxFifoPushCommand>,
    pub popr: RO<u32>,
    pub txfifo: [RO<u32>; 4],
    _reserved3: [RO<u32>; 12],
    pub rxfifo: [RO<u32>; 4]
}

pub const SPI_ADDRS: [*mut Registers; 3] = [0x4002_C000 as *mut Registers,
                                            0x4002_D000 as *mut Registers,
                                            0x400A_C000 as *mut Registers];

bitfields![ u32,
    MCR ModuleConfiguration [
        MSTR 31 [
            Master = 1,
            Slave = 0
        ],
        CONT_SCKE 30 [],
        FRZ 27 [],
        MTFE 26 [],
        PCSSE 25 [
            PeripheralChipSelect = 0,
            ActiveLowStrobe = 1
        ],
        ROOE 24 [
            IgnoreOverflow = 0,
            ShiftOverflow = 1
        ],
        PCSIS (Mask(0b11_1111), 16) [
            AllInactiveHigh = 0x3F,
            AllInactiveLow = 0x0
        ],
        DOZE 15 [],
        MDIS 14 [],
        DIS_TXF 13 [],
        DIS_RXF 12 [],
        CLR_TXF 11 [],
        CLR_RXF 10 [],
        SMPL_PT (Mask(0b11), 8) [
            ZeroCycles = 0,
            OneCycle = 1,
            TwoCycles = 2
        ],
        HALT 0 []
    ],

    TCR TransferCount [
        SPI_TCNT (Mask(0xFFFF), 16) []
    ],

    CTAR ClockAndTransferAttributes [
        DBR 31 [],
        FMSZ (Mask(0xF), 27) [],
        CPOL 26 [
            IdleLow = 0,
            IdleHigh = 1
        ],
        CPHA 25 [
            SampleLeading = 0,
            SampleTrailing = 1
        ],
        LSBFE 24 [
            MsbFirst = 0,
            LsbFirst = 1
        ],
        PCSSCK (Mask(0b11), 22) [
            Prescaler1 = 0,
            Prescaler3 = 1,
            Prescaler5 = 2,
            Prescaler7 = 3
        ],
        PASC (Mask(0b11), 20) [
            Delay1 = 0,
            Delay3 = 1,
            Delay5 = 2,
            Delay7 = 3
        ],
        PDT (Mask(0b11), 18) [
            Delay1 = 0,
            Delay3 = 1,
            Delay5 = 2,
            Delay7 = 3
        ],
        PBR (Mask(0b11), 16) [
            BaudRatePrescaler2 = 0,
            BaudRatePrescaler3 = 1,
            BaudRatePrescaler5 = 2,
            BaudRatePrescaler7 = 3
        ],
        CSSCK (Mask(0b1111), 12) [
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
        ASC (Mask(0b1111), 8) [],
        DT (Mask(0b1111), 4) [],
        BR (Mask(0b1111), 0) [
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

    CTAR_SLAVE ClockAndTransferAttributesSlave [
        FMSZ (Mask(0xF), 27) [],
        CPOL 26 [
            IdleLow = 0,
            IdleHigh = 1
        ],
        CPHA 25 [
            SampleLeading = 0,
            SampleTrailing = 1
        ]
    ],

    SR Status [
        TCF 31 [],
        TXRS 30 [],
        EOQF 28 [],
        TFUF 27 [],
        TFFF 25 [],
        RFOF 19 [],
        RFDF 17 [],
        TXCTR (Mask(0xF), 12) [],
        TXNXTPTR (Mask(0xF), 8) [],
        RXCTR (Mask(0xF), 4) [],
        POPNXTPTR (Mask(0xF), 0) []
    ],

    RSER RequestSelectAndEnable [
        TCF_RE 31 [],
        EOQF_RE 28 [],
        TFUF_RE 27 [],
        TFFF_RE 25 [],
        TFFF_DIRS 24 [
            Interrupt = 0,
            Dma = 1
        ],
        RFOF_RE 19 [],
        RFDF_RE 17 [],
        RFDF_DIRS 16 [
            Interrupt = 0,
            Dma = 1
        ]
    ]
];

bitfields![ u16, 
    PUSHR_CMD TxFifoPushCommand [
        CONT 15 [
            ChipSelectInactiveBetweenTxfers = 0,
            ChipSelectAssertedBetweenTxfers = 1
        ],
        CTAS (Mask(0b111), 12) [
            Ctar0 = 0,
            Ctar1 = 1
        ],
        EOQ 11 [],
        CTCNT 10 [],
        PCS (Mask(0b111111), 0) []
    ]
];
