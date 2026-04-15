use crate::cpu::{snapshot::CpuSnapshot, Cpu};

#[test]
fn halt_sets_halted() {
    let mut cpu = Cpu::new_with_program(&[0x76]);
    cpu.step().unwrap();
    assert!(cpu.halted());
}

#[test]
fn halt_only_advances_pc() {
    let mut cpu = Cpu::new_with_program(&[0x76]);
    let before = CpuSnapshot::from(&cpu);

    cpu.step().unwrap();

    let expected = CpuSnapshot {
        pc: before.pc.wrapping_add(1),
        ..before
    };

    assert_eq!(CpuSnapshot::from(&cpu), expected);
}

#[test]
fn halt_idles_on_subsequent_steps() {
    let mut cpu = Cpu::new_with_program(&[0x76, 0x00]);
    cpu.step().unwrap();
    let pc_after_halt = cpu.pc();
    cpu.step().unwrap();
    cpu.step().unwrap();
    assert_eq!(cpu.pc(), pc_after_halt);
}
