//Integration test
use super::super::hardware::Flag::ConditionFlags;
use super::super::hardware::Memory::Memory;
use super::super::hardware::{RegisterEnum as Register, Registers};
use super::super::Instructions;


fn encode_br(n: bool, z: bool, p: bool, pc_offset9: i16) -> u16 {
    let opcode = 0b0000 << 12;
    let flags = ((n as u16) << 11) | ((z as u16) << 10) | ((p as u16) << 9);
    let offset = (pc_offset9 as u16) & 0x1FF; // Ensure 9 bits
    opcode | flags | offset
}


#[test]
fn integration_test_add_register_mode() {
    let mut registers = Registers::new();

    // Initialize registers
    registers.write(Register::R0, 20); // R0 = 20
    registers.write(Register::R1, 22); // R1 = 22

    // Encode ADD R2, R0, R1
    let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + R1 = 42
    assert_eq!(registers.read(Register::R2), 42);
    // Verify condition flags are set to POS
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

#[test]
fn integration_test_add_immediate_mode() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(Register::R0, 15); // R0 = 15

    // Encode ADD R2, R0, #10
    let instr = 0b0001_010_000_1_01010; // DR=R2, SR1=R0, imm5=10
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + 10 = 25
    assert_eq!(registers.read(Register::R2), 25);
    // Verify condition flags are set to POS
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

#[test]
fn integration_test_add_result_zero() {
    let mut registers = Registers::new();

    // Initialize registers
    registers.write(Register::R0, 5); // R0 = 5
    registers.write(Register::R1, -5i16 as u16); // R1 = -5

    // Encode ADD R2, R0, R1
    let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + R1 = 0
    assert_eq!(registers.read(Register::R2), 0);
    // Verify condition flags are set to ZRO
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::ZRO.bits() as u16
    );
}

#[test]
fn integration_test_add_overflow_negative() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers with values that cause negative overflow
    registers.write(Register::R0, 0x8000); // R0 = -32768
    registers.write(Register::R1, 0xFFFF); // R1 = -1

    // Encode ADD R2, R0, R1
    let instr = 0b0001_010_000_000_001; // DR=R2, SR1=R0, SR2=R1
    Instructions::add(instr, &mut registers);

    // Verify R2 = R0 + R1 = -32768 + (-1) = 32767 (due to wrapping)
    assert_eq!(registers.read(Register::R2), 0x7FFF);
    // Verify condition flags are set to POS
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

#[test]
fn test_add_registers_negative() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(Register::R1, (-15i16) as u16); // R1 = -15 -> 0xFFF1
    registers.write(Register::R2, (-10i16) as u16); // R2 = -10 -> 0xFFF6

    // Encode ADD R3, R1, R2
    let instr = 0b0001_011_001_000_010; // Opcode=0001 (ADD), DR=R3, SR1=R1, SR2=R2

    // Execute the ADD instruction
    Instructions::add(instr, &mut registers);

    // Verify R3 = R1 + R2 = -25 -> 0xFFE7
    assert_eq!(registers.read(Register::R3), 0xFFE7);
    // Verify condition flags are set to NEG
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );
}
#[test]
fn test_add_registers_zero() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(Register::R1, 10);
    registers.write(Register::R2, (-10i16) as u16); // R2 = -10 -> 0xFFF6

    // Encode ADD R3, R1, R2
    let instr = 0b0001_011_001_000_010; // Opcode=0001 (ADD), DR=R3, SR1=R1, SR2=R2

    // Execute the ADD instruction
    Instructions::add(instr, &mut registers);

    // Verify R3 = R1 + R2 = 0
    assert_eq!(registers.read(Register::R3), 0);
    // Verify condition flags are set to ZRO
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::ZRO.bits() as u16
    );
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
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
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
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );
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
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::ZRO.bits() as u16
    );
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
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

#[test]
fn integration_test_br_take_branch_n_flag() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to NEG
    registers.write(Register::COND, ConditionFlags::NEG.bits() as u16);

    // Encode BRn PCoffset9=1
    let instr = encode_br(true, false, false, 1); // BRn with PCoffset9=1

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Verify PC is updated correctly
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_add(1));
}

#[test]
fn integration_test_br_take_branch_z_flag() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to ZRO
    registers.write(Register::COND, ConditionFlags::ZRO.bits() as u16);

    // Encode BRz PCoffset9=2
    let instr = encode_br(false, true, false, 2); // BRz with PCoffset9=2

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Verify PC is updated correctly
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_add(2));
}

#[test]
fn integration_test_br_take_branch_p_flag() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to POS
    registers.write(Register::COND, ConditionFlags::POS.bits() as u16);

    // Encode BRp PCoffset9=3
    let instr = encode_br(false, false, true, 3); // BRp with PCoffset9=3

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Verify PC is updated correctly
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_add(3));
}

#[test]
fn integration_test_br_take_branch_multiple_flags() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to ZRO
    registers.write(Register::COND, ConditionFlags::ZRO.bits() as u16);

    // Encode BRnz PCoffset9=4 (branch on Negative or Zero)
    let instr = encode_br(true, true, false, 4); // BRnz with PCoffset9=4

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Since ZRO flag is set and BRnz checks for N or Z, branch is taken
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_add(4));
}

#[test]
fn integration_test_br_not_take_branch_multiple_flags() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to POS
    registers.write(Register::COND, ConditionFlags::POS.bits() as u16);

    // Encode BRnz PCoffset9=5 (branch on Negative or Zero)
    let instr = encode_br(true, true, false, 5); // BRnz with PCoffset9=5

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Since N and Z flags are not set, branch is not taken
    assert_eq!(registers.read(Register::PC), current_pc);
}

#[test]
fn integration_test_br_no_flags() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to POS
    registers.write(Register::COND, ConditionFlags::POS.bits() as u16);

    // Encode BR PCoffset9=6 with no flags (n=0, z=0, p=0)
    let instr = encode_br(false, false, false, 6); // BR with no flags

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Since no flags are specified, branch is never taken
    assert_eq!(registers.read(Register::PC), current_pc);
}

#[test]
fn integration_test_br_backward_branch() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to ZRO
    registers.write(Register::COND, ConditionFlags::ZRO.bits() as u16);

    // Encode BRz PCoffset9=-1 (0x1FF in two's complement)
    let instr = encode_br(false, true, false, -1); // BRz with PCoffset9=-1

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Verify PC is updated correctly (backwards by 1)
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_sub(1));
}

#[test]
fn integration_test_br_PC_wrap_around() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to POS
    registers.write(Register::COND, ConditionFlags::POS.bits() as u16);

    // Set PC near the maximum value
    registers.write(Register::PC, 0xFFFE);

    // Encode BRp PCoffset9=3 (which should wrap PC to 0x0001)
    let instr = encode_br(false, false, true, 3); // BRp with PCoffset9=3

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Expected PC: 0xFFFE + 3 = 0x0001 (wrap around)
    assert_eq!(registers.read(Register::PC), 0x0001);
}

#[test]
fn integration_test_br_max_positive_offset() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to POS
    registers.write(Register::COND, ConditionFlags::POS.bits() as u16);

    // Encode BRp PCoffset9=255 (maximum positive for 9-bit signed)
    let instr = encode_br(false, false, true, 255); // BRp with PCoffset9=255

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Verify PC is updated correctly
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_add(255));
}

#[test]
fn integration_test_br_max_negative_offset() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to N
    registers.write(Register::COND, ConditionFlags::NEG.bits() as u16);

    // Encode BRn PCoffset9=-255
    let instr = encode_br(true, false, false, -255); // BRn with PCoffset9=-255

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Verify PC is updated correctly
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_sub(255));
}

#[test]
fn integration_test_br_all_flags_set() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to NZP (all flags set)
    registers.write(Register::COND, ConditionFlags::NEG.bits() as u16
                                 | ConditionFlags::ZRO.bits() as u16
                                 | ConditionFlags::POS.bits() as u16);

    // Encode BRnzp PCoffset9=1
    let instr = encode_br(true, true, true, 1); // BRnzp with PCoffset9=1

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Since all flags are set, branch is taken
    assert_eq!(registers.read(Register::PC), current_pc.wrapping_add(1));
}

#[test]
fn integration_test_br_no_matching_flags() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize condition flags to ZRO
    registers.write(Register::COND, ConditionFlags::ZRO.bits() as u16);

    // Encode BRn PCoffset9=2 (checking Negative flag)
    let instr = encode_br(true, false, false, 2); // BRn with PCoffset9=2

    // Current PC
    let current_pc = registers.read(Register::PC);

    // Execute the BR instruction
    Instructions::br(instr, &mut registers);

    // Since Negative flag is not set, branch is not taken
    assert_eq!(registers.read(Register::PC), current_pc);
}

fn encode_not(dr: u16, sr: u16) -> u16 {
    (0b1001 << 12) | (dr << 9) | (sr << 6) | 0x3F
}

#[test]
fn integration_test_not_positive_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with a positive value
    registers.write(Register::R1, 0x1234); // R1 = 0x1234

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0xEDCB
    assert_eq!(registers.read(Register::R2), !0x1234);

    // Verify condition flags are set to NEG (since ~0x1234 = 0xEDCB, which is negative)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );
}

#[test]
fn integration_test_not_negative_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with a negative value
    registers.write(Register::R1, 0x8000); // R1 = 0x8000 (-32768)

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0x7FFF
    assert_eq!(registers.read(Register::R2), !0x8000);

    // Verify condition flags are set to POS (since ~0x8000 = 0x7FFF, which is positive)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

#[test]
fn integration_test_not_zero_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with zero
    registers.write(Register::R1, 0x0000); // R1 = 0x0000

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0xFFFF
    assert_eq!(registers.read(Register::R2), !0x0000);

    // Verify condition flags are set to NEG (since ~0x0000 = 0xFFFF, which is negative)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );
}

#[test]
fn integration_test_not_max_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with maximum value
    registers.write(Register::R1, 0xFFFF); // R1 = 0xFFFF (-1)

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0x0000
    assert_eq!(registers.read(Register::R2), !0xFFFF);

    // Verify condition flags are set to ZRO (since ~0xFFFF = 0x0000)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::ZRO.bits() as u16
    );
}

#[test]
fn integration_test_not_min_value() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with minimum value
    registers.write(Register::R1, 0x8000); // R1 = 0x8000 (-32768)

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0x7FFF
    assert_eq!(registers.read(Register::R2), !0x8000);

    // Verify condition flags are set to POS (since ~0x8000 = 0x7FFF, which is positive)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

#[test]
fn integration_test_not_all_flags_set() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with a value that sets all flags after NOT
    registers.write(Register::R1, 0x0001); // R1 = 0x0001

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0xFFFE
    assert_eq!(registers.read(Register::R2), !0x0001);

    // Verify condition flags are set to NEG (since 0xFFFE is negative)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );
}

#[test]
fn integration_test_not_registers_unchanged() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize register R1 with a value
    registers.write(Register::R1, 0x0F0F); // R1 = 0x0F0F
    // Initialize register R3 with another value
    registers.write(Register::R3, 0xAAAA); // R3 = 0xAAAA

    // Encode NOT R2, R1
    let instr = encode_not(2, 1); // DR=R2, SR=R1

    // Execute the NOT instruction
    Instructions::bitwise_not(instr, &mut registers);

    // Verify R2 = ~R1 = 0xF0F0
    assert_eq!(registers.read(Register::R2), !0x0F0F);

    // Verify other registers remain unchanged
    assert_eq!(registers.read(Register::R3), 0xAAAA);

    // Verify condition flags are set to NEG (since 0xF0F0 is negative)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );
}

#[test]
fn integration_test_not_multiple_operations() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    // Initialize registers
    registers.write(Register::R1, 0x00FF); // R1 = 0x00FF
    registers.write(Register::R2, 0xFF00); // R2 = 0xFF00

    // Encode NOT R3, R1
    let instr1 = encode_not(3, 1); // DR=R3, SR=R1

    // Execute the first NOT instruction
    Instructions::bitwise_not(instr1, &mut registers);

    // Verify R3 = ~R1 = 0xFF00
    assert_eq!(registers.read(Register::R3), !0x00FF);

    // Verify condition flags are set to NEG (since 0xFF00 is negative)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::NEG.bits() as u16
    );

    // Encode NOT R4, R2
    let instr2 = encode_not(4, 2); // DR=R4, SR=R2

    // Execute the second NOT instruction
    Instructions::bitwise_not(instr2, &mut registers);

    // Verify R4 = ~R2 = 0x00FF
    assert_eq!(registers.read(Register::R4), !0xFF00);

    // Verify condition flags are set to POS (since 0x00FF is positive)
    assert_eq!(
        registers.read(Register::COND),
        ConditionFlags::POS.bits() as u16
    );
}

// fn encode_jmp(base_r: u16) -> u16 {
//     (0b1100 << 12) | (base_r << 6)
// }

// #[test]
// fn integration_test_jmp_register_mode() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize Base Register R1 with a positive address
//     registers.write(Register::R1, 0x3000); // R1 = 0x3000

//     // Encode JMP R1
//     let instr = encode_jmp(1); // JMP R1

//     // Current PC
//     let current_pc = registers.read(Register::PC);

//     // Execute the JMP instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC is set to R1's value
//     assert_eq!(registers.read(Register::PC), 0x3000);

//     // Verify other registers remain unchanged (e.g., R0 is 0)
//     assert_eq!(registers.read(Register::R0), 0x0000);
//     assert_eq!(registers.read(Register::R2), 0x0000);
//     assert_eq!(registers.read(Register::R3), 0x0000);
//     assert_eq!(registers.read(Register::R4), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);
//     assert_eq!(registers.read(Register::R7), 0x0000);

//     // Verify condition flags remain unchanged (assuming ZRO by default)
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

// #[test]
// fn integration_test_jmp_ret() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize R7 with a return address
//     registers.write(Register::R7, 0x4000); // R7 = 0x4000

//     // Encode RET (JMP R7)
//     let instr = encode_jmp(7); // RET is equivalent to JMP R7

//     // Current PC
//     let current_pc = registers.read(Register::PC);

//     // Execute the JMP instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC is set to R7's value
//     assert_eq!(registers.read(Register::PC), 0x4000);

//     // Verify other registers remain unchanged
//     assert_eq!(registers.read(Register::R0), 0x0000);
//     assert_eq!(registers.read(Register::R1), 0x0000);
//     assert_eq!(registers.read(Register::R2), 0x0000);
//     assert_eq!(registers.read(Register::R3), 0x0000);
//     assert_eq!(registers.read(Register::R4), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);

//     // Verify condition flags remain unchanged
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

// #[test]
// fn integration_test_jmp_same_address() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize R2 with the current PC value
//     registers.write(Register::PC, 0x5000); // PC = 0x5000
//     registers.write(Register::R2, 0x5000); // R2 = 0x5000

//     // Encode JMP R2
//     let instr = encode_jmp(2); // JMP R2

//     // Execute the JMP instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC remains unchanged
//     assert_eq!(registers.read(Register::PC), 0x5000);

//     // Verify other registers remain unchanged
//     assert_eq!(registers.read(Register::R0), 0x0000);
//     assert_eq!(registers.read(Register::R1), 0x0000);
//     assert_eq!(registers.read(Register::R3), 0x0000);
//     assert_eq!(registers.read(Register::R4), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);
//     assert_eq!(registers.read(Register::R7), 0x0000);

//     // Verify condition flags remain unchanged
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

// #[test]
// fn integration_test_jmp_other_registers_unchanged() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize registers
//     registers.write(Register::R3, 0x6000); // R3 = 0x6000
//     registers.write(Register::R4, 0x7000); // R4 = 0x7000

//     // Encode JMP R3
//     let instr = encode_jmp(3); // JMP R3

//     // Current PC
//     let current_pc = registers.read(Register::PC);

//     // Execute the JMP instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC is set to R3's value
//     assert_eq!(registers.read(Register::PC), 0x6000);

//     // Verify R4 remains unchanged
//     assert_eq!(registers.read(Register::R4), 0x7000);

//     // Verify other registers remain unchanged
//     assert_eq!(registers.read(Register::R0), 0x0000);
//     assert_eq!(registers.read(Register::R1), 0x0000);
//     assert_eq!(registers.read(Register::R2), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);
//     assert_eq!(registers.read(Register::R7), 0x0000);

//     // Verify condition flags remain unchanged
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

// #[test]
// fn integration_test_jmp_multiple_operations() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize registers
//     registers.write(Register::R1, 0x3000); // R1 = 0x3000
//     registers.write(Register::R2, 0x4000); // R2 = 0x4000

//     // Encode JMP R1
//     let instr1 = encode_jmp(1); // JMP R1

//     // Execute the first JMP instruction
//     Instructions::jmp(instr1, &mut registers);

//     // Verify PC is set to R1's value
//     assert_eq!(registers.read(Register::PC), 0x3000);

//     // Encode JMP R2
//     let instr2 = encode_jmp(2); // JMP R2

//     // Execute the second JMP instruction
//     Instructions::jmp(instr2, &mut registers);

//     // Verify PC is set to R2's value
//     assert_eq!(registers.read(Register::PC), 0x4000);

//     // Verify other registers remain unchanged
//     assert_eq!(registers.read(Register::R0), 0x0000);
//     assert_eq!(registers.read(Register::R3), 0x0000);
//     assert_eq!(registers.read(Register::R4), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);
//     assert_eq!(registers.read(Register::R7), 0x0000);

//     // Verify condition flags remain unchanged
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

// #[test]
// fn integration_test_jmp_registers_unchanged() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize registers
//     registers.write(Register::R1, 0x3000); // R1 = 0x3000
//     registers.write(Register::R2, 0x4000); // R2 = 0x4000
//     registers.write(Register::R3, 0x5000); // R3 = 0x5000

//     // Encode JMP R1
//     let instr = encode_jmp(1); // JMP R1

//     // Execute the JMP instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC is set to R1's value
//     assert_eq!(registers.read(Register::PC), 0x3000);

//     // Verify other registers remain unchanged
//     assert_eq!(registers.read(Register::R2), 0x4000);
//     assert_eq!(registers.read(Register::R3), 0x5000);
//     assert_eq!(registers.read(Register::R4), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);
//     assert_eq!(registers.read(Register::R7), 0x0000);

//     // Verify condition flags remain unchanged
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

// #[test]
// fn integration_test_jmp_ret_multiple_returns() {
//     let mut registers = Registers::new();
//     let mut memory = Memory::new();

//     // Initialize R7 with multiple return addresses
//     registers.write(Register::R7, 0x7000); // First return address
//     // Assume a mechanism to handle multiple returns, e.g., stacking (not shown here)

//     // Encode RET (JMP R7)
//     let instr = encode_jmp(7); // RET is equivalent to JMP R7

//     // Execute the first RET instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC is set to R7's first value
//     assert_eq!(registers.read(Register::PC), 0x7000);

//     // Update R7 for the second return address
//     registers.write(Register::R7, 0x8000); // Second return address

//     // Execute the second RET instruction
//     Instructions::jmp(instr, &mut registers);

//     // Verify PC is set to R7's second value
//     assert_eq!(registers.read(Register::PC), 0x8000);

//     // Verify other registers remain unchanged
//     assert_eq!(registers.read(Register::R0), 0x0000);
//     assert_eq!(registers.read(Register::R1), 0x0000);
//     assert_eq!(registers.read(Register::R2), 0x0000);
//     assert_eq!(registers.read(Register::R3), 0x0000);
//     assert_eq!(registers.read(Register::R4), 0x0000);
//     assert_eq!(registers.read(Register::R5), 0x0000);
//     assert_eq!(registers.read(Register::R6), 0x0000);

//     // Verify condition flags remain unchanged
//     assert_eq!(
//         registers.read(Register::COND),
//         ConditionFlags::ZRO.bits() as u16
//     );
// }

fn encode_jsr(pc_offset11: i16) -> u16 {
    let opcode = 0b0100 << 12; // Opcode for JSR
    let long_flag = 1 << 11;    // long_flag set to 1 for JSR
    let offset = (pc_offset11 as u16) & 0x07FF; // Ensure 11 bits
    opcode | long_flag | offset
}

/// Helper function to encode the JSRR instruction.
///
/// - `base_reg`: Base register number (0-7).
///
/// Returns the encoded 16-bit instruction.
fn encode_jsrr(base_reg: usize) -> u16 {
    let opcode = 0b0100 << 12; // Opcode for JSRR
    let long_flag = 0 << 11;    // long_flag set to 0 for JSRR
    let base = (base_reg & 0x7) << 6; // Ensure base_reg is 3 bits
    (opcode | long_flag | base) as u16
}


// not in the mood maybe later 
