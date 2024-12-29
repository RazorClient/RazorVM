# LC-3 Instructions Module

This module provides implementations for the LC-3 (Little Computer 3) instruction set, enabling the simulation of LC-3 assembly programs. Each instruction is represented as a method that executes the operation, modifies the register and memory states, and updates condition flags as necessary.


## Instructions Implemented

### Arithmetic and Logic

#### ADD
- **Format**:
  ```
  15        12 11        9 8        6 5        4         0
  +------------+------------+----------+-------------------+
  |  Opcode    |   DR       |   SR1    | Mode |  Operand    |
  +------------+------------+----------+-------------------+
  ```
- **Description**:
  - Adds two registers or a register and an immediate value.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn add(instr: u16, registers: &mut Registers)
  ```

#### BITWISE AND
- **Format**:
  ```
  15        12 11        9 8        6 5        4         0
  +------------+------------+----------+-------------------+
  |  Opcode    |   DR       |   SR1    | Mode |  Operand    |
  +------------+------------+----------+-------------------+
  ```
- **Description**:
  - Performs a bitwise AND between two registers or a register and an immediate value.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn bitwise_and(instr: u16, registers: &mut Registers)
  ```

#### NOT
- **Description**:
  - Performs a bitwise NOT on a source register and stores the result in a destination register.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn bitwise_not(instr: u16, registers: &mut Registers)
  ```

---

### Memory Access

#### LD (Load)
- **Format**:
  ```
  15        12 11        9 8                         0
  +------------+------------+---------------------------+
  |   Opcode   | Destination |        PCoffset9         |
  +------------+------------+---------------------------+
  ```
- **Description**:
  - Loads a value from memory using a PC-relative offset.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn ld(instr: u16, registers: &mut Registers, memory: &Memory)
  ```

#### LDI (Load Indirect)
- **Description**:
  - Loads a value indirectly from memory using two levels of indirection.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn ldi(instr: u16, registers: &mut Registers, memory: &Memory)
  ```

#### LEA (Load Effective Address)
- **Description**:
  - Loads an effective address into a register based on a PC-relative offset.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn lea(instr: u16, registers: &mut Registers)
  ```

#### LDR (Load Register)
- **Description**:
  - Loads a value from memory at an address computed from a base register and an offset.
  - Updates condition flags.
- **Signature**:
  ```rust
  pub fn ldr(instr: u16, registers: &mut Registers, memory: &Memory)
  ```

#### ST (Store)
- **Description**:
  - Stores a value from a register into memory at a PC-relative address.
- **Signature**:
  ```rust
  pub fn st(instr: u16, registers: &mut Registers, memory: &mut Memory)
  ```

#### STR (Store Register)
- **Description**:
  - Stores a value from a register into memory at an address computed from a base register and an offset.
- **Signature**:
  ```rust
  pub fn str(instr: u16, registers: &mut Registers, memory: &mut Memory)
  ```

#### STI (Store Indirect)
- **Description**:
  - Stores a value indirectly into memory using two levels of indirection.
- **Signature**:
  ```rust
  pub fn sti(instr: u16, registers: &mut Registers, memory: &mut Memory)
  ```

---

### Control Flow

#### BR (Branch)
- **Description**:
  - Checks condition flags and branches to a PC-relative address if conditions are met.
- **Signature**:
  ```rust
  pub fn br(instr: u16, registers: &mut Registers)
  ```

#### JMP (Jump)
- **Description**:
  - Sets the PC to the value in a base register. Also handles the RET instruction.
- **Signature**:
  ```rust
  pub fn jmp(instr: u16, registers: &mut Registers)
  ```

#### JSR (Jump to Subroutine)
- **Description**:
  - Stores the current PC in R7 and jumps to a PC-relative or base register address.
- **Signature**:
  ```rust
  pub fn jsr(instr: u16, registers: &mut Registers)
  ```

#### TRAP
- **Description**:
  - Executes a TRAP routine for system calls.
- **Signature**:
  ```rust
  pub fn trap(instr: u16, registers: &mut Registers, memory: &mut Memory)
  ```


## Utility Functions

### Sign Extension
- **Description**:
  - Extends a value to 16 bits, preserving the sign.
- **Signature**:
  ```rust
  fn sign_extend(x: u16, bit_count: usize) -> u16
  ```

### Register Extraction
- **Description**:
  - Extracts a register identifier from an instruction.
- **Signature**:
  ```rust
  fn extract_register(instr: u16, shift: usize) -> RegisterEnum
  ```








# LC-3 CPU Decode Module

The decode module is a critical component of the LC-3 CPU. It is responsible for interpreting the binary instruction words of the LC-3 architecture by extracting and mapping opcodes to their corresponding operations.



## Responsibilities

- **Opcode Extraction**: Identifies and extracts the opcode (top 4 bits) from a 16-bit instruction.
- **Opcode Mapping**: Matches the extracted opcode to the corresponding operation using the `OpCode` enum.
- **Error Handling**: Validates the extracted opcode and identifies invalid opcodes using the `OpCodeError` enum.



## Key Functions

### `extract_op_code`
Extracts the opcode from the top 4 bits of a 16-bit instruction and returns the corresponding `OpCode`.

- **Input**: A 16-bit binary instruction.
- **Output**: A `Result` wrapping an `OpCode` or an `OpCodeError`.
- **Purpose**: This function provides a mechanism to decode LC-3 instructions into operations that can be executed by the CPU.

### `execute_instruction`
Handles the execution of LC-3 instructions by decoding the opcode and dispatching the corresponding operation.

- **Input**: 
  - A 16-bit binary instruction.
  - A mutable reference to the LC-3 `Registers`.
  - A mutable reference to the LC-3 `Memory`.
- **Operation**: 
  - Decodes the instruction using `extract_op_code`.
  - Matches the opcode to its corresponding handler in the `Instructions` module.
  - Executes the identified operation, updating registers and memory as needed.
- **Error Handling**: Validates opcodes and logs errors for invalid instructions to prevent undefined behavior.