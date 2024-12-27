//Integration test
use super::lc3::hardware::{Registers, Register, Memory};
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
