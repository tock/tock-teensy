use kernel::common::regs::{ReadWrite, ReadOnly};

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
