use solana_program::{account_info::{next_account_info, AccountInfo, }, entrypoint::{entrypoint, ProgramResult}, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar};
use borsh::{BorshDeserialize, BorshSerialize};
use thiserror::Error;

entrypoint!(process_instruction);

#[derive(Error, Debug)]
pub enum FavoritesError {
    #[error("The Data Array Length cannot be 0")]
    InvalidLength,

    #[error("The Data Array Length cannot be greater than 5")]
    ExceedsMaxLength,

    #[error("Individual Item cannot have length greater than 10")]
    ExceedsIndMaxLength,

    #[error("Invalid PDA")]
    IncorrectFavoritesId,

    #[error("Signer Not found")]
    SignerNotFound
}

// Implement From trait for ProgramError to return our FavoritesError as a ProgramError
impl From<FavoritesError> for ProgramError {

    fn from(value: FavoritesError) -> Self {
        ProgramError::Custom(value as u32)    
    }
}

pub enum Instruction {
    SetFavorites {
        data: Vec<Option<String>>
    },
    Initialize
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Favorites {
    data: Vec<Option<String>>,
    
}

impl Favorites {

    pub fn validate(&self) -> ProgramResult{
        if self.data.len() == 0 {
            Err(FavoritesError::InvalidLength.into())
        }
        else if self.data.len() > 5 {
            Err(FavoritesError::ExceedsMaxLength.into())
        }
        else{

            let iter = self.data.iter();

            for val in iter {
                if let Some(data) = val {
                    if data.len() > 10 {
                        return Err(FavoritesError::ExceedsIndMaxLength.into());
                    }
                }
            }

            msg!("Validated successfully");
            Ok(())
        }

    }

    pub fn get_size() -> Result<usize, ProgramError> {

        // We limit each word with only 10 letters and the array of length 5

        let word = "tenletters".to_string();

        let mut data = Vec::new();

        for _  in 0..5 {
            data.push(Some(word.clone()));
        }

        let favorites = Favorites{data};

        let size = borsh::to_vec(&favorites)?.len();
        
        Ok(size)
    }

    pub fn new()-> Self {
        let mut v : Vec<Option<String>> = Vec::new();

        for _ in 0..5 {
            v.push(None);
        }

        let favorites = Favorites{data:v};

        favorites
    }


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

pub fn process_setfavorites_instruction(program_id:&Pubkey, accounts: &[AccountInfo], favorites:Favorites) -> ProgramResult {

    let iter = &mut accounts.iter();

    let user_account = next_account_info(iter)?;
    let favorites_account = next_account_info(iter)?;

    msg!("reached set favorites instruction");

    // check if the favorites account is owned by the Program
    if favorites_account.owner != program_id {

        msg!("favorites account owner : {}", favorites_account.owner);
        msg!("programid : {}", program_id);
        return Err(ProgramError::IllegalOwner);
    }

    msg!("The owner is set correct");

    // check if the user signed the transaction
    if user_account.is_signer == false {
        return Err(FavoritesError::SignerNotFound.into())
    }

    msg!("The user is set as signer");

    // check if the favorites_account is writable
    if favorites_account.is_writable == false {
        return Err(ProgramError::Immutable)
    }

    msg!("favorites account is set as writable");

    let seed = "favorites".to_string();
    let seeds = [seed.as_bytes(), &user_account.key.to_bytes()];


    let (favorites_derived_key, _bump) = Pubkey::find_program_address(&seeds, program_id);

    if favorites_derived_key != *favorites_account.key {
        return Err(FavoritesError::IncorrectFavoritesId.into())
    }

    msg!("Passed correct favorites account");

    let mut onchain_data = &mut *favorites_account.data.borrow_mut();

    favorites.serialize(&mut onchain_data)?;

    msg!("SetFavorites Executed Successfully");

    Ok(())
}

pub fn process_initialize_instruction(program_id:&Pubkey, accounts: &[AccountInfo]) -> ProgramResult{

    let iter = &mut accounts.iter();
    let user_account = next_account_info(iter)?;
    let favorites_account = next_account_info(iter)?;
    let system_program_account = next_account_info(iter)?;

    let seeds = [b"favorites", user_account.key.as_ref()];

    let (favorites_address, bump)  = Pubkey::find_program_address(&seeds, program_id);

    msg!("favorites address is : {}", favorites_address);

    // Throw error if the PDA mismatch in client and onchain

    if favorites_address != *favorites_account.key {
        Err(ProgramError::IncorrectProgramId)?
    }

    let favorites_data_size = favorites_account.data.borrow().len();

    if favorites_data_size > 0 {
        Err(ProgramError::AccountAlreadyInitialized)?
    }


    let favorites_size = Favorites::get_size()?;

    let lamports = Rent::get()?.minimum_balance(favorites_size);

    msg!("before calling cpi");

    let ix = system_instruction::create_account(
        user_account.key, 
        favorites_account.key, 
        lamports, 
        favorites_size as u64, 
        program_id
    );

    invoke_signed(
        &ix, 
        &[user_account.clone(), favorites_account.clone(), system_program_account.clone()], 
        &[&[b"favorites", user_account.key.as_ref(), &[bump]]]
    )?;

    msg!("cpi completed successfully!");

    Ok(())


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