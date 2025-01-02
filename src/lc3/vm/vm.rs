use std::io::{self};

use crate::lc3::cpu::instruction::Instructions;
use crate::lc3::cpu::decode::execute_instruction;
use crate::lc3::hardware::Memory::{Memory,MEMORY_SIZE};
use crate::lc3::hardware::Reg::{Registers,RegisterEnum};
use crate::lc3::hardware::Flag::ConditionFlags;
use crate::lc3::sys::file::read_image;
// use  crate::lc3::
// use  crate::lc3::
// use  crate::lc3::
pub struct LC3 {
    memory: Memory,
    registers: Registers,
}

impl LC3 {
    /// Create a new LC3 instance
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    pub fn load_image(&mut self, image_path: &str)->io::Result<()>{
        //load the file into a new Memory instance
        let mut loaded_memory = read_image(image_path)?;

        // Merge the loaded memory into the VM's existing memory
        for address in 0..MEMORY_SIZE {
            let value = loaded_memory.read(address);
            self.memory.write(address, value);
        }

        Ok(())
    }
    /// Run the VM main loop
    pub fn run(&mut self) {
        // Initialize registers
        self.registers.write(RegisterEnum::COND, ConditionFlags::ZRO.bits());
        self.registers.write(RegisterEnum::PC, 0x3000);
        println!("Registers Init");

        let running = true;

        let instructions = Instructions;

        while running {
        // Fetch the program counter (PC)
        let pc = self.registers.read(RegisterEnum::PC);
        // Fetch the instruction from memory
        let instr = self.memory.read(pc as usize);
        // Increment the PC
        self.registers.write(RegisterEnum::PC, pc.wrapping_add(1));
        // Print the instruction being executed
        println!("Executing instruction: {}", instr);
        // Decode and execute the instruction
        execute_instruction(instr, &mut self.registers, &mut self.memory);

        }
        println!("VM execution completed.");
        
    }
}
