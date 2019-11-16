pub struct Registers {
    R0:  u32,
    R1:  u32,
    R2:  u32,
    R3:  u32,
    R4:  u32,
    R5:  u32,
    R6:  u32,
    R7:  u32,
    R8:  u32,
    R9:  u32,
    R10: u32,
    R11: u32,
    R12: u32,
    SP:  u32,
    LR:  u32,
    PC:  u32,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            R0:  0u32,
            R1:  0u32,
            R2:  0u32,
            R3:  0u32,
            R4:  0u32,
            R5:  0u32,
            R6:  0u32,
            R7:  0u32,
            R8:  0u32,
            R9:  0u32,
            R10: 0u32,
            R11: 0u32,
            R12: 0u32,
            SP:  0u32,
            LR:  0u32,
            PC:  0u32,
        }
    }
}
