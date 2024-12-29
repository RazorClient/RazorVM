use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MEMORY_SIZE: usize = 1 << 16;
#[derive(Copy)]
pub struct Memory {
    data: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 65536] }
    }

    pub fn read(&self, address: usize) -> u16 {
        let effective_address = address & 0xFFFF; // Wrap within 16-bit range
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
    pub fn load_program<P: AsRef<Path>>(
        &mut self,
        filename: P,
        start_addr: u16,
    ) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        //     let mut addr = start_addr as usize;
        //     for chunk in buffer.chunks(2) {
        //         if chunk.len() == 2 {
        //             let word = u16::from_be_bytes([chunk[0], chunk[1]]);
        //             self.data[addr] = word;
        //             addr += 1;
        //         }
        //     }
        Ok(())
    }
}
impl Clone for Memory {
    fn clone(&self) -> Memory {
        *self
    }
}

pub enum MemoryMappedReg {
    /// keyboard status: The KBSR indicates whether a key has been pressed
    Kbsr = 0xFE00, /* keyboard status */
    /// keyboard data: The KBDR identifies which key was pressed
    Kbdr = 0xFE02, /* keyboard data */
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
        memory.write(0x0000, 42); // Write value at address 0x0000
        assert_eq!(memory.read(0x1_0000), 42); // Read from address 0x1_0000 (wraps to 0x0000)
    }
        
}
