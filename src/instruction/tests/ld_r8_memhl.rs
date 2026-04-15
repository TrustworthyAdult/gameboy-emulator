use crate::{
    cpu::{Cpu, registers::Register8},
    reg,
};

fn test_ld_r8_from_mem_hl(opcode: u8, dest: Register8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    let addr = 0xC123;
    let expected = 0xAB;

    cpu.set_register16(reg!(HL), addr);
    cpu.write(addr, expected);

    cpu.step().unwrap();

    assert_eq!(cpu.get_register8(dest), expected, "Failed for {:?}", dest);
}

#[test]
fn test_ld_a_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x7E, reg!(A));
}

#[test]
fn test_ld_b_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x46, reg!(B));
}

#[test]
fn test_ld_c_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x4E, reg!(C));
}

#[test]
fn test_ld_d_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x56, reg!(D));
}

#[test]
fn test_ld_e_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x5E, reg!(E));
}

#[test]
fn test_ld_h_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x66, reg!(H));
}

#[test]
fn test_ld_l_from_mem_hl() {
    test_ld_r8_from_mem_hl(0x6E, reg!(L));
}
