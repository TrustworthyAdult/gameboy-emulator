use super::{Instruction, Opcode, OpcodeDecodeError};
use crate::cpu::Cpu;
use thiserror::Error;

impl Instruction {
    pub fn decode(opcode: Opcode, cpu: &mut Cpu) -> Result<Self, InstructionDecodeError> {
        match opcode {
            Opcode::Nop => Ok(Instruction::Nop),
            Opcode::JpImm16 => Ok(Instruction::JpImm16(cpu.fetch_word())),
            Opcode::LdR8(reg) => Ok(Instruction::LdR8 {
                reg,
                value: cpu.fetch_byte(),
            }),
            Opcode::LdR8R8 { dst, src } => Ok(Instruction::LdR8R8 { dst, src }),
            Opcode::LdR16(reg) => Ok(Instruction::LdR16 {
                reg,
                value: cpu.fetch_word(),
            }),
            Opcode::IncR8(reg) => Ok(Instruction::IncR8(reg)),
            Opcode::DecR8(reg) => Ok(Instruction::DecR8(reg)),
            Opcode::IncR16(reg) => Ok(Instruction::IncR16(reg)),
            Opcode::DecR16(reg) => Ok(Instruction::DecR16(reg)),
            Opcode::LdMemHLR8(reg) => Ok(Instruction::LdMemHLR8(reg)),
            Opcode::LdR8FromMemHL(reg) => Ok(Instruction::LdR8FromMemHL(reg)),
            Opcode::LdAFromAddr => Ok(Instruction::LdAFromAddr(cpu.fetch_word())),
            Opcode::LdAddrA => Ok(Instruction::LdAddrA(cpu.fetch_word())),
            Opcode::Di => Ok(Instruction::Di),
            Opcode::Ei => Ok(Instruction::Ei),
        }
    }
}

#[derive(Debug, Error)]
pub enum InstructionDecodeError {
    #[error("Opcode decoding failed: {0}")]
    Opcode(#[from] OpcodeDecodeError),

    #[error("Invalid operands or malformed instruction")]
    OperandFormat,

    #[error("Instruction not yet implemented")]
    NotImplemented,
}
