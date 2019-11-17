use crate::gba::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GPU {
    bus: Rc<RefCell<Bus>>
}

impl GPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> GPU {
        GPU { bus: bus }
    }
}