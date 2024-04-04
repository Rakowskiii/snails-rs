use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    sysvar::Sysvar,
};
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let ix = crate::instruction::BlabladurInstruction::try_from_slice(instruction_data)?;
        match ix {
            crate::instruction::BlabladurInstruction::InitialiseVault => {
                let accounts = &mut accounts.iter();
                let player = next_account_info(accounts)?;
                let vault = next_account_info(accounts)?;

                let (vault_address, bump) = Pubkey::find_program_address(
                    &[&player.key.to_bytes()[..32], b"vault"],
                    program_id,
                );

                if vault_address != *vault.key {
                    msg!("Vault address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                let len = std::mem::size_of::<crate::state::Vault>();
                let rent = solana_program::rent::Rent::get()?.minimum_balance(len);

                msg!("Initialising vault");
                invoke_signed(
                    &system_instruction::create_account(
                        &player.key,
                        &vault.key,
                        rent,
                        len as u64,
                        program_id,
                    ),
                    &[player.clone(), vault.clone()],
                    &[&[&player.key.to_bytes()[..32], b"vault", &[bump]]],
                )?;
                msg!("Vault initialised");

                Ok(())
            }
            crate::instruction::BlabladurInstruction::Deposit { amount } => {
                let accounts = &mut accounts.iter();
                let vault = next_account_info(accounts)?;

                if vault.owner != program_id {
                    msg!("Vault is not owned by the program");
                    return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
                }

                let mut vault_data = crate::state::Vault::try_from_slice(&vault.data.borrow())?;
                vault_data.amount += amount;
                vault_data.serialize(&mut &mut vault.data.borrow_mut()[..])?;

                msg!("Deposited {} into vault", amount);
                Ok(())
            }
        }
    }
}
