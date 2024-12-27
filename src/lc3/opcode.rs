#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    BR = 0, // Branch
    ADD,    // Add
    LD,     // Load
    ST,     // Store
    JSR,    // Jump Register
    AND,    // Bitwise AND
    LDR,    // Load Register
    STR,    // Store Register
    RTI,    // Unused
    NOT,    // Bitwise NOT
    LDI,    // Load Indirect
    STI,    // Store Indirect
    JMP,    // Jump
    RES,    // Reserved (Unused)
    LEA,    // Load Effective Address
    TRAP,   // Execute Trap
}

impl OpCode {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            _ => None,
        }
    }
}
