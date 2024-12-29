#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    Br = 0,
    Add = 1,
    Ld = 2,
    St = 3,
    Jsr = 4,
    And = 5,
    Ldr = 6,
    Str = 7,
    Rti = 8,
    Not = 9,
    Ldi = 10,
    Sti = 11,
    Jmp = 12,
    Res = 13,
    Lea = 14,
    Trap = 15,
}

#[derive(Debug, PartialEq)]
pub enum OpCodeError {
    InvalidOpcode,
}

impl OpCode {
    #[inline]
    pub const fn get(value: u16) -> Result<OpCode, OpCodeError> {
        match value {
            0 => Ok(OpCode::Br),
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Ld),
            3 => Ok(OpCode::St),
            4 => Ok(OpCode::Jsr),
            5 => Ok(OpCode::And),
            6 => Ok(OpCode::Ldr),
            7 => Ok(OpCode::Str),
            8 => Ok(OpCode::Rti),
            9 => Ok(OpCode::Not),
            10 => Ok(OpCode::Ldi),
            11 => Ok(OpCode::Sti),
            12 => Ok(OpCode::Jmp),
            13 => Ok(OpCode::Res),
            14 => Ok(OpCode::Lea),
            15 => Ok(OpCode::Trap),
            _ => Err(OpCodeError::InvalidOpcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_codes_initial_values() {
        assert_eq!(Ok(OpCode::Br), OpCode::get(0));
        assert_eq!(Ok(OpCode::Add), OpCode::get(1));
        assert_eq!(Ok(OpCode::Ld), OpCode::get(2));
        assert_eq!(Ok(OpCode::St), OpCode::get(3));
        assert_eq!(Ok(OpCode::Jsr), OpCode::get(4));
        assert_eq!(Ok(OpCode::And), OpCode::get(5));
        assert_eq!(Ok(OpCode::Ldr), OpCode::get(6));
        assert_eq!(Ok(OpCode::Str), OpCode::get(7));
        assert_eq!(Ok(OpCode::Rti), OpCode::get(8));
        assert_eq!(Ok(OpCode::Not), OpCode::get(9));
        assert_eq!(Ok(OpCode::Ldi), OpCode::get(10));
        assert_eq!(Ok(OpCode::Sti), OpCode::get(11));
        assert_eq!(Ok(OpCode::Jmp), OpCode::get(12));
        assert_eq!(Ok(OpCode::Res), OpCode::get(13));
        assert_eq!(Ok(OpCode::Lea), OpCode::get(14));
        assert_eq!(Ok(OpCode::Trap), OpCode::get(15));
    }

    #[test]
    fn invalid_op_codes() {
        assert_eq!(Err(OpCodeError::InvalidOpcode), OpCode::get(16)); // Out of range
        assert_eq!(Err(OpCodeError::InvalidOpcode), OpCode::get(255)); // Completely invalid
    }
}
