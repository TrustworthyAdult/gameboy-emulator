use crate::cpu::{Cpu, flags::Flag};

pub struct FlagAdjustment {
    pub zero: Option<bool>,
    pub subtract: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>,
}

impl FlagAdjustment {
    pub fn apply(&self, cpu: &mut Cpu) {
        if let Some(value) = self.zero {
            cpu.set_flag(Flag::Zero, value);
        }

        if let Some(value) = self.subtract {
            cpu.set_flag(Flag::Subtract, value);
        }

        if let Some(value) = self.half_carry {
            cpu.set_flag(Flag::HalfCarry, value);
        }

        if let Some(value) = self.carry {
            cpu.set_flag(Flag::Carry, value);
        }
    }
}
