use super::io::IORegisters;

/*  Memory Mapping Unit, main memory stored as byte arrays, 
    4-bytes per word, little-endian format
*/
pub struct MMU {
    ram: [u8; 0x1000],
    io: IORegisters,
}

impl MMU {
    pub fn new() -> MMU {
        MMU { 
            ram: [0; 0x1000],
            io: IORegisters::new(),
        }
    }

    pub fn read(&self, addr: u32) -> u32 {
        let addr = (addr << 2) as usize;
        let bytes = &self.ram[addr..addr + 4];
        bytes.iter()
            .enumerate()
            .fold(0, |wd, (i, b)| { wd | (*b as u32) << i * 8 })
    }

    pub fn write(&mut self, addr: u32, data: u32) {
        let addr = (addr << 2) as usize;
        let bytes = &mut self.ram[addr..addr + 4];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (data >> (i * 8) & 0xff) as u8;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_word() {
        let mut mem = MMU::new();
        mem.write(1, 0xf7ae00b1);
        mem.write(2, 0x457f110c);
        assert_eq!(mem.ram[0..4], [0x00; 4]);
        assert_eq!(mem.ram[4..8], [0xb1, 0x00, 0xae, 0xf7]);
        assert_eq!(mem.ram[8..12], [0x0c, 0x11, 0x7f, 0x45]);
    }

    #[test]
    fn test_read_word() {
        let mut mem = MMU::new();
        let data: u32 = 0x89d8eaf0;
        mem.write(33, data);
        assert_eq!(data, mem.read(33));
    }
}
