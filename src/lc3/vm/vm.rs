use std::io::{self};

use crate::lc3::cpu::instruction::Instructions;
use crate::lc3::hardware::Memory::Memory;
use crate::lc3::hardware::Reg::Registers;
// use  crate::lc3::
// use  crate::lc3::
// use  crate::lc3::
pub struct LC3 {
    memory: Memory,
    registers:Registers,
}

impl LC3 {
    /// Create a new LC3 instance
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
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


        println!("VM execution completed.");
    }

}
