use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError, 
    pubkey::Pubkey
};

use crate::{
    instruction::{
        set::process_setfavorites_instruction, 
        initialize::process_initialize_instruction
    },
    state::Favorites
};

use borsh::BorshDeserialize;

pub enum Instruction {
    SetFavorites {
        data: Vec<Option<String>>
    },
    Initialize
}


impl Instruction{

    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        
        let (variant, rest) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    

        match *variant {
            0 => {
                msg!("before deserializing");
                let favorites = Favorites::try_from_slice(rest)?;
                msg!("after deserializing");
                // Validate the length of the array
                // Validate the length of the strings in array
                favorites.validate()?;

                Ok(Self::SetFavorites { data: favorites.data })
            },

            1 => {
                Ok(Self::Initialize)
            },
            _ => {
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}

pub fn process_instruction(program_id:&Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult{
    
    match Instruction::unpack(instruction_data) ? {

        Instruction::SetFavorites { data } => {

            msg!("SetFavorites is being called");

            let favorites = Favorites {data};

            process_setfavorites_instruction(program_id, accounts, favorites)?
        }

        Instruction::Initialize => {
            process_initialize_instruction(program_id, accounts)?            
        }
    }


    Ok(())
    
}