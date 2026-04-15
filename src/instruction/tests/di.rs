use crate::cpu::{Cpu, snapshot::CpuSnapshot};

#[test]
fn di_clears_ime() {
    let mut cpu = Cpu::new_with_program(&[0xF3]);
    cpu.step().unwrap();
    assert!(!cpu.ime());
}

#[test]
fn di_only_advances_pc() {
    let mut cpu = Cpu::new_with_program(&[0xF3]);
    let before = CpuSnapshot::from(&cpu);

    cpu.step().unwrap();

    let expected = CpuSnapshot {
        pc: before.pc.wrapping_add(1),
        ..before
    };

    assert_eq!(CpuSnapshot::from(&cpu), expected);
}
