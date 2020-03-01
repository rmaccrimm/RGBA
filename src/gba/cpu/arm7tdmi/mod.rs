pub mod arm7tdmi;
pub mod interrupts;
mod instruction; 
pub mod registers;

pub use arm7tdmi::ARM7TDMI;
// pub use interrupts::Interrupts;
// pub use registers::Registers;
pub use instruction::arm_decode::ARMDecodeTable;

