use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{entrypoint, msg};

pub mod instruction;
mod processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello to Blabladur security lab!");
    if let Err(err) =
        crate::processor::Processor::process_instruction(program_id, accounts, instruction_data)
    {
        msg!("Error: {:?}", err);
        return Err(err);
    }
    Ok(())
}
