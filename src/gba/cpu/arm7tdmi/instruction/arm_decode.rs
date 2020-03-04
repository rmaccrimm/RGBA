use crate::gba::bus::Bus;
use std::vec::Vec;
use std::ops::Fn;
use super::instruction::*;

pub struct ARMDecodeTable {
    function_table: Vec<Box<dyn Fn(&mut Bus, u32)>>
}

/// Get index for function table. Only 12 bits are needed to decode the instruction set
fn index(instr: u32) -> u32 {
    return instr >> 16 & 0xff0 | instr >> 4 & 0xf0;
}

impl ARMDecodeTable {
    pub fn execute(&mut self, bus: &mut Bus, instr: u32) {
        (self.function_table[instr as usize])(bus, instr);
    }

    pub fn new() -> ARMDecodeTable {
        let mut table: Vec<Box<dyn Fn(&mut Bus, u32)>> = Vec::new();
        for _ in 0..(1 << 12) {
            table.push(Box::new(|_, _| {}));
        }

        // Branch instructions
        let pattern = 0b_0000_1010_0000_0000_0000_0000_0000_0000;
        for link in 0..2 {
            for bitmask_lo in 0..0xf {
                for bitmask_hi in 0..0xf {
                    let id = index(pattern | link << 23 | bitmask_hi << 20 | bitmask_lo << 4);
                    table[id as usize] = Box::new(|bus, instr| arm_branch(bus, instr));
                }
            }
        }
        let pattern = 0b_0000_0001_0010_1111_1111_1111_0001_0000;
        table[index(pattern) as usize] = Box::new(|bus, instr| arm_branch_exchange(bus, instr));

        // 
        let pattern
        
        ARMDecodeTable {
            function_table: table
        }
    }
}

