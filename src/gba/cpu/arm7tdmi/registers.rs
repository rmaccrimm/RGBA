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

pub enum Status {
    N = 1 << 31,
    Z = 1 << 30,
    C = 1 << 29,
    V = 1 << 28,
    DisableIRQ = 1 << 7,
    DisableFIQ = 1 << 6,
    Thumb = 1 << 5,
    Mode = 0x1f,
}

// May need additional values for backwards compatible modes
pub enum Mode {
    User = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

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
}

pub fn set(reg: u32, mask: u32) -> bool {
    reg & mask == mask
}

pub fn clear(reg: u32, mask: u32) -> bool {
    reg & mask == 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set() {
        let rg1 = 0b11111010111111010111101101011101;
        let rg2 = 0b00110000111101010111101101011101;
        let msk = 0b00111000111101010111101101011101;
        assert!(set(rg1, msk));
        assert!(!set(rg2, msk));
    }

    #[test]
    fn test_clear() {
        let rg1 = 0b11000111000010100000000000000000;
        let msk = 0b00111000111101010111101101011101;
        assert!(clear(rg1, msk));
    }
}
