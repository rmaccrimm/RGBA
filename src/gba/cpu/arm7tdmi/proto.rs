use crate::gba::bus::Bus;
use std::boxed::Box;

pub struct ARMInstruction {
    pub handler: Box<dyn Fn(&mut Bus, u32)>,
    pub assembly: Box<dyn Fn(u32) -> String>,
}

pub struct Matcher {
    pub bit_pattern: String,
    pub instr_pattern: String,
}

impl Matcher {
    pub fn new(bit_pattern: String, instr_pattern: String) -> Self {
        Matcher {
            bit_pattern,
            instr_pattern,
        }
    }

    pub fn match_instr(_instr: u32) -> Option<ARMInstruction> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn test_matcher() {
        
    }
}
