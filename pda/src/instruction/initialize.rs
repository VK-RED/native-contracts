use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke_signed,
    program_error::ProgramError, 
    pubkey::Pubkey, 
    rent::Rent,
    system_instruction, 
    sysvar::Sysvar
};

use crate::state::Favorites;

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