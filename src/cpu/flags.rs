use super::Cpu;

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry
}

impl Flag {
    pub fn bit_mask(&self) -> u8 {
        match self {
            Flag::Zero      => 0b10000000,
            Flag::Subtract  => 0b01000000,
            Flag::HalfCarry => 0b00100000,
            Flag::Carry     => 0b00010000
        }
    }
}

impl Cpu {
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.get_register8(reg!(F)) & flag.bit_mask() != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let f = self.get_register8(reg!(F));
        let mask = flag.bit_mask();
        self.set_register8(reg!(F), if value { f | mask } else { f & !mask });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::flat_memory::FlatMemory;

    fn make_cpu() -> Cpu {
        Cpu::new(Box::new(FlatMemory::new()))
    }

    #[test]
    fn test_set_flag_sets_true() {
        let mut cpu = make_cpu();
        cpu.set_flag(Flag::Zero, true);
        assert!(cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn test_set_flag_clears_false() {
        let mut cpu = make_cpu();
        cpu.set_flag(Flag::Zero, true);
        cpu.set_flag(Flag::Zero, false);
        assert!(!cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn test_set_flag_does_not_affect_others_when_setting() {
        let mut cpu = make_cpu();
        cpu.set_flag(Flag::Carry, true);
        cpu.set_flag(Flag::Zero, true);
        assert!(cpu.get_flag(Flag::Carry));
    }

    #[test]
    fn test_set_flag_does_not_affect_others_when_clearing() {
        let mut cpu = make_cpu();
        cpu.set_flag(Flag::Carry, true);
        cpu.set_flag(Flag::Zero, true);
        cpu.set_flag(Flag::Zero, false);
        assert!(cpu.get_flag(Flag::Carry));
        assert!(!cpu.get_flag(Flag::Zero));
    }
}
