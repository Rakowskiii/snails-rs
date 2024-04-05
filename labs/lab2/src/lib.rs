use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{entrypoint, msg};

pub mod instruction;
mod processor;
pub mod state;

entrypoint!(process_instruction);

/// This is a example of storage vault
/// The admin can initialize the contract by creating vault and config
/// The admin address is stored in config
/// Anyone can deposit money into the vault
/// Only the admin can close the vault (and withdraw all money to admin account)
/// Or at least that was the idea...
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Welcome to the second lab!");
    if let Err(err) =
        crate::processor::Processor::process_instruction(program_id, accounts, instruction_data)
    {
        msg!("Error: {:?}", err);
        return Err(err);
    }
    Ok(())
}
