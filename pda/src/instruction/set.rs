use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError, 
    pubkey::Pubkey
};

use borsh::BorshSerialize;

use crate::{errors::FavoritesError, state::Favorites};

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