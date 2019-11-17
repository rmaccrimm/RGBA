use crate::gba::bus::Bus;
use super::registers::Registers;
use std::rc::Rc;
use std::cell::RefCell;

enum State {
    ARM,
    THUMB,
}

pub struct ARM7TDMI {
    bus: Rc<RefCell<Bus>>,
    registers: Registers,
    state: State,
}

impl ARM7TDMI {
    pub fn new(bus: Rc<RefCell<Bus>>) -> ARM7TDMI {
        ARM7TDMI {
            bus: bus,
            registers: Registers::new(),
            state: State::ARM,
        }
    }

    pub fn step(&mut self) {
        let mut bus = self.bus.borrow_mut();
        let x = bus.mmu.read(0);
        println!("{}", x);
        bus.mmu.write(0, 0xfae30000);
    }
}
