//! Implementation of the MK66 Random Number Generator (RNGA)

use core::cell::Cell;
use common::regs::{ReadWrite, WriteOnly, ReadOnly};
use kernel::hil::rng::{self, Continue};

#[repr(C, packed)]
struct Registers {
    control: ReadWrite<u8, Control>,
    _unused0: [u8; 3],
    status: ReadOnly<u8, Status>,
    reg_level: ReadOnly<u8>,
    reg_size: ReadOnly<u8>,
    _unused1: u8,
    entropy: WriteOnly<u32>,
    output: ReadOnly<u32>,
}

bitfields! [
    u8,
    CR Control [
        GO 0 [],
        HA 1 [],
        INTM 2 [],
        CLRI 3 [],
        SLP 4 []
    ],
    S Status [
        SECV 0 [],
        LRS 1 [],
        ORU 2 [],
        ERRI 3 [],
        SLP 4 []
    ]
];

const BASE_ADDRESS: *const Registers = 0x40029000 as *const Registers;

pub struct Rnga<'a> {
    regs: *const Registers,
    client: Cell<Option<&'a rng::Client>>,
}

pub static mut RNGA: Rnga<'static> = Rnga::new();

impl<'a> Rnga<'a> {
    const fn new() -> Rnga<'a> {
        Rnga {
            regs: BASE_ADDRESS,
            client: Cell::new(None),
        }
    }

    pub fn set_client(&self, client: &'a rng::Client) {
        self.client.set(Some(client));
    }
        
    pub fn set_clock_gate(&self) {
        use regs::sim::*;
        let sim = unsafe { &*SIM };
        sim.scgc6.modify(SCGC6::RNGA::SET);
    }

    pub fn normal_mode(&self) {
        let regs = unsafe { &*self.regs };
        regs.control.modify(CR::SLP::CLEAR);
    }

    pub fn sleep_mode(&self) {
        let regs = unsafe { &*self.regs };
        regs.control.modify(CR::SLP::SET);
    }

    pub fn start(&self) {
        let regs = unsafe { &*self.regs };
        regs.control.modify(CR::INTM::SET + CR::HA::SET + CR::GO::SET);
    }

    pub fn valid_data(&self) -> bool {
        let regs = unsafe { &*self.regs };
        regs.reg_level.get() == 1
    }

    pub fn get_data(&self) -> Option<u32> {
        if self.valid_data() {
            let regs = unsafe { &*self.regs };
            Some(regs.output.get())
        } else {
            None
        }
    }
    
    pub fn poll(&self) {
        while true {
            if !self.valid_data() {
                continue
            }
            
            let result = self.client.get().unwrap()
                .randomness_available(&mut RngaIter(self));
            if let Continue::Done = result {
                self.sleep_mode(); 
                break;
            }
        }
    }
}

struct RngaIter<'a, 'b: 'a>(&'a Rnga<'b>);

impl<'a, 'b> Iterator for RngaIter<'a, 'b> {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> { 
        self.0.get_data()
    }
}

impl<'a> rng::RNG for Rnga<'a> {
    fn get(&self) {
        self.set_clock_gate();
        self.normal_mode();
        self.start();
        self.poll();
    }
}
