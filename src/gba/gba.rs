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

    /// begin program execution
    pub fn run(&mut self) {
        self.arm_decode_table.execute(&mut self.bus, 12);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_call_stored_closure() {
        let mut gba = GBA::new();
        gba.run();
        assert!(gba.bus.cpu.R0 == 19);
    }
}

