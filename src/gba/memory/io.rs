pub struct IORegisters {
    x: u32
}

impl IORegisters {
    pub fn new() -> IORegisters{
        IORegisters { x: 0 }
    }    

    fn read(&self, addr: u32) -> u32 {
        return self.x;
    }
    
    fn write(&mut self, addr: u32, val: u32) {
        self.x = val;
    }
}