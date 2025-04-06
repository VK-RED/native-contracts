use solana_program::entrypoint;

pub mod errors;
pub mod state;
pub mod instruction;

use instruction::process_instruction;

entrypoint!(process_instruction);