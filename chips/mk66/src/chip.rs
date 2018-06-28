use cortexm4;
use kernel::Chip;
use pit;
use spi;
use gpio;
use uart;

pub struct MK66 {
    pub mpu: (),
    pub systick: (),
}

impl MK66 {
    pub unsafe fn new() -> MK66 {
        // Set up DMA channels
        // TODO: implement

        MK66 {
            mpu: (),
            systick: ()
        }
    }
}

impl Chip for MK66 {
    type MPU = ();
    type SysTick = ();

    fn service_pending_interrupts(&mut self) {
        use nvic::*;
        unsafe {
            while let Some(interrupt) = cortexm4::nvic::next_pending() {
                match interrupt {
                    PCMA => gpio::PA.handle_interrupt(),
                    PCMB => gpio::PB.handle_interrupt(),
                    PCMC => gpio::PC.handle_interrupt(),
                    PCMD => gpio::PD.handle_interrupt(),
                    PCME => gpio::PE.handle_interrupt(),
                    PIT2 => pit::PIT.handle_interrupt(),
                    SPI0 => spi::SPI0.handle_interrupt(),
                    SPI1 => spi::SPI1.handle_interrupt(),
                    SPI2 => spi::SPI2.handle_interrupt(),
                    UART0 => uart::UART0.handle_interrupt(),
                    UART1 => uart::UART1.handle_interrupt(),
                    _ => {}
                }

                let n = cortexm4::nvic::Nvic::new(interrupt);
                n.clear_pending();
                n.enable();
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm4::nvic::has_pending() }
    }

    fn mpu(&self) -> &Self::MPU {
        &self.mpu
    }

    fn systick(&self) -> &Self::SysTick {
        &self.systick
    }

    fn sleep(&self) {
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm4::support::atomic(f)
    }
}
