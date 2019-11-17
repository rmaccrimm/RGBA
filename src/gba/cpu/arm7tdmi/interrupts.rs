pub struct Interrupts {
    pub i: bool
}

impl Interrupts {
    pub fn new() -> Interrupts {
        Interrupts { i: false }
    }
}