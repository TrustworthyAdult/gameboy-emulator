use crate::{
    cpu::{Cpu, registers::Register8},
    reg,
};

fn test_ld_r8_imm8(register: Register8, opcode: u8) {
    let val = 0xAB;
    let mut cpu = Cpu::new_with_program(&[opcode, val]);
    cpu.step().unwrap();
    assert_eq!(cpu.get_register8(register), val);
}

#[test]
fn test_ld_b() {
    test_ld_r8_imm8(reg!(B), 0x06);
}

#[test]
fn test_ld_c() {
    test_ld_r8_imm8(reg!(C), 0x0E);
}

#[test]
fn test_ld_d() {
    test_ld_r8_imm8(reg!(D), 0x16);
}

#[test]
fn test_ld_e() {
    test_ld_r8_imm8(reg!(E), 0x1E);
}

#[test]
fn test_ld_h() {
    test_ld_r8_imm8(reg!(H), 0x26);
}

#[test]
fn test_ld_l() {
    test_ld_r8_imm8(reg!(L), 0x2E);
}

#[test]
fn test_ld_a() {
    test_ld_r8_imm8(reg!(A), 0x3E);
}
