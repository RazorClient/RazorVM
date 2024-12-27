//Integration test
use super::lc3::hardware::{Registers, RegisterEnum as Register, Memory};
use super::lc3::cpu::instruction::Instructions;
use super::hardware::flags::ConditionFlags;

#[test]
fn integration_test_add_register_mode() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(RegisterEnum::R0, 20); // R0 = 20
    registers.write(RegisterEnum::R1, 22); // R1 = 22

    // Encode ADD R2, R0, R1
    let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + R1 = 42
    assert_eq!(registers.read(RegisterEnum::R2), 42);
    // Verify condition flags are set to POS
    assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
}

#[test]
fn integration_test_add_immediate_mode() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(RegisterEnum::R0, 15); // R0 = 15

    // Encode ADD R2, R0, #10
    let instr = 0b0001_010_000_1_01010; // DR=R2, SR1=R0, imm5=10
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + 10 = 25
    assert_eq!(registers.read(RegisterEnum::R2), 25);
    // Verify condition flags are set to POS
    assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
}

#[test]
fn integration_test_add_result_zero() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

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
fn integration_test_add_overflow_negative() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers with values that cause negative overflow
    registers.write(RegisterEnum::R0, 0x8000); // R0 = -32768
    registers.write(RegisterEnum::R1, 0xFFFF); // R1 = -1

    // Encode ADD R2, R0, R1
    let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + R1 = -32768 + (-1) = 32767 (due to wrapping)
    assert_eq!(registers.read(RegisterEnum::R2), 0x7FFF);
    // Verify condition flags are set to POS
    assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
}

#[test]
fn integration_test_add_immediate_mode_negative() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(RegisterEnum::R0, 10); // R0 = 10

    // Encode ADD R2, R0, #-5 (imm5 = 0b111011 = -5)
    let instr = 0b0001_010_000_1_11101; // DR=R2, SR1=R0, imm5=-5
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + (-5) = 5
    assert_eq!(registers.read(RegisterEnum::R2), 5);
    // Verify condition flags are set to POS
    assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::POS.bits() as u16);
}

#[test]
fn integration_test_add_immediate_mode_zero() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(RegisterEnum::R0, 5); // R0 = 5

    // Encode ADD R2, R0, #-5 (imm5 = 0b111011 = -5)
    let instr = 0b0001_010_000_1_11101; // DR=R2, SR1=R0, imm5=-5
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + (-5) = 0
    assert_eq!(registers.read(RegisterEnum::R2), 0);
    // Verify condition flags are set to ZRO
    assert_eq!(registers.read(RegisterEnum::COND), ConditionFlags::ZRO.bits() as u16);
}


#[test]
fn integration_test_ldi_positive_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize PC to 0x2000
    registers.write(Register::PC, 0x2000);

    // Destination register R1
    let dr = Register::R1;

    // PCoffset9 = 0x002 (2)
    let pc_offset9 = 0x002;

    // Encode LDI R1, PCoffset9=2
    let instr = (0b1010 << 12) | (1 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R1, PCoffset9=2

    // Set memory at PC + 2 = 0x2002 to 0x3000 (addr2)
    memory.write(0x2002, 0x3000);

    // Set memory at addr2 = 0x3000 to 0x5555 (value to load)
    memory.write(0x3000, 0x5555);

    // Execute the LDI instruction
    Instructions::ldi(instr, &mut registers, &memory);

    // Verify R1 = 0x5555
    assert_eq!(registers.read(dr), 0x5555);

    // Verify condition flags are set to POS
    assert_eq!(registers.read(Register::COND), ConditionFlags::POS.bits() as u16);
}

/// Integration test for the LDI instruction with a negative value.
#[test]
fn integration_test_ldi_negative_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize PC to 0x2000
    registers.write(Register::PC, 0x2000);

    // Destination register R2
    let dr = Register::R2;

    // PCoffset9 = 0x001 (1)
    let pc_offset9 = 0x001;

    // Encode LDI R2, PCoffset9=1
    let instr = (0b1010 << 12) | (2 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R2, PCoffset9=1

    // Set memory at PC + 1 = 0x2001 to 0x3001 (addr2)
    memory.write(0x2001, 0x3001);

    // Set memory at addr2 = 0x3001 to 0x8000 (negative value)
    memory.write(0x3001, 0x8000);

    // Execute the LDI instruction
    Instructions::ldi(instr, &mut registers, &memory);

    // Verify R2 = 0x8000
    assert_eq!(registers.read(dr), 0x8000);

    // Verify condition flags are set to NEG
    assert_eq!(registers.read(Register::COND), ConditionFlags::NEG.bits() as u16);
}

/// Integration test for the LDI instruction with a zero value.
#[test]
fn integration_test_ldi_zero_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize PC to 0x2000
    registers.write(Register::PC, 0x2000);

    // Destination register R3
    let dr = Register::R3;

    // PCoffset9 = 0x003 (3)
    let pc_offset9 = 0x003;

    // Encode LDI R3, PCoffset9=3
    let instr = (0b1010 << 12) | (3 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R3, PCoffset9=3

    // Set memory at PC + 3 = 0x2003 to 0x3002 (addr2)
    memory.write(0x2003, 0x3002);

    // Set memory at addr2 = 0x3002 to 0x0000 (zero value)
    memory.write(0x3002, 0x0000);

    // Execute the LDI instruction
    Instructions::ldi(instr, &mut registers, &memory);

    // Verify R3 = 0x0000
    assert_eq!(registers.read(dr), 0x0000);

    // Verify condition flags are set to ZRO
    assert_eq!(registers.read(Register::COND), ConditionFlags::ZRO.bits() as u16);
}

/// Integration test for the LDI instruction with a negative PCoffset9 (backward addressing).
#[test]
fn integration_test_ldi_negative_pc_offset() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize PC to 0x2001
    registers.write(Register::PC, 0x2001);

    // Destination register R4
    let dr = Register::R4;

    // PCoffset9 = 0x1FF (-1 in 9-bit two's complement)
    let pc_offset9 = 0x1FF;

    // Encode LDI R4, PCoffset9=-1
    let instr = (0b1010 << 12) | (4 << 9) | (pc_offset9 & 0x1FF); // Opcode=1010 (LDI), DR=R4, PCoffset9=-1

    // Set memory at PC - 1 = 0x2000 to 0x3003 (addr2)
    memory.write(0x2000, 0x3003);

    // Set memory at addr2 = 0x3003 to 0x5678 (value to load)
    memory.write(0x3003, 0x5678);

    // Execute the LDI instruction
    Instructions::ldi(instr, &mut registers, &memory);

    // Verify R4 = 0x5678
    assert_eq!(registers.read(dr), 0x5678);

    // Verify condition flags are set to POS
    assert_eq!(registers.read(Register::COND), ConditionFlags::POS.bits() as u16);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_and_immediate_mode() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Set up initial register values
        registers.write(1, 0b10101010); // SR1

        // Instruction: AND R0, R1, #0b111
        let instr = 0b0101_000_001_1_00000111;

        // Execute the instruction
        bitwise_and(instr, &mut registers, &memory);

        // Verify result
        assert_eq!(registers.read(0), 0b00000010); // R0

        // Verify flags
        // Assuming update_flags is correct, verify ZRO flag
        assert_eq!(registers.read_cond_flags(), ConditionFlags::ZRO);
    }

    #[test]
    fn test_bitwise_and_register_mode() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Set up initial register values
        registers.write(1, 0b10101010); // SR1
        registers.write(2, 0b11110000); // SR2

        // Instruction: AND R0, R1, R2
        let instr = 0b0101_000_001_0_00000010;

        // Execute the instruction
        bitwise_and(instr, &mut registers, &memory);

        // Verify result
        assert_eq!(registers.read(0), 0b10100000); // R0

        // Verify flags
        // Assuming update_flags is correct, verify POS flag
        assert_eq!(registers.read_cond_flags(), ConditionFlags::POS);
    }

    #[test]
    fn test_bitwise_and_zero_result() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Set up initial register values
        registers.write(1, 0b10101010); // SR1
        registers.write(2, 0b01010101); // SR2

        // Instruction: AND R0, R1, R2
        let instr = 0b0101_000_001_0_00000010;

        // Execute the instruction
        bitwise_and(instr, &mut registers, &memory);

        // Verify result
        assert_eq!(registers.read(0), 0); // R0

        // Verify flags
        // Assuming update_flags is correct, verify ZRO flag
        assert_eq!(registers.read_cond_flags(), ConditionFlags::ZRO);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_and_immediate_mode() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Set up initial register values
        registers.write(1, 0b10101010); // SR1

        // Instruction: AND R0, R1, #0b111
        let instr = 0b0101_000_001_1_00000111;

        // Execute the instruction
        bitwise_and(instr, &mut registers, &memory);

        // Verify result
        assert_eq!(registers.read(0), 0b00000010); // R0

        // Verify flags
        // Assuming update_flags is correct, verify ZRO flag
        assert_eq!(registers.read_cond_flags(), ConditionFlags::ZRO);
    }

    #[test]
    fn test_bitwise_and_register_mode() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Set up initial register values
        registers.write(1, 0b10101010); // SR1
        registers.write(2, 0b11110000); // SR2

        // Instruction: AND R0, R1, R2
        let instr = 0b0101_000_001_0_00000010;

        // Execute the instruction
        bitwise_and(instr, &mut registers, &memory);

        // Verify result
        assert_eq!(registers.read(0), 0b10100000); // R0

        // Verify flags
        // Assuming update_flags is correct, verify POS flag
        assert_eq!(registers.read_cond_flags(), ConditionFlags::POS);
    }

    #[test]
    fn test_bitwise_and_zero_result() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        // Set up initial register values
        registers.write(1, 0b10101010); // SR1
        registers.write(2, 0b01010101); // SR2

        // Instruction: AND R0, R1, R2
        let instr = 0b0101_000_001_0_00000010;

        // Execute the instruction
        bitwise_and(instr, &mut registers, &memory);

        // Verify result
        assert_eq!(registers.read(0), 0); // R0

        // Verify flags
        // Assuming update_flags is correct, verify ZRO flag
        assert_eq!(registers.read_cond_flags(), ConditionFlags::ZRO);
    }
}
