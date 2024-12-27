use super::hardware::Memory::Memory;
use super::hardware::Reg::{RegisterEnum, Registers};

/// Represents LC-3 instructions and their implementations.
pub struct Instructions;

impl Instructions {
    /// The ADD instruction can either:
    /// - Add two registers: `ADD DR, SR1, SR2`
    /// - Add a register and an immediate value: `ADD DR, SR1, imm5`
    pub fn add(instr: u16, registers: &mut Registers) {
        // Extract destination register (DR)
        let dr = extract_register(instr, 9);
        // Extract first source register (SR1)
        let sr1 = extract_register(instr, 6);
        // Determine if immediate mode is used
        let imm_flag = (instr >> 5) & 0x1;

        let result = if imm_flag != 0 {
            // Sign-extend the 5-bit immediate value
            let imm5 = sign_extend(instr & 0x1F, 5);
            // Perform addition with immediate value
            registers.read(sr1).wrapping_add(imm5)
        } else {
            // Extract second source register (SR2)
            let sr2 = extract_register(instr, 0);
            // Perform addition with two registers
            registers.read(sr1).wrapping_add(registers.read(sr2))
        };

        // Write the result to the destination register
        registers.write(dr, result);

        // Update condition flags based on the result
        registers.update_flags(dr);
    }

    pub fn ldi(instr: u16, registers: &mut Registers, memory: &Memory) {
        // Extract destination register (DR)
        let dr = extract_register(instr, 9);

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
    /// If bit [5] is 0, the second source operand is obtained from SR2.
    /// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
    /// In either case, the second source operand and the contents of SR1 are bit- wise ANDed,
    /// and the result stored in DR. The condition codes are set, based on whether the binary value produced,
    /// taken as a 2’s complement integer, is negative, zero, or positive.
    ///  BIT-Wise AND instruction.
    pub fn bitwise_and(instr: u16, registers: &mut Registers) {
        // Extract destination register (DR)
        let dr = extract_register(instr, 9);

        // Extract source register (SR1)
        let sr1 = extract_register(instr, 6);

        // Check immediate flag
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag != 0 {
            // Immediate mode: Extract and sign-extend imm5
            let imm5 = sign_extend(instr & 0x1F, 5);

            // Perform bitwise AND with immediate value
            let result = registers.read(sr1) & imm5;

            // Write the result to the destination register
            registers.write(dr, result);
        } else {
            // Register mode: Extract source register 2 (SR2)
            let sr2 = extract_register(instr, 0);

            // Perform bitwise AND with second register
            let result = registers.read(sr1) & registers.read(sr2);

            // Write the result to the destination register
            registers.write(dr, result);
        }

        // Update condition flags based on the result
        registers.update_flags(dr);
    }

    /// Executes the NOT (Bitwise NOT) instruction.
    ///
    /// `NOT DR, SR`:
    /// - Performs a bitwise NOT on the value in SR.
    /// - Stores the result in DR.
    /// - Updates condition flags based on the result.
    pub fn bitwise_not(instr: u16, registers: &mut Registers) {
        // Extract destination register (DR) from bits 11-9
        let dr = extract_register(instr, 9);
        // Extract source register (SR) from bits 8-6
        let sr = extract_register(instr, 6);

        // Perform bitwise NOT on the value in SR
        let value = !registers.read(sr);

        // Write the result to DR
        registers.write(dr, value);

        // Update condition flags based on the result
        registers.update_flags(dr);
    }

    /// `BR n,z,p PCoffset9`:
    /// - Checks the condition flags (`n`, `z`, `p`).
    /// - If any of the specified flags match the current condition flags, branch is taken.
    /// - Branching is performed by adding the sign-extended `PCoffset9` to the current `PC`.
    pub fn br(instr: u16, registers: &mut Registers) {
        // Extract condition flags from bits 11-9
        let cond_flag = (instr >> 9) & 0x7; // 3 bits: n, z, p

        // Extract PCoffset9 from bits 8-0 and sign-extend it
        let pc_offset = sign_extend(instr & 0x1FF, 9) as i16;

        // Current PC
        let pc = registers.read(RegisterEnum::PC) as i16;

        // Current condition flags
        let current_cond = registers.read(RegisterEnum::COND) as u16;

        // Check if any of the specified condition flags are set
        if cond_flag & current_cond != 0 {
            // Branch is taken: update PC
            let new_pc = pc.wrapping_add(pc_offset);
            registers.write(RegisterEnum::PC, new_pc as u16);
        }
        // If branch not taken, PC remains unchanged (already incremented)
    }
    // 15        12 11 9 8 6 5         0
    // +------------+-----+-----------+
    // |   Opcode   |BaseR|   Unused   |
    // +------------+-----+-----------+

    /// `JMP BaseR`:
    /// - Sets PC to the value contained in BaseR.
    /// - Also handles the RET instruction when BaseR is R7.
    pub fn jmp(instr: u16, registers: &mut Registers) {
        // Extract Base Register (BaseR) from bits 11-9
        let base_r_index = (instr >> 6) & 0x7; // Bits 8-6 represent BaseR
        let base_r = RegisterEnum::try_from(base_r_index as usize).expect("Invalid Base Register");

        // Retrieve the value from BaseR
        let target_address = registers.read(base_r);

        // Set PC to the target address
        registers.write(RegisterEnum::PC, target_address);
    }

    // 15        12 11        0
    // +------------+------------+
    // |   Opcode   | PCoffset11  |
    // +------------+------------+
        /// Executes the JSR (Jump to Subroutine) instruction.
    ///
    /// `JSR PCoffset11`:
    /// - Stores the current PC in R7.
    /// - Adds the sign-extended PCoffset11 to the current PC to get the target address.
    /// - Sets PC to the target address.
    pub fn jsr(instr: u16, registers: &mut Registers) {
        let base_reg = (instr >> 6) & 0x7;
        let long_pc_offset = sign_extend(instr & 0x7ff, 11);
        let long_flag = (instr >> 11) & 1;
        let read_val=registers.read(RegisterEnum::PC);
        registers.write(RegisterEnum::R7,read_val);
    
        if long_flag != 0 {
            let val: u16 = registers.read(RegisterEnum::PC) + long_pc_offset as u16;
            registers.write(RegisterEnum::PC, val);
        } else {
            registers.write(RegisterEnum::PC, registers.read(RegisterEnum::try_from(base_reg as usize).expect("Invalid Base Register")));/* JSRR */
        }
    }

    




}

/// Sign-extends a value to the given bit width.
///
/// - `x`: The value to sign-extend.
/// - `bit_count`: The original bit width of the value.

fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
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

#[cfg(test)]
mod instruction_tests;
