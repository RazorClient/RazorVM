use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

pub const MEMORY_SIZE: usize = 1 << 16;

pub struct Memory {
    data: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 65536],
        }
    }

    pub fn read(&self, address: usize) -> u16 {
        if address < self.data.len() {
            self.data[address]
        } else {
            panic!("Memory read out of bounds at address: {:#X}", address);
        }
    }

    pub fn write(&mut self, address: usize, value: u16) {
        if address < self.data.len() {
            self.data[address] = value;
        } else {
            panic!("Memory write out of bounds at address: {:#X}", address);
        }
    }
    pub fn load_program<P: AsRef<Path>>(&mut self, filename: P, start_addr: u16) -> std::io::Result<()> {
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




#[test]
fn test_memory_read_write() {
    let mut memory = Memory::new();
    memory.write(0x1234, 42);
    assert_eq!(memory.read(0x1234), 42);
}

#[test]
#[should_panic]
fn test_memory_out_of_bounds_read() {
    let memory = Memory::new();
    memory.read(0x1_0000); // Invalid address
}
