pub struct IORegisters {
    pub x: u32
}

impl IORegisters {
    pub fn new() -> IORegisters{
        IORegisters { x: 0 }
    }    
}