use super::hardware::Reg::{RegisterEnum,Registers};
use super::hardware::Memory::Memory;

/// Represents LC-3 instructions and their implementations.
pub struct Instructions;

impl Instructions {

    /// The ADD instruction can either:
    /// - Add two registers: `ADD DR, SR1, SR2`
    /// - Add a register and an immediate value: `ADD DR, SR1, imm5`
    pub fn add(instr: u16, registers: &mut Registers) {
        // Extract destination register (DR)
        let dr = Self::extract_register(instr, 9);
        // Extract first source register (SR1)
        let sr1 = Self::extract_register(instr, 6);
        // Determine if immediate mode is used
        let imm_flag = (instr >> 5) & 0x1;

        let result = if imm_flag != 0 {
            // Sign-extend the 5-bit immediate value
            let imm5 = sign_extend(instr & 0x1F, 5);
            // Perform addition with immediate value
            registers.read(sr1).wrapping_add(imm5)
        } else {
            // Extract second source register (SR2)
            let sr2 = Self::extract_register(instr, 0);
            // Perform addition with two registers
            registers.read(sr1).wrapping_add(registers.read(sr2))
        };

        // Write the result to the destination register
        registers.write(dr, result);

        // Update condition flags based on the result
        registers.update_flags(dr);
    }

    /// Executes the LD (Load) instruction.
    ///
    /// `LD DR, PCoffset9`:
    /// - Calculates the target address by adding sign-extended PCoffset9 to PC.
    /// - Loads the value from the target address into DR.
    /// - Updates condition flags based on the loaded value.
    pub fn ld(instr: u16, registers: &mut Registers, memory: &Memory) {
        // Extract destination register (DR)
        let dr = Self::extract_register(instr, 9);
        // Extract PCoffset9 and sign-extend it
        let pc_offset = sign_extend(instr & 0x1FF, 9) as i16;
        // Read current PC
        let pc = registers.read(RegisterEnum::PC) as i16;
        // Calculate target address with wrapping
        let target_address = (pc.wrapping_add(pc_offset)) as usize;
        // Read the value from the target address
        let value = memory.read(target_address);
        // Write the value to the destination register
        registers.write(dr, value);
        // Update condition flags based on the loaded value
        registers.update_flags(dr);
    }

    pub fn ldi(instr: u16, registers: &mut Registers, memory: &Memory) {
        // Extract destination register (DR)
        let dr = Self::extract_register(instr, 9);

        // Extract PCoffset9 and sign-extend it
        let pc_offset = sign_extend(instr & 0x1FF, 9) as i16;

        // Read current PC
        let pc = registers.read(RegisterEnum::PC) as i16;

        // Calculate the first address: PC + PCoffset9
        let addr1 = (pc.wrapping_add(pc_offset)) as usize;

        // Read the address stored at addr1
        let addr2 = memory.read(addr1) as usize;

        // Read the value stored at addr2
        let value = memory.read(addr2);

        // Write the value to the destination register
        registers.write(dr, value);

        // Update condition flags based on the loaded value
        registers.update_flags(dr);
    }

    /// Extracts a register from an instruction.
    /// 
    /// - `instr`: The 16-bit LC-3 instruction word.
    /// - `shift`: The bit position of the register in the instruction.
    ///
    /// Returns the corresponding `Register`.
    fn extract_register(instr: u16, shift: usize) -> RegisterEnum {
        match (instr >> shift) & 0x7 {
            0 => RegisterEnum::R0,
            1 => RegisterEnum::R1,
            2 => RegisterEnum::R2,
            3 => RegisterEnum::R3,
            4 => RegisterEnum::R4,
            5 => RegisterEnum::R5,
            6 => RegisterEnum::R6,
            7 => RegisterEnum::R7,
            _ => unreachable!(), // Should never happen due to 3-bit mask
        }
    }
}

/// Sign-extends a value to the given bit width.
/// 
/// - `x`: The value to sign-extend.
/// - `bit_count`: The original bit width of the value.
///
/// Returns the sign-extended value.
fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}




///Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lc3::hardware::{Registers, RegisterEnum};
    use crate::lc3::hardware::Flag::ConditionFlags;

    #[test]
    fn test_add_register_mode() {
        let mut registers = Registers::new();

        // Initialize registers
        registers.write(RegisterEnum::R0, 5);  // R0 = 5
        registers.write(RegisterEnum::R1, 10); // R1 = 10

        // Encode ADD R2, R0, R1
        let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
        Instructions::add(instr, &mut registers);

        // Verify R2 = R0 + R1 = 15
        assert_eq!(registers.read(RegisterEnum::R2), 15);
        // Verify condition flags are set to POS
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
    }

    #[test]
    fn test_add_immediate_mode_positive() {
        let mut registers = Registers::new();

        // Initialize registers
        registers.write(RegisterEnum::R0, 5); // R0 = 5

        // Encode ADD R2, R0, #3
        let instr = 0b0001_010_000_1_00011; // DR=R2, SR1=R0, imm5=3
        Instructions::add(instr, &mut registers);

        // Verify R2 = R0 + 3 = 8
        assert_eq!(registers.read(RegisterEnum::R2), 8);
        // Verify condition flags are set to POS
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
    }

    #[test]
    fn test_add_immediate_mode_negative() {
        let mut registers = Registers::new();

        // Initialize registers
        registers.write(RegisterEnum::R0, 5); // R0 = 5

        // Encode ADD R2, R0, #-3 (imm5 = 0b11101 which is -3)
        let instr = 0b0001_010_000_1_11101; // DR=R2, SR1=R0, imm5=-3
        Instructions::add(instr, &mut registers);

        // Verify R2 = R0 + (-3) = 2
        assert_eq!(registers.read(RegisterEnum::R2), 2);
        // Verify condition flags are set to POS
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
    }

    #[test]
    fn test_add_result_zero() {
        let mut registers = Registers::new();

        // Initialize registers
        registers.write(RegisterEnum::R0, 5);  // R0 = 5
        registers.write(RegisterEnum::R1, -5i16 as u16); // R1 = -5

        // Encode ADD R2, R0, R1
        let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
        Instructions::add(instr, &mut registers);

        // Verify R2 = R0 + R1 = 0
        assert_eq!(registers.read(RegisterEnum::R2), 0);
        // Verify condition flags are set to ZRO
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::ZRO.bits() as u16);
    }

    #[test]
    fn test_add_overflow() {
        let mut registers = Registers::new();

        // Initialize registers with maximum positive value
        registers.write(RegisterEnum::R0, 0x7FFF); // R0 = 32767
        registers.write(RegisterEnum::R1, 1);      // R1 = 1

        // Encode ADD R2, R0, R1
        let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
        Instructions::add(instr, &mut registers);

        // Verify R2 = R0 + R1 = 32768 (0x8000)
        assert_eq!(registers.read(RegisterEnum::R2), 0x8000);
        // Verify condition flags are set to NEG (since 0x8000 is negative in two's complement)
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::NEG.bits() as u16);
    }


    /// Tests the LDI instruction with a positive value.
    #[test]
    fn test_ldi_positive_value() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        // Initialize PC to 0x3000
        registers.write(RegisterEnum::PC, 0x3000);

        // Destination register R1
        //let dr = RegisterEnum::R1;

        // PCoffset9 = 0x001 (1)
        let pc_offset9 = 0x001;

        // Encode LDI R1, PCoffset9=1
        let instr = (0b1010 << 12) | (1 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R1, PCoffset9=1

        // Set memory at PC + 1 = 0x3001 to 0x4000 (addr2)
        memory.write(0x3001, 0x4000);

        // Set memory at addr2 = 0x4000 to 0xABCD (value to load)
        memory.write(0x4000, 0xABCD);

        // Execute the LDI instruction
        Instructions::ldi(instr, &mut registers, &memory);

        // Verify R1 = 0xABCD
        assert_eq!(registers.read(RegisterEnum::R1), 0xABCD);

        // Verify condition flags are set to POS
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
    }

    /// Tests the LDI instruction with a negative value.
    #[test]
    fn test_ldi_negative_value() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        // Initialize PC to 0x3000
        registers.write(RegisterEnum::PC, 0x3000);

        // PCoffset9 = 0x002 (2)
        let pc_offset9 = 0x002;

        // Encode LDI R2, PCoffset9=2
        let instr = (0b1010 << 12) | (2 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R2, PCoffset9=2

        // Set memory at PC + 2 = 0x3002 to 0x4001 (addr2)
        memory.write(0x3002, 0x4001);

        // Set memory at addr2 = 0x4001 to 0x8000 (negative value in two's complement)
        memory.write(0x4001, 0x8000);

        // Execute the LDI instruction
        Instructions::ldi(instr, &mut registers, &memory);

        // Verify R2 = 0x8000
        assert_eq!(registers.read(RegisterEnum::R2), 0x8000);

        // Verify condition flags are set to NEG
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::NEG.bits() as u16);
    }

    /// Tests the LDI instruction with a zero value.
    #[test]
    fn test_ldi_zero_value() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        // Initialize PC to 0x3000
        registers.write(RegisterEnum::PC, 0x3000);

        // PCoffset9 = 0x003 (3)
        let pc_offset9 = 0x003;

        // Encode LDI R3, PCoffset9=3
        let instr = (0b1010 << 12) | (3 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R3, PCoffset9=3

        // Set memory at PC + 3 = 0x3003 to 0x4002 (addr2)
        memory.write(0x3003, 0x4002);

        // Set memory at addr2 = 0x4002 to 0x0000 (zero value)
        memory.write(0x4002, 0x0000);

        // Execute the LDI instruction
        Instructions::ldi(instr, &mut registers, &memory);

        // Verify R3 = 0x0000
        assert_eq!(registers.read(RegisterEnum::R3), 0x0000);

        // Verify condition flags are set to ZRO
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::ZRO.bits() as u16);
    }

    /// Tests the LDI instruction with a negative PCoffset9 (backward addressing).
    #[test]
    fn test_ldi_negative_pc_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        // Initialize PC to 0x3001
        registers.write(RegisterEnum::PC, 0x3001);

        // PCoffset9 = 0x1FF (-1 in 9-bit two's complement)
        let pc_offset9 = 0x1FF;

        // Encode LDI R4, PCoffset9=-1
        let instr = (0b1010 << 12) | (4 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R4, PCoffset9=-1

        // Set memory at PC - 1 = 0x3000 to 0x4003 (addr2)
        memory.write(0x3000, 0x4003);

        // Set memory at addr2 = 0x4003 to 0x5678 (value to load)
        memory.write(0x4003, 0x5678);

        // Execute the LDI instruction
        Instructions::ldi(instr, &mut registers, &memory);

        // Verify R4 = 0x5678
        assert_eq!(registers.read(RegisterEnum::R4), 0x5678);

        // Verify condition flags are set to POS
        assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
    }

    /// Tests the LDI instruction with an out-of-bounds memory access.
    #[test]
    #[should_panic(expected = "Memory read out of bounds")]
    fn test_ldi_out_of_bounds() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Initialize PC to 0xFFFF
        registers.write(RegisterEnum::PC, 0xFFFF);

        // PCoffset9 = 0x001 (1)
        let pc_offset9 = 0x001;

        // Encode LDI R5, PCoffset9=1
        let instr = (0b1010 << 12) | (5 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R5, PCoffset9=1

        // Attempt to execute the LDI instruction, should panic due to out-of-bounds memory access
        Instructions::ldi(instr, &mut registers, &memory);
    }
}


    
