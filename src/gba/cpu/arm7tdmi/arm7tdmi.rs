use super::{registers::Registers, Interrupts};
use crate::gba::SharedState;
use std::rc::Rc;
use std::cell::RefCell;

pub enum State {
    ARM,
    THUMB,
}

pub struct ARM7TDMI {
    // bus: Rc<RefCell<SharedState>>,
    registers: Registers,
    state: State,
}

impl ARM7TDMI {
    pub fn new(bus: &SharedState) -> ARM7TDMI {
        ARM7TDMI {
            // bus: Rc::new(RefCell::new(bus))
            registers: Registers::new(),
            state: State::ARM,
        }
    }

    pub fn step(&mut self) {
        
    }
}
