use super::cpu::arm7tdmi::{Flags, Interrupts};
use super::memory::MMU;

/*  Bus owns all state that is shared between components, accessed globally via Rc<RefCell<Bus>>.
    Might eventually refactor into a heirarchy depending on performance
*/
pub struct Bus {
    pub mmu: MMU,
    pub interrupts: Interrupts,
    pub flags: Flags,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            mmu: MMU::new(),
            interrupts: Interrupts::new(),
            flags: Flags::new(),
        }
    }
}
