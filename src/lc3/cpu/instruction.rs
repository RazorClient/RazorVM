use crate::lc3::hardware::{Registers, Memory}; 

pub struct Instructions;

impl Instructions {
    pub fn add(instr: u16, registers: &mut Registers) {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag != 0 {
            let imm5 = sign_extend(instr & 0x1F, 5);
            registers.write(r0 as usize, registers.read(r1 as usize) + imm5);
        } else {
            let r2 = instr & 0x7;
            registers.write(r0 as usize, registers.read(r1 as usize) + registers.read(r2 as usize));
        }

        registers.update_flags(r0 as usize);
    }

   
}

fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

