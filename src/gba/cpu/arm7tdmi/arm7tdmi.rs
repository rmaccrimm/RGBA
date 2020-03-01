use super::registers::Registers;
use crate::gba::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;
use super::instruction::arm_decode::ARMDecodeTable;

pub enum Mode {
    User,
    FIQ,
    IRQ,
    Supervisor,
    Abort,
    Undefined,
    System
}

pub struct ARM7TDMI {
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
    pub mode: Mode,
}

impl ARM7TDMI {
    pub fn new() -> ARM7TDMI {
        ARM7TDMI {
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
            mode: Mode::User
        }
    }

//     pub fn fetch(&mut self) -> u32 {
//         let bus = self.bus.borrow_mut();
//         let instr = bus.mmu.read(self.reg.PC);
//         self.reg.PC += 4;
//         instr
//     }

//     pub fn change_mode(&mut self, mode: Mode) {
//         self.mode = mode;
//     }
}
