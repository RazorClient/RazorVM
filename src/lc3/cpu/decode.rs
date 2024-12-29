use super::opcode::OpCode;
//use crate::lc3::cpu::instruction; // Access instruction module
//use crate::lc3::hardware; // Access hardware components
pub struct Decoder;

impl Decoder {
    pub fn decode(op: u16) -> Option<OpCode> {
        let opcode = OpCode::from((op >> 12) as u8);
        if let Some(op) = opcode {
            if op == OpCode::ADD {
                // Call execute_add if the opcode is ADD
                // Assuming we have access to registers and memory here
            }
            return Some(op);
        }
        None
    }
}
