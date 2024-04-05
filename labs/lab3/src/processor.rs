use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
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
            crate::instruction::BlabladurInstruction::Initialise => {
                let accounts = &mut accounts.iter();
                let payer = next_account_info(accounts)?;
                let config = next_account_info(accounts)?;
                let vault = next_account_info(accounts)?;

                // -------- check accounts --------
                if !payer.is_signer {
                    msg!("Payer is not a signer");
                    return Err(
                        solana_program::program_error::ProgramError::MissingRequiredSignature,
                    );
                }

                if !config.is_signer {
                    msg!("Config is not a signer");
                    return Err(
                        solana_program::program_error::ProgramError::MissingRequiredSignature,
                    );
                }

                let (vault_address, bump) = Pubkey::find_program_address(&[b"vault"], program_id);

                if vault_address != *vault.key {
                    msg!("Vault address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                // ---- Set up the vault account ----

                let ix = system_instruction::create_account(
                    &payer.key,
                    &vault.key,
                    solana_program::rent::Rent::get()?.minimum_balance(0),
                    0,
                    program_id,
                );

                invoke_signed(
                    &ix,
                    &[payer.clone(), vault.clone()],
                    &[&[b"vault", &[bump]]],
                )?;

                // ---- transfer config ownership to program ----

                // Figure out account size and rent
                let len = std::mem::size_of::<crate::state::Config>();
                let rent = solana_program::rent::Rent::get()?.minimum_balance(len);

                // Make sure rent is paid
                let ix = system_instruction::transfer(&payer.key, &config.key, rent);
                invoke(&ix, &[payer.clone(), config.clone()])?;

                // Allocate space for the config
                let ix = system_instruction::allocate(config.key, len as u64);
                invoke(&ix, &[config.clone()])?;

                // Assign the config account to the program
                let ix = system_instruction::assign(config.key, program_id);
                invoke(&ix, &[config.clone()])?;

                // Create the config
                let config_data = crate::state::Config { admin: *payer.key };

                // Serialize the config into the account
                config_data.serialize(&mut &mut config.data.borrow_mut()[..])?;

                Ok(())
            }

            crate::instruction::BlabladurInstruction::CloseContract => {
                let accounts = &mut accounts.iter();
                let payer = next_account_info(accounts)?;
                let config = next_account_info(accounts)?;
                let vault = next_account_info(accounts)?;

                if !payer.is_signer {
                    msg!("Payer is not a signer");
                    return Err(
                        solana_program::program_error::ProgramError::MissingRequiredSignature,
                    );
                }

                // Load up the config
                let config = crate::state::Config::try_from_slice(&config.data.borrow())?;

                // Verify that the payer is the admin from the config
                if config.admin != *payer.key {
                    msg!("Payer is not the admin of the contract");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                let (vault_key, _) = Pubkey::find_program_address(&[b"vault"], program_id);

                if vault_key != *vault.key {
                    msg!("Vault address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                // ---- Close the vault account ----
                let amount = vault.lamports();
                **vault.lamports.borrow_mut() = 0;
                **payer.lamports.borrow_mut() += amount;

                msg!("Bye bye cruel world!");
                Ok(())
            }
        }
    }
}
