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
    let offset = (instr & 0x7fffff) << 4;
    let link = (instr >> 24 & 1) == 1;
    format!("{}", if link { "BL" } else { "B" })
}

pub fn branch_exchange(instr: u32) -> String {
    let link = (instr >> 24 & 1) == 1;
    format!("{}", if link { "BLX" } else { "BX" })
}

pub fn data_proc_immediate(instr: u32) -> String {
    String::from("<Data Proc immediate>")
}

pub fn data_proc(instr: u32) -> String {
    String::from("<Data Proc>")
}

pub fn multiply(instr: u32) -> String {
    String::from("<Multiply>")
}
