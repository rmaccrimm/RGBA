extern crate byteorder;

use super::bus::Bus;
use super::cpu::arm7tdmi::ARMDecodeTable;

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::read;
use std::io::Cursor;

pub struct GBA {
    pub bus: Bus,
    pub arm_decode_table: ARMDecodeTable,
}

impl GBA {
    pub fn new() -> GBA {
        GBA {
            bus: Bus::new(),
            arm_decode_table: ARMDecodeTable::new(),
        }
    }

    /// Execute a single instruction and update all hardware
    pub fn step(&mut self) {
        let pc = self.bus.cpu.PC;
        let val = self.bus.mmu.read(pc);
        self.bus.mmu.write(pc, val + 1);
        self.bus.cpu.PC += 1;
    }

    pub fn load_rom(&mut self, filename: &String) {
        let file_bytes = read(filename).unwrap();
        println!("Read {} bytes", file_bytes.len());
        self.bus.mmu.ram = vec![0u32; file_bytes.len() / 4];
        let mut reader = Cursor::new(file_bytes);
        reader
            .read_u32_into::<LittleEndian>(&mut self.bus.mmu.ram)
            .expect("Failed to write bytes");
    }
}

#[cfg(test)]
mod test {}
