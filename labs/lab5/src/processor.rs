use solana_sdk::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Hello to Baldur!");
        Ok(())
    }
}
