pub mod decode;
pub mod execute;
mod flag_adjustment;
pub mod opcode;

pub use decode::InstructionDecodeError;
pub use execute::InstructionExecuteError;
pub use opcode::{Opcode, OpcodeDecodeError};

use crate::cpu::registers::{Register8, Register16};

#[cfg(test)]
mod tests;

pub enum Instruction {
    Nop,
    JpImm16(u16),
    LdR8 { reg: Register8, value: u8 },
    LdR8R8 { dst: Register8, src: Register8 },
    LdR16 { reg: Register16, value: u16 },
    IncR8(Register8),
    DecR8(Register8),
    IncR16(Register16),
    DecR16(Register16),
    LdMemHLR8(Register8),
    LdR8FromMemHL(Register8),
    LdAFromAddr(u16),
    LdAddrA(u16),
    Di,
    Ei,
}
