use std::io::{self};

use crate::lc3::cpu::Instructions;
use crate::lc3::hardware::Memory::Memory;
use crate::lc3::hardware::Reg;
// use  crate::lc3::
// use  crate::lc3::
// use  crate::lc3::
pub struct LC3 {
    memory: Memory,
    registers: Reg::Registers,
}

impl LC3 {
    /// Create a new LC3 instance
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Reg::Registers::new(),
        }
    }

    /// Load a program image into memory
    pub fn load_image(&mut self, path: &str) -> io::Result<()> {
        // let mut file = File::open(path)?;
        // let mut buffer = Vec::new();
        // file.read_to_end(&mut buffer)?;

        // self.memory.load_binary(&buffer)?;
        Ok(())
    }

    /// Run the VM main loop
    pub fn main_loop(&mut self) {
        // // Initialize registers
        // // self.registers.write(R_COND, ConditionFlags::ZRO as u16);
        // // self.registers.write(R_PC, 0x3000);

        let running = true;

        let instructions = Instructions;

        // while running {
        //     let pc = self.registers.read(R_PC);
        //     let instr = self.memory.read(pc as usize);
        //     self.registers.write(R_PC, pc.wrapping_add(1));

        //     let opcode = Decoder::decode(instr);

        //     match opcode {
        //         instruction::OpCode::ADD => instructions.add(instr, &mut self.registers, &mut self.memory),
        //         // instruction::OpCode::AND => instruction::and(&mut self.registers, &mut self.memory, instr),
        //         // instruction::OpCode::NOT => instruction::not(&mut self.registers, instr),
        //         // OP_BR => instruction::br(&mut self.registers, instr),
        //         // OP_JMP => instruction::jmp(&mut self.registers, instr),
        //         // OP_JSR => instruction::jsr(&mut self.registers, instr),
        //         // OP_LD => instruction::ld(&mut self.registers, &mut self.memory, instr),
        //         // OP_LDI => instruction::ldi(&mut self.registers, &mut self.memory, instr),
        //         // OP_LDR => instruction::ldr(&mut self.registers, &mut self.memory, instr),
        //         // OP_LEA => instruction::lea(&mut self.registers, instr),
        //         // OP_ST => instruction::st(&mut self.registers, &mut self.memory, instr),
        //         // OP_STI => instruction::sti(&mut self.registers, &mut self.memory, instr),
        //         // OP_STR => instruction::str(&mut self.registers, &mut self.memory, instr),
        //         // OP_TRAP => {
        //         //     if !instruction::trap(&mut self.registers, &mut self.memory, instr) {
        //         //         running = false;
        //         //     }
        //         // }
        //         _ => {
        //             eprintln!("Unknown or unsupported opcode encountered: {:04X}", opcode);
        //             running = false;
        //         }
        //     }
        println!("VM execution completed.");
    }

    //     pub fn execute_instruction(&mut self, instr: u16) {
    //         // Decode the instruction
    //         if let Some(opcode) = Decoder::decode(instr) {
    //             match opcode {
    //                 OpCode::ADD => {
    //                     // Call execute_add from Instructions
    //                     Instructions::add(instr, &mut self.registers);
    //                 }
    //                 // Handle other opcodes...
    //                 _ => {}
    //             }
    //         }
    //     }
}
