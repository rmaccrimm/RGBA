 pub struct Registers {
    pub R0: u32,
    pub R1: u32,
    pub R2: u32,
    pub R3: u32,
    pub R4: u32,
    pub R5: u32,
    pub R6: u32,
    pub R7: u32,
    pub R8: u32,
    pub R9: u32,
    pub R10: u32,
    pub R11: u32,
    pub R12: u32,
    pub SP: u32,
    pub LR: u32,
    pub PC: u32,
    pub CPSR: u32,
 }

pub enum Flag { N, Z, C, V }

impl Registers {
    pub fn new() -> Registers {
        Registers {
            R0: 0u32,
            R1: 0u32,
            R2: 0u32,
            R3: 0u32,
            R4: 0u32,
            R5: 0u32,
            R6: 0u32,
            R7: 0u32,
            R8: 0u32,
            R9: 0u32,
            R10: 0u32,
            R11: 0u32,
            R12: 0u32,
            SP: 0u32,
            LR: 0u32,
            PC: 0u32,
            CPSR: 0,
        }
    }

    // Get current status of a flag
    pub fn status(&self, flag: Flag) -> bool {
        let result = match flag {
            Flag::N => self.CPSR & (1 << 31),
            Flag::Z => self.CPSR & (1 << 30),
            Flag::C => self.CPSR & (1 << 29),
            Flag::V => self.CPSR & (1 << 28),
        };
        result != 0
    }

    #[allow(unused_variables)]
    pub fn set_flag(&mut self, flag: Flag) {
        // match flag {
            // N => set(self.CPSR, N)
        // };
    }
}

fn set(reg: u32, mask: u32) -> bool {
    reg & mask == mask
}

fn clear(reg: u32, mask: u32) -> bool {
    reg & mask == 0
}


#[cfg(test)]
mod test {
}
