use super::cpu::arm7tdmi::ARMDecodeTable;
use super::bus::Bus;

pub struct GBA {
    pub bus: Bus,
    arm_decode_table: ARMDecodeTable
}

impl GBA {
    pub fn new() -> GBA {
        GBA {
            bus: Bus::new(),
            arm_decode_table: ARMDecodeTable::new()
        }
    }

    /// Execute a single instruction and update all hardware
    pub fn step(&mut self) {
        let pc = self.bus.cpu.PC;
        println!("PC is {}", pc);
        let val = self.bus.mmu.read(pc);
        self.bus.mmu.write(pc, val + 1);
        self.bus.cpu.PC += 1;
    }
}
    

#[cfg(test)]
mod test {
}

