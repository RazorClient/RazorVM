use crate::lc3::hardware::Flag::ConditionFlags;


pub struct Registers {
    pub data: [u16; 10], // R0-R7, PC (8), COND (9)
}

impl Registers {
    /// Creates a new Registers instance with all registers initialized to 0.
    pub fn new() -> Self {
        Registers {
            data: [0; 10],
        }
    }

    /// Reads a value from the specified register.
    pub fn read(&self, reg: usize) -> u16 {
        if reg < self.data.len() {
            self.data[reg]
        } else {
            panic!("Register read out of bounds: {}", reg);
        }
    }

    /// Writes a value to the specified register.
    pub fn write(&mut self, reg: usize, value: u16) {
        if reg < self.data.len() {
            self.data[reg] = value;
        } else {
            panic!("Register write out of bounds: {}", reg);
        }
    }

    /// Updates the condition flags (COND register) based on the value of the specified register.
    pub fn update_flags(&mut self, reg: usize) {
        if reg >= 8 {
            panic!("Cannot update flags for non-general-purpose register: {}", reg);
        }

        let value = self.data[reg] as i16;
        let new_flags = ConditionFlags::update_from_value(value);
        self.data[9] = new_flags.bits() as u16; // Store the flags in the COND register (R9)
    }
}

#[test]
fn test_register_read_write() {
    let mut registers = Registers::new();
    registers.write(0, 5);
    assert_eq!(registers.read(0), 5);
}

#[test]
fn test_update_flags() {
    let mut registers = Registers::new();
    registers.write(0, 0); // Zero
    registers.update_flags(0);
    assert_eq!(registers.read(9), ConditionFlags::ZRO.bits() as u16);

    registers.write(0, 1); // Positive
    registers.update_flags(0);
    assert_eq!(registers.read(9), ConditionFlags::POS.bits() as u16);

    registers.write(0, -1i16 as u16); // Negative
    registers.update_flags(0);
    assert_eq!(registers.read(9), ConditionFlags::NEG.bits() as u16);
}
