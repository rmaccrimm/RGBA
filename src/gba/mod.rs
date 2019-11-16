pub mod memory;
pub mod cpu;

use memory::MMU;
use cpu::arm7tdmi::{Interrupts, Flags};

pub struct SharedState {
    mmu: MMU,
    interrupts: Interrupts,
    flags: Flags,
}
