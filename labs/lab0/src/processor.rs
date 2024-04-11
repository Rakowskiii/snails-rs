use borsh::BorshDeserialize;
use solana_sdk::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let ix = crate::instruction::BlabladurInstruction::try_from_slice(instruction_data)?;
        match ix {
            crate::instruction::BlabladurInstruction::WelcomeInstruction(name) => {
                msg!("Hello {}! Welcome to Blabladur Solana Security Labs!", name);

                Ok(())
            }
            crate::instruction::BlabladurInstruction::NoOp => {
                msg!("NoOp!");
                Err(solana_program::program_error::ProgramError::InvalidInstructionData)
            }
        }
    }
}
