use std::io::{self};

use crate::lc3::cpu::instruction::Instructions;
use crate::lc3::hardware::Memory::{Memory,MEMORY_SIZE};
use crate::lc3::hardware::Reg::Registers;
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
                  // Use the `read_image` function to load the file into a new Memory instance
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

        println!("VM execution completed.");
    }
}
