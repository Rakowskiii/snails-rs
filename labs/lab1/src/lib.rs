use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{entrypoint, msg};

pub mod instruction;
mod processor;
pub mod state;

entrypoint!(process_instruction);

/// This is a funny game where each player has their vault
/// The goal of the game is to have the lowest amount of money in your vault
/// The game doesn't end, and has no purpose other than to have fun
/// If the vault is full, the player is out of the game
/// The player can deposit money into any vault
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Welcome to the first lab!");
    if let Err(err) =
        crate::processor::Processor::process_instruction(program_id, accounts, instruction_data)
    {
        msg!("Error: {:?}", err);
        return Err(err);
    }
    Ok(())
}
