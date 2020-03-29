#![allow(unused_variables)]

use crate::gba::bus::Bus;
use crate::gba::cpu::arm7tdmi::{Registers, Flag};

pub fn check_cond(reg: &mut Registers, instr: u32) -> bool {
    let op: u8 = (instr >> 28) as u8;
    let n = reg.status(Flag::N);
    let z = reg.status(Flag::Z);
    let c = reg.status(Flag::C);
    let v = reg.status(Flag::V);
    match op {
        0b0000 => z,
        0b0001 => !z,
        0b0010 => c,
        0b0011 => !c,
        0b0100 => n,
        0b0101 => !n,
        0b0110 => v,
        0b0111 => !v,
        0b1000 => c && !z,
        0b1001 => !c || z,
        0b1010 => n == v,
        0b1011 => n != v,
        0b1100 => !z && (n == v),
        0b1101 => z && (n != v),
        0b1110 => true,
        _      => false, // undefined 
    }
}

pub fn branch(bus: &mut Bus, instr: u32) {
    
}

pub fn branch_exchange(bus: &mut Bus, instr: u32) {
    
}

pub fn data_proc_immediate(bus: &mut Bus, instr: u32) {
    
}

pub fn data_proc(bus: &mut Bus, instr: u32) {

}

pub fn multiply(bus: &mut Bus, instr: u32) {
    
}
    
