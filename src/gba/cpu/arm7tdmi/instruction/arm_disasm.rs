pub fn cond_string(instr: u32) -> String {
    String::from(match instr >> 28 {
        0b0000 => "EQ",
        0b0001 => "NE",
        0b0010 => "CS",
        0b0011 => "CC",
        0b0100 => "MI",
        0b0101 => "PL",
        0b0110 => "VS",
        0b0111 => "VC",
        0b1000 => "HI",
        0b1001 => "LS",
        0b1010 => "GE",
        0b1011 => "LT",
        0b1100 => "GT",
        0b1101 => "LE",
        0b1110 => "AL",
        _ => "??",
    })
}

pub fn branch(instr: u32) -> String {
    let _offset = (instr & 0x7fffff) << 4;
    let link = (instr >> 24 & 1) == 1;
    format!("{}", if link { "BL" } else { "B" })
}

#[allow(unused_variables)]
pub fn branch_exchange(instr: u32) -> String {
    format!("BX")
}

#[allow(unused_variables)]
pub fn alu_immediate(instr: u32) -> String {
    String::from("<ALU immediate>")
}

#[allow(unused_variables)]
pub fn alu(instr: u32) -> String {
    String::from("<ALU>")
}

#[allow(unused_variables)]
pub fn multiply(instr: u32) -> String {
    String::from("<Multiply>")
}
