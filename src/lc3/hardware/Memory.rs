pub struct Memory {
    pub data: [u16; 1 << 16],
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
