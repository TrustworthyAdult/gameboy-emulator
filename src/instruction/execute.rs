use super::{
    Instruction, Opcode, Operand,
    flag_adjustment::{self, FlagAdjustment},
};
use crate::{
    cpu::{
        Cpu,
        registers::{Register8, Register16},
    },
    reg, unpack_operand,
};
use thiserror::Error;

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), InstructionExecuteError> {
        let operand = &self.operand;
        match self.opcode {
            // Nop
            Opcode::Nop => Ok(()),

            // Jump
            Opcode::JpImm16 => self.jp_imm16(cpu, unpack_operand!(operand, Imm16)),

            // Ld r8 imm8
            Opcode::LdR8(r8) => self.ld_r8(cpu, r8, unpack_operand!(operand, Imm8)),

            // Ld r8 r8
            Opcode::LdR8R8 { dst, src } => self.ld_r8_r8(cpu, dst, src),

            // Ld MemHL r8
            Opcode::LdMemHLR8(reg) => self.ld_mem_hlr8(cpu, reg),

            // Ld r8 MemHL
            Opcode::LdR8FromMemHL(reg) => self.ld_r8_from_mem_hl(cpu, reg),

            // Ld A from Address
            Opcode::LdAFromAddr => self.ld_a_from_addr(cpu, unpack_operand!(operand, Address)),

            Opcode::LdAddrA => self.ld_addr_a(cpu, unpack_operand!(operand, Address)),

            // Inc r8
            Opcode::IncR8(reg) => self.inc_r8(cpu, reg),

            // Dec r8
            Opcode::DecR8(reg) => self.dec_r8(cpu, reg),

            // Inc r16
            Opcode::IncR16(reg) => self.inc_r16(cpu, reg),

            // Dec r16
            Opcode::DecR16(reg) => self.dec_r16(cpu, reg),

            // Ld r16
            Opcode::LdR16(reg) => self.ld_r16(cpu, reg, unpack_operand!(operand, Imm16)),
        }
    }

    fn jp_imm16(&self, cpu: &mut Cpu, addr: u16) -> Result<(), InstructionExecuteError> {
        cpu.set_pc(addr);
        Ok(())
    }

    fn ld_r8(&self, cpu: &mut Cpu, r8: Register8, imm8: u8) -> Result<(), InstructionExecuteError> {
        cpu.set_register8(r8, imm8);
        Ok(())
    }

    fn ld_r8_r8(
        &self,
        cpu: &mut Cpu,
        dst: Register8,
        src: Register8,
    ) -> Result<(), InstructionExecuteError> {
        cpu.set_register8(dst, cpu.get_register8(src));
        Ok(())
    }

    fn ld_mem_hlr8(&self, cpu: &mut Cpu, reg: Register8) -> Result<(), InstructionExecuteError> {
        cpu.write(cpu.get_register16(reg!(HL)), cpu.get_register8(reg));
        Ok(())
    }

    fn ld_r8_from_mem_hl(
        &self,
        cpu: &mut Cpu,
        reg: Register8,
    ) -> Result<(), InstructionExecuteError> {
        cpu.set_register8(reg, cpu.read(cpu.get_register16(reg!(HL))));
        Ok(())
    }

    fn ld_a_from_addr(&self, cpu: &mut Cpu, addr: u16) -> Result<(), InstructionExecuteError> {
        cpu.set_register8(reg!(A), cpu.read(addr));
        Ok(())
    }

    fn ld_addr_a(&self, cpu: &mut Cpu, addr: u16) -> Result<(), InstructionExecuteError> {
        cpu.write(addr, cpu.get_register8(reg!(A)));
        Ok(())
    }

    fn inc_r8(&self, cpu: &mut Cpu, reg: Register8) -> Result<(), InstructionExecuteError> {
        let before = cpu.get_register8(reg);
        let result = cpu.add_r8(reg, 1);

        let flag_adjustment = FlagAdjustment {
            zero: Some(result == 0),
            subtract: Some(false),
            half_carry: Some(before & 0b1111 == 0b1111),
            carry: None,
        };

        flag_adjustment.apply(cpu);

        Ok(())
    }

    fn dec_r8(&self, cpu: &mut Cpu, reg: Register8) -> Result<(), InstructionExecuteError> {
        let before = cpu.get_register8(reg);
        cpu.sub_r8(reg, 1);
        let result = cpu.get_register8(reg);

        FlagAdjustment {
            zero: Some(result == 0),
            subtract: Some(true),
            half_carry: Some(before & 0b1111 == 0b0000),
            carry: None,
        }
        .apply(cpu);

        Ok(())
    }

    fn inc_r16(&self, cpu: &mut Cpu, reg: Register16) -> Result<(), InstructionExecuteError> {
        cpu.add_r16(reg, 1);
        Ok(())
    }

    fn dec_r16(&self, cpu: &mut Cpu, reg: Register16) -> Result<(), InstructionExecuteError> {
        cpu.sub_r16(reg, 1);
        Ok(())
    }

    fn ld_r16(
        &self,
        cpu: &mut Cpu,
        reg: Register16,
        value: u16,
    ) -> Result<(), InstructionExecuteError> {
        cpu.set_register16(reg, value);
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum InstructionExecuteError {
    #[error("Unsupported instruction execution")]
    NotImplemented,

    #[error("Invalid register access")]
    InvalidRegister,

    #[error("Memory access failed")]
    MemoryError,

    #[error("Wrong operand type. Expected {expected}, received {received}")]
    UnexpectedOperand {
        expected: &'static str,
        received: &'static str,
    },
}
