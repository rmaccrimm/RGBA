pub mod interrupts;
pub mod instruction; 
pub mod registers;

// pub use interrupts::Interrupts;
pub use registers::{Registers, Flag};
pub use instruction::arm_decode::ARMDecodeTable;

