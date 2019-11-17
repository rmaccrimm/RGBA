use super::cpu::arm7tdmi::ARM7TDMI;
use super::gpu::GPU;
use super::apu::APU;
use super::bus::Bus;

use std::cell::RefCell;
use std::rc::Rc;

pub struct GBA {
    cpu: ARM7TDMI,
    gpu: GPU,
    apu: APU,
    bus: Rc<RefCell<Bus>>,
}

impl GBA {
    pub fn new() -> GBA {
        let shared = Rc::new(RefCell::new(Bus::new()));
        GBA {
            bus: Rc::clone(&shared),
            cpu: ARM7TDMI::new(Rc::clone(&shared)),
            gpu: GPU::new(Rc::clone(&shared)),
            apu: APU::new(Rc::clone(&shared)),
        }
    }

    // begin program execution
    pub fn run(&mut self) {

    }
}