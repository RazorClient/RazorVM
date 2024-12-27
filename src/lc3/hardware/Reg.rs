use super::Flag::ConditionFlags;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterEnum {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8,   // Program Counter
    COND = 9, // Condition Flags
}

impl TryFrom<usize> for RegisterEnum {
    type Error = &'static str;

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(RegisterEnum::R0),
            1 => Ok(RegisterEnum::R1),
            2 => Ok(RegisterEnum::R2),
            3 => Ok(RegisterEnum::R3),
            4 => Ok(RegisterEnum::R4),
            5 => Ok(RegisterEnum::R5),
            6 => Ok(RegisterEnum::R6),
            7 => Ok(RegisterEnum::R7),
            8 => Ok(RegisterEnum::PC),
            9 => Ok(RegisterEnum::COND),
            _ => Err("Invalid register index"),
        }
    }
}

pub struct Registers {
    pub data: [u16; 10], // R0-R7, PC (8), COND (9)
}

impl Registers {
    /// Creates a new Registers instance with all registers initialized to 0.
    pub fn new() -> Self {
        Registers { data: [0; 10] }
    }

    /// Reads a value from the specified register.
    pub fn read(&self, reg: RegisterEnum) -> u16 {
        let index = reg as usize;
        self.data.get(index).copied().unwrap_or_else(|| {
            panic!("Register read out of bounds: {}", index);
        })
    }

    /// Writes a value to the specified register.
    pub fn write(&mut self, reg: RegisterEnum, value: u16) {
        let index = reg as usize;
        if let Some(reg_val) = self.data.get_mut(index) {
            *reg_val = value;
        } else {
            panic!("Register write out of bounds: {}", index);
        }
    }

    /// Updates the condition flags (COND register) based on the value of the specified register.
    pub fn update_flags(&mut self, reg: RegisterEnum) {
        let reg_index = reg as usize;

        if reg_index >= 8 {
            panic!(
                "Cannot update flags for non-general-purpose register: {:?}",
                reg
            );
        }

        let value = self.data[reg_index] as i16;
        let new_flags = ConditionFlags::update_from_value(value);
        self.data[RegisterEnum::COND as usize] = new_flags.bits() as u16; // Store the flags in the COND register (R9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_read_write_named() {
        let mut regs = Registers::new();

        // Write and read R0
        regs.write(RegisterEnum::R0, 0x1234);
        assert_eq!(regs.read(RegisterEnum::R0), 0x1234);

        // Write and read PC
        regs.write(RegisterEnum::PC, 0x3000);
        assert_eq!(regs.read(RegisterEnum::PC), 0x3000);
    }

    #[test]
    fn test_update_flags_named() {
        let mut regs = Registers::new();

        // Write 0 to R0 and update flags
        regs.write(RegisterEnum::R0, 0);
        regs.update_flags(RegisterEnum::R0);
        assert_eq!(
            regs.read(RegisterEnum::COND),
            ConditionFlags::ZRO.bits() as u16
        );

        // Write positive value to R1 and update flags
        regs.write(RegisterEnum::R1, 1);
        regs.update_flags(RegisterEnum::R1);
        assert_eq!(
            regs.read(RegisterEnum::COND),
            ConditionFlags::POS.bits() as u16
        );

        // Write negative value (-1 in two's complement) to R2 and update flags
        regs.write(RegisterEnum::R2, -1i16 as u16);
        regs.update_flags(RegisterEnum::R2);
        assert_eq!(
            regs.read(RegisterEnum::COND),
            ConditionFlags::NEG.bits() as u16
        );
    }
}
