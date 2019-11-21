use crate::gba::cpu::arm7tdmi::ARM7TDMI;
use std::vec::Vec;
use std::ops::Fn;


struct ARMDecodeTable {
    function_table: Vec<Box<dyn Fn(&mut ARM7TDMI)>>
}

fn set_r0(cpu: &mut ARM7TDMI, val: u32) {
    cpu.reg.R0 = val;
}

impl ARMDecodeTable {
    pub fn execute(&mut self, cpu: &mut ARM7TDMI, instr: u32) {
        let decode_val: u16 = ((instr >> 16 & 0xff0) | (instr >> 4 & 0xf)) as u16;
        (self.function_table[decode_val as usize])(cpu);
    }

    pub fn new() -> ARMDecodeTable {
        let mut table: Vec<Box<dyn Fn(&mut ARM7TDMI)>> = Vec::new();
        table.push(Box::new(|cpu| set_r0(cpu, 19)));
        ARMDecodeTable {
            function_table: table
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gba::GBA;

    #[test]
    fn test_call_stored_closure() {
        let mut arm_decode_table = ARMDecodeTable::new();
        let mut gba = GBA::new();
        arm_decode_table.execute(&mut gba.cpu, 0);
        assert!(gba.cpu.reg.R0 == 19);
    }

    #[test]
    #[should_panic]
    fn test_call_undefined() {
        let mut arm_decode_table = ARMDecodeTable::new();
        let mut gba = GBA::new();
        arm_decode_table.execute(&mut gba.cpu, 1 << 4);
    }

}

