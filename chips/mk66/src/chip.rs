use cortexm4;
use kernel::Chip;
use kernel::common::{RingBuffer, Queue};
use nvic;
use timer;

pub struct MK66 {
    pub mpu: cortexm4::mpu::MPU,
    pub systick: &'static cortexm4::systick::SysTick,
}

// Interrupt queue allocation
const IQ_SIZE: usize = 100;
static mut IQ_BUF: [nvic::NvicIdx; IQ_SIZE] = [nvic::NvicIdx::DMA0; IQ_SIZE];
pub static mut INTERRUPT_QUEUE: Option<RingBuffer<'static, nvic::NvicIdx>> = None;

impl MK66 {
    pub unsafe fn new() -> MK66 {
        // Initialize interrupt queue
        INTERRUPT_QUEUE = Some(RingBuffer::new(&mut IQ_BUF));

        // Set up DMA channels
        // TODO: implement

        MK66 {
            mpu: cortexm4::mpu::MPU::new(),
            systick: cortexm4::systick::SysTick::new(),
        }
    }
}

impl Chip for MK66 {
    type MPU = cortexm4::mpu::MPU;
    type SysTick = cortexm4::systick::SysTick;

    fn service_pending_interrupts(&mut self) {
        use nvic::NvicIdx::*;

        unsafe {
            let iq = INTERRUPT_QUEUE.as_mut().unwrap();
            while let Some(interrupt) = iq.dequeue() {
                match interrupt {
                    PIT0 => timer::PIT0.handle_interrupt(),
                    PIT1 => timer::PIT1.handle_interrupt(),
                    PIT2 => timer::PIT2.handle_interrupt(),
                    PIT3 => timer::PIT3.handle_interrupt(),
                    _ => {}
                }

                nvic::enable(interrupt);
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { INTERRUPT_QUEUE.as_mut().unwrap().has_elements() }
    }

    fn mpu(&self) -> &Self::MPU {
        &self.mpu
    }

    fn systick(&self) -> &Self::SysTick {
        &self.systick
    }
}
