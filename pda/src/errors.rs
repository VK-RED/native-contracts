use solana_program::program_error::ProgramError;
use thiserror::Error;

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