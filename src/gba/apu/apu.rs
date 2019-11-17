use crate::gba::bus::Bus;
use std::rc::Rc;
use std::cell::RefCell;

pub struct APU {
    bus: Rc<RefCell<Bus>>
}

impl APU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> APU {
        APU { bus: bus }
    }
}