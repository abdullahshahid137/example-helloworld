use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

#[derive(Debug)]
pub enum Instruction {
    add,
    sub,
    set(u32),
}

impl Instruction {
    pub fn unpack(buffer: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = buffer.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => return Ok(Instruction::add),
            1 => return Ok(Instruction::sub),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8;4], _> = rest[..4].try_into();
                match val {
                    Ok(i) => return Ok(Instruction::set(u32::from_le_bytes(i))),
                    _ => return Err(ProgramError::InvalidInstructionData),
                }
            },
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}


