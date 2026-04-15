use crate::cpu::{Cpu, registers::Register16};

fn test_ld_r16_imm16(opcode: u8, reg: Register16) {
    let value = 0x1234;
    let low = (value & 0x00FF) as u8;
    let high = (value >> 8) as u8;

    let mut cpu = Cpu::new_with_program(&[opcode, low, high]);
    cpu.step().unwrap();

    assert_eq!(cpu.get_register16(reg), value, "LD {:?}, imm16 failed", reg);
}

#[test]
fn test_ld_bc_imm16() {
    test_ld_r16_imm16(0x01, Register16::BC);
}

#[test]
fn test_ld_de_imm16() {
    test_ld_r16_imm16(0x11, Register16::DE);
}

#[test]
fn test_ld_hl_imm16() {
    test_ld_r16_imm16(0x21, Register16::HL);
}

#[test]
fn test_ld_sp_imm16() {
    test_ld_r16_imm16(0x31, Register16::SP);
}
