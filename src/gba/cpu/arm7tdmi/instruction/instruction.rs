#![allow(unused_variables)]

use crate::gba::cpu::arm7tdmi::ARM7TDMI;
use crate::gba::cpu::arm7tdmi::registers::Status;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub fn execute(instr: u32, cpu: &mut ARM7TDMI) {
    let Thumb = cpu.reg.CPSR & (Status::Thumb as u32) != 0;
    if Thumb {
        decode_Thumb(instr, cpu)
    }
    else {
        decode_ARM(instr, cpu);
    }
}

#[derive(FromPrimitive)]
enum Cond {
    EQ = 0,
    NE = 1,
    CS = 2,
    CC = 3,
    MI = 4,
    PL = 5,
    VS = 6,
    VC = 7,
    HI = 8,
    LS = 9,
    GE = 10,
    LT = 11,
    GT = 12,
    LE = 13,
    AL = 14,
    UNPREDICTABLE = 15,
}

fn check_cond(instr: u32, cpu: &mut ARM7TDMI) -> bool {
    let n = cpu.reg.CPSR & (Status::N as u32) != 0;
    let z = cpu.reg.CPSR & (Status::Z as u32) != 0;
    let c = cpu.reg.CPSR & (Status::C as u32) != 0;
    let v = cpu.reg.CPSR & (Status::V as u32) != 0;
    // impossible to fail, since only 4 bits
    let cond: Cond = FromPrimitive::from_u32(instr >> 28 & 0xf).unwrap();
    match cond {
        Cond::EQ =>  z,
        Cond::NE => !z,
        Cond::CS =>  c,
        Cond::CC => !c,
        Cond::MI =>  n,
        Cond::PL => !n,
        Cond::VS =>  v,
        Cond::VC => !v,
        Cond::HI =>  c || !z,
        Cond::LS => !c ||  z,
        Cond::GE => n == v,
        Cond::LT => n != v,
        Cond::GT => !z && (n == v),
        Cond::LE =>  z || (n != v),
        Cond::AL =>  true,
        Cond::UNPREDICTABLE => false
    }
}

fn decode_ARM(instr: u32, cpu: &mut ARM7TDMI) {

}

fn decode_Thumb(instr: u32, cpu: &mut ARM7TDMI) {
    
}

fn decode_ALU(instr: u32, cpu: &mut ARM7TDMI) {
    
}

fn decode_branch(instr: u32, cpu: &mut ARM7TDMI) {
    
}

    
