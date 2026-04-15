use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    cpu::registers::{Register8, Register16},
    reg,
};

pub enum Opcode {
    //Nop
    Nop,

    //Jump
    JpImm16,

    //Ld r8 imm8
    LdR8(Register8),

    // Ld r16 imm8
    LdR16(Register16),

    //Ld r8 r8
    LdR8R8 { dst: Register8, src: Register8 },

    // INC r8
    IncR8(Register8),

    // DEC r8
    DecR8(Register8),

    // INC r16
    IncR16(Register16),

    // DEC r16
    DecR16(Register16),

    // Ld MemHL r8
    LdMemHLR8(Register8),

    LdR8FromMemHL(Register8),

    LdAFromAddr,
    LdAddrA,
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeDecodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            // Nop
            0x00 => Ok(Opcode::Nop),

            // Jump
            0xC3 => Ok(Opcode::JpImm16),

            // Ld r8 imm8
            0x06 => Ok(Opcode::LdR8(reg!(B))),
            0x0E => Ok(Opcode::LdR8(reg!(C))),
            0x16 => Ok(Opcode::LdR8(reg!(D))),
            0x1E => Ok(Opcode::LdR8(reg!(E))),
            0x26 => Ok(Opcode::LdR8(reg!(H))),
            0x2E => Ok(Opcode::LdR8(reg!(L))),
            0x3E => Ok(Opcode::LdR8(reg!(A))),

            // Ld r8 r8
            0x40 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(B),
            }),
            0x41 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(C),
            }),
            0x42 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(D),
            }),
            0x43 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(E),
            }),
            0x44 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(H),
            }),
            0x45 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(L),
            }),
            0x47 => Ok(Opcode::LdR8R8 {
                dst: reg!(B),
                src: reg!(A),
            }),
            0x48 => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(B),
            }),
            0x49 => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(C),
            }),
            0x4A => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(D),
            }),
            0x4B => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(E),
            }),
            0x4C => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(H),
            }),
            0x4D => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(L),
            }),
            0x4F => Ok(Opcode::LdR8R8 {
                dst: reg!(C),
                src: reg!(A),
            }),
            0x50 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(B),
            }),
            0x51 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(C),
            }),
            0x52 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(D),
            }),
            0x53 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(E),
            }),
            0x54 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(H),
            }),
            0x55 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(L),
            }),
            0x57 => Ok(Opcode::LdR8R8 {
                dst: reg!(D),
                src: reg!(A),
            }),
            0x58 => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(B),
            }),
            0x59 => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(C),
            }),
            0x5A => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(D),
            }),
            0x5B => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(E),
            }),
            0x5C => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(H),
            }),
            0x5D => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(L),
            }),
            0x5F => Ok(Opcode::LdR8R8 {
                dst: reg!(E),
                src: reg!(A),
            }),
            0x60 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(B),
            }),
            0x61 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(C),
            }),
            0x62 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(D),
            }),
            0x63 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(E),
            }),
            0x64 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(H),
            }),
            0x65 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(L),
            }),
            0x67 => Ok(Opcode::LdR8R8 {
                dst: reg!(H),
                src: reg!(A),
            }),
            0x68 => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(B),
            }),
            0x69 => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(C),
            }),
            0x6A => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(D),
            }),
            0x6B => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(E),
            }),
            0x6C => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(H),
            }),
            0x6D => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(L),
            }),
            0x6F => Ok(Opcode::LdR8R8 {
                dst: reg!(L),
                src: reg!(A),
            }),
            0x78 => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(B),
            }),
            0x79 => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(C),
            }),
            0x7A => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(D),
            }),
            0x7B => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(E),
            }),
            0x7C => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(H),
            }),
            0x7D => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(L),
            }),
            0x7F => Ok(Opcode::LdR8R8 {
                dst: reg!(A),
                src: reg!(A),
            }),

            // Accumulator <=> Address
            0xFA => Ok(Opcode::LdAFromAddr),
            0xEA => Ok(Opcode::LdAddrA),

            // Inc r8
            0x3C => Ok(Opcode::IncR8(reg!(A))),
            0x04 => Ok(Opcode::IncR8(reg!(B))),
            0x0C => Ok(Opcode::IncR8(reg!(C))),
            0x14 => Ok(Opcode::IncR8(reg!(D))),
            0x1C => Ok(Opcode::IncR8(reg!(E))),
            0x24 => Ok(Opcode::IncR8(reg!(H))),
            0x2C => Ok(Opcode::IncR8(reg!(L))),

            //Dec r8
            0x3D => Ok(Opcode::DecR8(reg!(A))),
            0x05 => Ok(Opcode::DecR8(reg!(B))),
            0x0D => Ok(Opcode::DecR8(reg!(C))),
            0x15 => Ok(Opcode::DecR8(reg!(D))),
            0x1D => Ok(Opcode::DecR8(reg!(E))),
            0x25 => Ok(Opcode::DecR8(reg!(H))),
            0x2D => Ok(Opcode::DecR8(reg!(L))),

            // Inc r16
            0x03 => Ok(Opcode::IncR16(reg!(BC))),
            0x13 => Ok(Opcode::IncR16(reg!(DE))),
            0x23 => Ok(Opcode::IncR16(reg!(HL))),
            0x33 => Ok(Opcode::IncR16(reg!(SP))),

            // Dec r16
            0x0B => Ok(Opcode::DecR16(reg!(BC))),
            0x1B => Ok(Opcode::DecR16(reg!(DE))),
            0x2B => Ok(Opcode::DecR16(reg!(HL))),
            0x3B => Ok(Opcode::DecR16(reg!(SP))),

            // Ld r8 MemHL
            0x46 => Ok(Opcode::LdR8FromMemHL(reg!(B))),
            0x4E => Ok(Opcode::LdR8FromMemHL(reg!(C))),
            0x56 => Ok(Opcode::LdR8FromMemHL(reg!(D))),
            0x5E => Ok(Opcode::LdR8FromMemHL(reg!(E))),
            0x66 => Ok(Opcode::LdR8FromMemHL(reg!(H))),
            0x6E => Ok(Opcode::LdR8FromMemHL(reg!(L))),
            0x7E => Ok(Opcode::LdR8FromMemHL(reg!(A))),

            // Write MemHL R8
            0x70 => Ok(Opcode::LdMemHLR8(reg!(B))),
            0x71 => Ok(Opcode::LdMemHLR8(reg!(C))),
            0x72 => Ok(Opcode::LdMemHLR8(reg!(D))),
            0x73 => Ok(Opcode::LdMemHLR8(reg!(E))),
            0x74 => Ok(Opcode::LdMemHLR8(reg!(H))),
            0x75 => Ok(Opcode::LdMemHLR8(reg!(L))),
            0x77 => Ok(Opcode::LdMemHLR8(reg!(A))),

            0x01 => Ok(Opcode::LdR16(reg!(BC))),
            0x11 => Ok(Opcode::LdR16(reg!(DE))),
            0x21 => Ok(Opcode::LdR16(reg!(HL))),
            0x31 => Ok(Opcode::LdR16(reg!(SP))),

            _ => Err(OpcodeDecodeError::InvalidOpcode(byte)),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpcodeDecodeError {
    #[error("Invalid opcode: 0x{0:02X}")]
    InvalidOpcode(u8),
}
