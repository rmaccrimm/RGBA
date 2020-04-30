use super::{arm_disasm, arm_instr};
use crate::gba::bus::Bus;
use std::ops::Fn;
use std::vec::Vec;

pub struct ARMDecodeTable {
    instr_table: Vec<Box<dyn Fn(&mut Bus, u32)>>,
    disasm_table: Vec<Box<dyn Fn(u32) -> String>>,
}

/// Get index for function tables. Only 12 bits are needed to decode the instruction set
/// ____ XXXX XXXX ____ ____ ____ XXXX ____
fn index(instr: u32) -> usize {
    let instr = instr as usize;
    let ind = ((instr >> 16) & 0xff0) | ((instr >> 4) & 0xf);
    assert!(ind < (1 << 12) as usize);
    ind
}

impl ARMDecodeTable {
    pub fn execute(&mut self, bus: &mut Bus, instr: u32) {
        (self.instr_table[instr as usize])(bus, instr);
    }

    pub fn disassemble(&self, instr: u32) -> String {
        self.disasm_table[index(instr)](instr)
    }

    pub fn new() -> ARMDecodeTable {
        let mut instr_table: Vec<Box<dyn Fn(&mut Bus, u32)>> = Vec::new();
        let mut disasm_table: Vec<Box<dyn Fn(u32) -> String>> = Vec::new();
        for _ in 0..(1 << 12) {
            instr_table.push(Box::new(|_, _| {}));
            disasm_table.push(Box::new(|_| {String::from("UNDEFINED")}));
        }

        // Branch instructions - B, BL, BLX
        let pattern = 0b_0000_1010_0000_0000_0000_0000_0000_0000;
        // bit 24
        for link in 0u32..2 { 
            // bits 20 - 23
            for offset_lo in 0u32..0x10 {
                for offset_hi in 0u32..0x10 {
                    let id = index(pattern | link << 24 | offset_hi << 20 | offset_lo << 4);
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
                // bits 4 - 7 of shift operand
                for mask in 0u32..0x10 {
                    let id = index(pattern | (opcode << 21) | (s << 20) | (mask << 4));
                    instr_table[id] =
                        Box::new(|bus, instr| arm_instr::data_proc_immediate(bus, instr));
                    disasm_table[id] = Box::new(|instr| arm_disasm::alu_immediate(instr));
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
                    let mask_range: u32 = if r == 1 { 8 } else { 16 };
                    for mask in 0..mask_range {
                        let id =
                            index(pattern | (opcode << 21) | (s << 20) | (mask << 5) | (r << 4));
                        instr_table[id] = Box::new(|bus, instr| arm_instr::data_proc(bus, instr));
                        disasm_table[id] = Box::new(|instr| arm_disasm::alu(instr));
                    }
                }
            }
        }

        // Multiply
        let pattern = 0b_1001_0000;
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
mod test {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(index(0xfedcba98), 0xed9);
        assert_eq!(index(0b0000_1010_0011_0001_0001_0001_1111_0000), 0b1010_0011_1111);
    }

    #[test]
    fn test_opcodes() {
        let decode_table = ARMDecodeTable::new();
        let disasm = |instr| {decode_table.disassemble(instr)};
        // Use "always" condition on all opcodes for now
        assert_eq!(disasm(0b1110_1010_0000_0000_0000_0000_0000_0000), "B");
        assert_eq!(disasm(0b1110_1010_0010_0111_0100_0000_0100_0001), "B");
        assert_eq!(disasm(0b1110_1010_1111_1111_1111_1111_1111_1111), "B");
        assert_eq!(disasm(0b1110_1011_0000_0000_0000_0000_0000_0000), "BL");
        assert_eq!(disasm(0b1110_1011_1111_1111_1111_1111_1111_1111), "BL");
        assert_eq!(disasm(0b1110_1011_0010_0101_0100_0100_0110_1001), "BL");
        assert_eq!(disasm(0b1110_0001_0010_1111_1111_1111_0001_0000), "BX");

        assert_eq!(disasm(0b1110_000_0000_0_0000_0000_0000_0110_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0000_1_0000_0000_0000_0001_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0001_0_0000_0000_0000_1000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0001_1_0000_0000_0000_0101_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0010_0_0000_0000_0000_1000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0010_1_0000_0000_0000_0101_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0011_0_0000_0000_0000_1111_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0011_1_0000_0000_0000_0010_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0100_0_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0100_1_0000_0000_0000_1000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0101_0_0000_0000_0000_0110_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0101_1_0000_0000_0000_1111_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0110_0_0000_0000_0000_0001_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0110_1_0000_0000_0000_0011_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0111_0_0000_0000_0000_1010_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_0111_1_0000_0000_0000_0101_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1000_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_000_1000_1_0000_0000_0000_0010_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1001_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_000_1001_1_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1010_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_000_1010_1_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1011_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_000_1011_1_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1100_0_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1100_1_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1101_0_0000_0000_0000_0110_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1101_1_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1110_0_0000_0000_0000_0001_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1110_1_0000_0000_0000_1000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1111_0_0000_0000_0000_0000_0000), "<ALU>");
        assert_eq!(disasm(0b1110_000_1111_1_0000_0000_0000_1111_0000), "<ALU>");

        assert_eq!(disasm(0b1110_001_0000_0_0000_0000_0000_0001_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0000_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0001_0_0000_0000_0000_0100_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0001_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0010_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0010_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0011_0_0000_0000_0000_0110_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0011_1_0000_0000_0000_0001_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0100_0_0000_0000_0000_1111_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0100_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0101_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0101_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0110_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0110_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0111_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_0111_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1000_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_001_1000_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1001_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_001_1001_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1010_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_001_1010_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1011_0_0000_0000_0000_0000_0000), "UNDEFINED");
        assert_eq!(disasm(0b1110_001_1011_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1100_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1100_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1101_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1101_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1110_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1110_1_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1111_0_0000_0000_0000_0000_0000), "<ALU immediate>");
        assert_eq!(disasm(0b1110_001_1111_1_0000_0000_0000_0000_0000), "<ALU immediate>");

        // Only MUL and MLA defined for ARMv4 (no halfword or long versions)
        assert_eq!(disasm(0b1110_000_0000_0_0000_0000_0000_1001_0000), "<Multiply>");
        assert_eq!(disasm(0b1110_000_0000_1_0000_0000_0000_1001_0000), "<Multiply>");
        assert_eq!(disasm(0b1110_000_0001_0_0000_0000_0000_1001_0000), "<Multiply>");
        assert_eq!(disasm(0b1110_000_0001_1_0000_0000_0000_1001_0000), "<Multiply>");
    }
}
