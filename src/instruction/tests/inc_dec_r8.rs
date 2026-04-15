use crate::{
    cpu::{Cpu, flags::Flag, registers::Register8},
    reg,
};

fn test_inc_r8(register: Register8, opcode: u8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    cpu.set_register8(register, 0x41);

    cpu.step().unwrap();
    assert_eq!(cpu.get_register8(register), 0x42);
}

fn test_dec_r8(register: Register8, opcode: u8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    cpu.set_register8(register, 0x43);

    cpu.step().unwrap();
    assert_eq!(cpu.get_register8(register), 0x42);
}

// Increment
#[test]
fn test_inc_a() {
    test_inc_r8(reg!(A), 0x3C);
}

#[test]
fn test_inc_b() {
    test_inc_r8(reg!(B), 0x04);
}

#[test]
fn test_inc_c() {
    test_inc_r8(reg!(C), 0x0C);
}

#[test]
fn test_inc_d() {
    test_inc_r8(reg!(D), 0x14);
}

#[test]
fn test_inc_e() {
    test_inc_r8(reg!(E), 0x1C);
}

#[test]
fn test_inc_h() {
    test_inc_r8(reg!(H), 0x24);
}

#[test]
fn test_inc_l() {
    test_inc_r8(reg!(L), 0x2C);
}

// inc_r8 flag tests (opcode 0x04 = INC B)
#[test]
fn test_inc_r8_zero_flag_set() {
    let mut cpu = Cpu::new_with_program(&[0x04]);
    cpu.set_register8(reg!(B), 0xFF);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Subtract));
    assert!(cpu.get_flag(Flag::HalfCarry));
}

#[test]
fn test_inc_r8_zero_flag_clear() {
    let mut cpu = Cpu::new_with_program(&[0x04]);
    cpu.set_register8(reg!(B), 0x41);
    cpu.step().unwrap();
    assert!(!cpu.get_flag(Flag::Zero));
}

#[test]
fn test_inc_r8_half_carry_set() {
    let mut cpu = Cpu::new_with_program(&[0x04]);
    cpu.set_register8(reg!(B), 0x0F);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::HalfCarry));
}

#[test]
fn test_inc_r8_half_carry_clear() {
    let mut cpu = Cpu::new_with_program(&[0x04]);
    cpu.set_register8(reg!(B), 0x10);
    cpu.step().unwrap();
    assert!(!cpu.get_flag(Flag::HalfCarry));
}

#[test]
fn test_inc_r8_subtract_clear() {
    let mut cpu = Cpu::new_with_program(&[0x04]);
    cpu.set_register8(reg!(B), 0x01);
    cpu.step().unwrap();
    assert!(!cpu.get_flag(Flag::Subtract));
}

#[test]
fn test_inc_r8_carry_unaffected() {
    let mut cpu = Cpu::new_with_program(&[0x04]);
    cpu.set_flag(Flag::Carry, true);
    cpu.set_register8(reg!(B), 0x01);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::Carry));
}

// dec_r8 flag tests (opcode 0x05 = DEC B)
#[test]
fn test_dec_r8_zero_flag_set() {
    let mut cpu = Cpu::new_with_program(&[0x05]);
    cpu.set_register8(reg!(B), 0x01);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::Zero));
}

#[test]
fn test_dec_r8_zero_flag_clear() {
    let mut cpu = Cpu::new_with_program(&[0x05]);
    cpu.set_register8(reg!(B), 0x43);
    cpu.step().unwrap();
    assert!(!cpu.get_flag(Flag::Zero));
}

#[test]
fn test_dec_r8_half_carry_set() {
    let mut cpu = Cpu::new_with_program(&[0x05]);
    cpu.set_register8(reg!(B), 0x10);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::HalfCarry));
}

#[test]
fn test_dec_r8_half_carry_clear() {
    let mut cpu = Cpu::new_with_program(&[0x05]);
    cpu.set_register8(reg!(B), 0x42);
    cpu.step().unwrap();
    assert!(!cpu.get_flag(Flag::HalfCarry));
}

#[test]
fn test_dec_r8_subtract_set() {
    let mut cpu = Cpu::new_with_program(&[0x05]);
    cpu.set_register8(reg!(B), 0x02);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::Subtract));
}

#[test]
fn test_dec_r8_carry_unaffected() {
    let mut cpu = Cpu::new_with_program(&[0x05]);
    cpu.set_flag(Flag::Carry, true);
    cpu.set_register8(reg!(B), 0x02);
    cpu.step().unwrap();
    assert!(cpu.get_flag(Flag::Carry));
}

// Decrement
#[test]
fn test_dec_a() {
    test_dec_r8(reg!(A), 0x3D);
}

#[test]
fn test_dec_b() {
    test_dec_r8(reg!(B), 0x05);
}

#[test]
fn test_dec_c() {
    test_dec_r8(reg!(C), 0x0D);
}

#[test]
fn test_dec_d() {
    test_dec_r8(reg!(D), 0x15);
}

#[test]
fn test_dec_e() {
    test_dec_r8(reg!(E), 0x1D);
}

#[test]
fn test_dec_h() {
    test_dec_r8(reg!(H), 0x25);
}

#[test]
fn test_dec_l() {
    test_dec_r8(reg!(L), 0x2D);
}
