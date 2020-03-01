use super::cpu::arm7tdmi::ARM7TDMI;
use super::memory::MMU;

/// Bus provides access to all shared state within the emulator
pub struct Bus {
    pub mmu: MMU,
    pub cpu: ARM7TDMI
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            mmu: MMU::new(),
            cpu: ARM7TDMI::new()
        }
    }
}
