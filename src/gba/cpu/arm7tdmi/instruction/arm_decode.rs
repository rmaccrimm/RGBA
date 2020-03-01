use crate::gba::bus::Bus;
use std::vec::Vec;
use std::ops::Fn;


pub struct ARMDecodeTable {
    function_table: Vec<Box<dyn Fn(&mut Bus)>>
}

fn set_r0(bus: &mut Bus, val: u32) {
    bus.cpu.R0 = val;
}

fn null(bus: &mut Bus) {}

impl ARMDecodeTable {
    /// Lookup the opcode in the precomputed instruction table.
    /// Only 12 bits are needed to fully decode the complete instruction set
    pub fn execute(&mut self, bus: &mut Bus, instr: u32) {
        // let decode_val: u16 = ((instr >> 16 & 0xff0) | (instr >> 4 & 0xf)) as u16;
        (self.function_table[instr as usize])(bus);
    }

    pub fn new() -> ARMDecodeTable {
        let mut table: Vec<Box<dyn Fn(&mut Bus)>> = Vec::new();
        for _ in 0..(1 << 12) {
            table.push(Box::new(|cpu| null(cpu)));
        }
        table[12] = Box::new(|bus| set_r0(bus, 19));
        ARMDecodeTable {
            function_table: table
        }
    }
}

