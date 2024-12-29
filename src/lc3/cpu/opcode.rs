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
            0 => Some(OpCode::BR),
            1 => Some(OpCode::ADD),
            2 => Some(OpCode::LD),
            3 => Some(OpCode::ST),
            4 => Some(OpCode::JSR),
            5 => Some(OpCode::AND),
            6 => Some(OpCode::LDR),
            7 => Some(OpCode::STR),
            8 => Some(OpCode::RTI),
            9 => Some(OpCode::NOT),
            10 => Some(OpCode::LDI),
            11 => Some(OpCode::STI),
            12 => Some(OpCode::JMP),
            13 => Some(OpCode::RES),
            14 => Some(OpCode::LEA),
            15 => Some(OpCode::TRAP),
            _ => None,
        }
    }
}
