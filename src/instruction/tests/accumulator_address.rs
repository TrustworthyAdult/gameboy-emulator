use crate::{cpu::Cpu, reg};

#[test]
fn test_ld_a_from_address() {
    let target_addr = 0x1234;
    let value = 0xAB;
    let program = [0xFA, 0x34, 0x12];

    let mut cpu = Cpu::new_with_program(&program);

    cpu.write(target_addr, value);

    assert_ne!(cpu.get_register8(reg!(A)), value);

    cpu.step().unwrap();

    assert_eq!(cpu.get_register8(reg!(A)), value);
}

#[test]
fn test_ld_address_a() {
    let addr: u16 = 0x1234;
    let value: u8 = 0xAB;
    let program = [0xEA, 0x34, 0x12];

    let mut cpu = Cpu::new_with_program(&program);

    cpu.set_register8(reg!(A), value);

    assert_ne!(cpu.read(addr), value);

    cpu.step().unwrap();

    assert_eq!(cpu.read(addr), value);
}
