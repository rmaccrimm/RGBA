use super::registers::Registers;
use crate::gba::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;

pub enum Mode {
    User,
    FIQ,
    IRQ,
    Supervisor,
    Abort,
    Undefined,
    System
}

pub struct ARM7TDMI {
    pub reg: Registers,
    bus: Rc<RefCell<Bus>>,
    mode: Mode,
    arm_decode_table: Vec<Option<Box<dyn FnMut()>>>,
}

impl ARM7TDMI {
    pub fn new(bus: Rc<RefCell<Bus>>) -> ARM7TDMI {
        ARM7TDMI {
            bus: bus,
            mode: Mode::User,
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

    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
