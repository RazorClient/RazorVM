use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::lc3::sys::file;
pub const MEMORY_SIZE: usize = 1 << 16;

#[derive(Copy)]
pub struct Memory {
    data: [u16; MEMORY_SIZE],
}

pub enum MemoryMappedReg {
    /// keyboard status: The KBSR indicates whether a key has been pressed
    Kbsr = 0xFE00, /* keyboard status */
    /// keyboard data: The KBDR identifies which key was pressed
    Kbdr = 0xFE02, /* keyboard data */
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 65536] }
    }

    pub fn read(&mut self, address: usize) -> u16 {
        let effective_address = address & 0xFFFF; // Wrap within 16-bit range

        // match effective_address {
        //     kbsr => {
        //         if check_key() {
        //             self.data[MemoryMappedReg::Kbsr as usize] = 1 << 15; // Set the high bit to indicate key press
        //             self.data[MemoryMappedReg::Kbdr as usize] = 1;
        //             // getchar::get_char() as u16; // Update keyboard data register
        //         } else {
        //             self.data[kbsr] = 0; // Clear the high bit if no key press
        //         }
        //     }
        //     _ => {} // Normal memory access
        // }
        if effective_address < self.data.len() {
            self.data[effective_address]
        } else {
            panic!("Memory read out of bounds at address: {:#X}", address);
        }
    }

    pub fn write(&mut self, address: usize, value: u16) {
        let effective_address = address & 0xFFFF; // Wrap within 16-bit range
        if effective_address < self.data.len() {
            self.data[effective_address] = value;
        } else {
            panic!("Memory write out of bounds at address: {:#X}", address);
        }
    }
}

impl Clone for Memory {
    fn clone(&self) -> Memory {
        *self
    }
}

fn check_key()-> bool{
   false
}

fn getchar() -> char {
    '\0' 
}
#[cfg(test)]
mod memory_test {
    use super::*;

    const EXPECTED_MEMORY_SIZE: usize = 65536;
    #[test]
    fn memory_size() {
        let memory = Memory::new();
        assert_eq!(memory.data.len(), EXPECTED_MEMORY_SIZE);
    }

    #[test]
    fn memory_size_constant() {
        assert_eq!(MEMORY_SIZE, EXPECTED_MEMORY_SIZE);
    }

    #[test]
    fn test_memory_read_write() {
        let mut memory = Memory::new();
        memory.write(0x1234, 42);
        assert_eq!(memory.read(0x1234), 42);
    }

    #[test]
    fn test_memory_wraparound_read() {
        let mut memory = Memory::new();
        memory.write(0x0000, 42);
        assert_eq!(memory.read(0x1_0000), 42); 
    }
    #[test]
    fn test_file_read() {

        let result = file::read_image("Static/out.obj");
        assert!(result.is_ok());
    }
}
