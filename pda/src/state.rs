use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError};
use crate::errors::FavoritesError;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Favorites {
    pub data: Vec<Option<String>>,
    
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