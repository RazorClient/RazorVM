use crate::lc3::hardware::{
    Memory::Memory,
    Reg::{RegisterEnum, Registers},
};
use std::io::{self, Read, Write};
use std::process;

/// Trap codes for the LC-3.
pub enum TrapCode {
    Getc = 0x20,  // Get character from keyboard
    Out = 0x21,   // Output a character
    Puts = 0x22,  // Output a word string
    In = 0x23,    // Get character with echo
    Putsp = 0x24, // Output a byte string
    Halt = 0x25,  // Halt the program
}

/// Executes a TRAP instruction.
/// - `instr`: The 16-bit LC-3 instruction word.
/// - `registers`: The mutable reference to the `Registers` struct.
/// - `memory`: The mutable reference to the `Memory` struct.
pub fn trap(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    // Save PC to R7 for return address
    let pc = registers.read(RegisterEnum::PC);
    registers.write(RegisterEnum::R7, pc);

    match instr & 0xFF {
        0x20 => {
            // TRAP GETC: Get a single ASCII character
            let input = io::stdin()
                .lock()
                .bytes()
                .next()
                .expect("No character input")
                .unwrap();
            registers.write(RegisterEnum::R0, input as u16);
        }
        0x21 => {
            // TRAP OUT: Output a single character
            let char_output = (registers.read(RegisterEnum::R0) as u8) as char;
            print!("{}", char_output);
            io::stdout().flush().expect("Failed to flush stdout");
        }
        0x22 => {
            // TRAP PUTS: Output a word string
            let mut address = registers.read(RegisterEnum::R0) as usize;
            loop {
                let word = memory.read(address);
                if word == 0 {
                    break;
                }
                // Only lower 8 bits are used for the character.
                let c = (word & 0xFF) as u8 as char;
                print!("{}", c);
                address += 1;
            }
            io::stdout().flush().expect("Failed to flush stdout (PUTS).");
        }
        0x23 => {
            // TRAP IN: Get a single character with echo
            print!("Enter a character: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let input = io::stdin()
                .lock()
                .bytes()
                .next()
                .expect("No character input")
                .unwrap();
            let char_output = input as char;
            print!("{}", char_output);
            registers.write(RegisterEnum::R0, input as u16);
            io::stdout().flush().expect("Failed to flush stdout after echo (IN).");
        }
        0x24 => {
            // TRAP PUTSP: Output a byte string
            let mut address = registers.read(RegisterEnum::R0) as usize;
            while memory.read(address) != 0 {
                let word = memory.read(address);
                let char1 = (word & 0xFF) as u8 as char;
                print!("{}", char1);
                let char2 = (word >> 8) as u8 as char;
                if char2 != '\0' {
                    print!("{}", char2);
                }
                address += 1;
            }
            io::stdout().flush().expect("Failed to flush stdout");
        }
        0x25 => {
            // TRAP HALT: Halt the program
            println!("HALT");
            process::exit(0);
        }
        _ => {
            // Unknown trap code
            panic!("Unknown TRAP code: {:#X}", instr & 0xFF);
        }
    }
}
