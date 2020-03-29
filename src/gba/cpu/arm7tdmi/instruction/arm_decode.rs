use super::{arm_disasm, arm_instr};
use crate::gba::bus::Bus;
use std::ops::Fn;
use std::vec::Vec;

pub struct ARMDecodeTable {
    instr_table: Vec<Box<dyn Fn(&mut Bus, u32)>>,
    disasm_table: Vec<Box<dyn Fn(u32) -> String>>,
}

/// Get index for function table. Only 12 bits are needed to decode the instruction set
/// ____ XXXX XXXX ____ ____ ____ XXXX ____
fn index(instr: u32) -> usize {
    let instr = instr as usize;
    return ((instr >> 16) & 0xff0) | ((instr >> 4) & 0xf);
}

impl ARMDecodeTable {
    pub fn execute(&mut self, bus: &mut Bus, instr: u32) {
        (self.instr_table[instr as usize])(bus, instr);
    }

    pub fn disassemble(&self, instr: u32) -> String {
        self.disasm_table[instr as usize](instr)
    }

    pub fn new() -> ARMDecodeTable {
        let mut instr_table: Vec<Box<dyn Fn(&mut Bus, u32)>> = Vec::new();
        let mut disasm_table: Vec<Box<dyn Fn(u32) -> String>> = Vec::new();
        for _ in 0..(1 << 12) {
            instr_table.push(Box::new(|_, _| {}));
        }

        // Branch instructions - B, BL, BLX
        let pattern = 0b_0000_1010_0000_0000_0000_0000_0000_0000;
        for link in 0u32..2 {
            // Set instruction for all possible offets
            for offset_lo in 0u32..0x10 {
                for offset_hi in 0u32..0x10 {
                    let id = index(pattern | link << 23 | offset_hi << 20 | offset_lo << 4);
                    instr_table[id] = Box::new(|bus, instr| arm_instr::branch(bus, instr));
                    disasm_table[id] = Box::new(|instr| arm_disasm::branch(instr));
                }
            }
        }
        // BX
        let id = index(0b_0000_0001_0010_1111_1111_1111_0001_0000);
        instr_table[id] = Box::new(|bus, instr| arm_instr::branch_exchange(bus, instr));
        disasm_table[id] = Box::new(|instr| arm_disasm::branch_exchange(instr));

        // Data processing immediate
        let pattern = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
        // bits 21 - 24
        for opcode in 0u32..0x10 {
            // bit 20
            for s in 0u32..2 {
                // s must be 1 for TST, TEQ, CMP, CMN
                if (opcode >= 0x8) && (opcode <= 0xb) && (s == 0) {
                    continue;
                }
                // bits 4 - 8 of shift operand
                for mask in 0u32..0x10 {
                    let id = index(pattern | (opcode << 21) | (s << 20) | (mask << 4));
                    instr_table[id] =
                        Box::new(|bus, instr| arm_instr::data_proc_immediate(bus, instr));
                    disasm_table[id] = Box::new(|instr| arm_disasm::data_proc_immediate(instr));
                }
            }
        }

        // Data processing
        let pattern = 0;
        // bits 21 - 24
        for opcode in 0u32..0x10 {
            // bit 20
            for s in 0u32..2 {
                // s must be 1 for TST, TEQ, CMP, CMN
                if (opcode >= 0x8) && (opcode <= 0xb) && (s == 0) {
                    continue;
                }
                // bit 4 - shift by register
                for r in 0u32..2 {
                    // if r = 1, bit 7 must be 0
                    let mask_range: u32 = if r == 1 { 4 } else { 8 };
                    for mask in 0..mask_range {
                        let id =
                            index(pattern | (opcode << 21) | (s << 20) | (mask << 5) | (r << 4));
                        instr_table[id] = Box::new(|bus, instr| arm_instr::data_proc(bus, instr));
                        disasm_table[id] = Box::new(|instr| arm_disasm::data_proc(instr));
                    }
                }
            }
        }

        // Multiply
        let pattern = 0b_10001_0000;
        // bits 21 - 24
        for &opcode in [0b0, 0b1, 0b10, 0b100, 0b101].iter() {
            // bit 20
            for &s in [0, 1].iter() {
                // s must be 0 for UMAAL
                if opcode == 0b10 && s == 0 {
                    continue;
                }
                let id = index(pattern | (opcode << 21) | (s << 20));
                instr_table[id] = Box::new(|bus, instr| arm_instr::multiply(bus, instr));
                disasm_table[id] = Box::new(|instr| arm_disasm::multiply(instr));
            }
        }

        ARMDecodeTable {
            instr_table,
            disasm_table,
        }
    }
}

#[cfg(test)]
mod test {}
