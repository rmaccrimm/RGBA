use super::registers::Registers;
// use super::instruction::execute;
use crate::gba::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ARM7TDMI {
    pub reg: Registers,
    bus: Rc<RefCell<Bus>>,
    arm_decode_table: Vec<Option<Box<dyn FnMut()>>>
}

impl ARM7TDMI {
    pub fn new(bus: Rc<RefCell<Bus>>) -> ARM7TDMI {
        ARM7TDMI {
            bus: bus,
            reg: Registers::new(),
            arm_decode_table: Vec::new()
        }
    }

    pub fn step(&mut self) {
        // execute(self.fetch(), self);
    }

    pub fn fetch(&mut self) -> u32 {
        let bus = self.bus.borrow_mut();
        let instr = bus.mmu.read(self.reg.PC);
        self.reg.PC += 1;
        instr
    }
}
