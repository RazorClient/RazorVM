use crate::lc3::cpu::instruction::Instructions;
use crate::lc3::cpu::opcode::{OpCode, OpCodeError};
use crate::lc3::hardware::{Memory::Memory, Reg::Registers};

/// Extracts the opcode (top 4 bits) from a 16-bit instruction.
#[inline]
pub fn extract_op_code(instruction: u16) -> Result<OpCode, OpCodeError> {
    OpCode::get(instruction >> 12)
}

pub fn execute_instruction(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    match extract_op_code(instr) {
        Ok(OpCode::Add) => Instructions::add(instr, registers),
        Ok(OpCode::And) => Instructions::bitwise_and(instr, registers),
        Ok(OpCode::Not) => Instructions::bitwise_not(instr, registers),
        Ok(OpCode::Br) => Instructions::br(instr, registers),
        Ok(OpCode::Jmp) => Instructions::jmp(instr, registers),
        Ok(OpCode::Jsr) => Instructions::jsr(instr, registers),
        Ok(OpCode::Ld) => Instructions::ld(instr, registers, memory),
        Ok(OpCode::Ldi) => Instructions::ldi(instr, registers, memory),
        Ok(OpCode::Ldr) => Instructions::ldr(instr, registers, memory),
        Ok(OpCode::Lea) => Instructions::lea(instr, registers),
        Ok(OpCode::St) => Instructions::st(instr, registers, memory),
        Ok(OpCode::Sti) => Instructions::sti(instr, registers, memory),
        Ok(OpCode::Str) => Instructions::str(instr, registers, memory),
        Ok(OpCode::Trap) => Instructions::trap(instr, registers, memory),
        Ok(OpCode::Rti) => {
            eprintln!("RTI instruction encountered: {:#06X}", instr);
        }
        Ok(OpCode::Res) => {
            eprintln!("RES instruction encountered: {:#06X}", instr);
        }
        Err(_) => eprintln!("Invalid opcode in instruction: {:#06X}", instr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lc3::cpu::opcode::OpCode;

    #[test]
    fn test_extract_all_opcodes() {
        for opcode in 0..=15 {
            let instruction = (opcode as u16) << 12; // Set top 4 bits to opcode
            let expected = OpCode::get(opcode).unwrap();
            assert_eq!(
                extract_op_code(instruction),
                Ok(expected),
                "Failed to decode opcode {}",
                opcode
            );
        }
    }
}
