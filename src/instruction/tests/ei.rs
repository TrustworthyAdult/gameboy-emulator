use crate::cpu::{Cpu, snapshot::CpuSnapshot};

#[test]
fn ei_does_not_set_ime_immediately() {
    let mut cpu = Cpu::new_with_program(&[0xFB, 0x00]);
    cpu.step().unwrap();
    assert!(!cpu.ime());
}

#[test]
fn ei_sets_ime_after_next_instruction() {
    let mut cpu = Cpu::new_with_program(&[0xFB, 0x00]);
    cpu.step().unwrap();
    cpu.step().unwrap();
    assert!(cpu.ime());
}

#[test]
fn ei_only_advances_pc() {
    let mut cpu = Cpu::new_with_program(&[0xFB]);
    let before = CpuSnapshot::from(&cpu);

    cpu.step().unwrap();

    let expected = CpuSnapshot {
        pc: before.pc.wrapping_add(1),
        ..before
    };

    assert_eq!(CpuSnapshot::from(&cpu), expected);
}
