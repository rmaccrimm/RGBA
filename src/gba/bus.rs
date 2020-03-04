use super::cpu::arm7tdmi::Registers;
use super::memory::MMU;

/// Bus provides access to all shared state within the emulator
pub struct Bus {
    pub mmu: MMU,
    pub cpu: Registers,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            mmu: MMU::new(),
            cpu: Registers::new()
        }
    }
}
