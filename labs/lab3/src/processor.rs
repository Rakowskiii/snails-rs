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

use crate::state;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let ix = crate::instruction::BlabladurInstruction::try_from_slice(instruction_data)?;
        match ix {
            crate::instruction::BlabladurInstruction::InitialiseProgram => {
                let accounts = &mut accounts.iter();
                let admin = next_account_info(accounts)?;
                let config_account = next_account_info(accounts)?;
                let state_account = next_account_info(accounts)?;

                if !admin.is_signer {
                    msg!("Admin is not a signer");
                    return Err(
                        solana_program::program_error::ProgramError::MissingRequiredSignature,
                    );
                }

                let config = state::Config { admin: *admin.key };
                let state = state::State { frozen: false };

                // This is computation expensive and shouldn't be done in the program
                // Should be provied by user and only verified in the program
                let (config_calculated, config_bump) =
                    Pubkey::find_program_address(&[b"config"], program_id);
                let (state_calculated, state_bump) =
                    Pubkey::find_program_address(&[b"state"], program_id);

                if *config_account.key != config_calculated {
                    msg!("Config address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                if *state_account.key != state_calculated {
                    msg!("State address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                let config_space = std::mem::size_of::<state::Config>();
                let state_space = std::mem::size_of::<state::State>();

                let rent = solana_program::rent::Rent::get()?;

                invoke_signed(
                    &system_instruction::create_account(
                        admin.key,
                        config_account.key,
                        rent.minimum_balance(config_space),
                        config_space as u64,
                        program_id,
                    ),
                    &[admin.clone(), config_account.clone()],
                    &[&[b"config", &[config_bump]]],
                )?;

                invoke_signed(
                    &system_instruction::create_account(
                        admin.key,
                        state_account.key,
                        rent.minimum_balance(state_space),
                        state_space as u64,
                        program_id,
                    ),
                    &[admin.clone(), state_account.clone()],
                    &[&[b"state", &[state_bump]]],
                )?;

                config.serialize(&mut &mut config_account.data.borrow_mut()[..])?;
                state.serialize(&mut &mut state_account.data.borrow_mut()[..])?;

                Ok(())
            }

            crate::instruction::BlabladurInstruction::InitialiseVault => {
                let accounts = &mut accounts.iter();
                let user = next_account_info(accounts)?;
                let vault = next_account_info(accounts)?;

                let (vault_calculated, vault_bump) = Pubkey::find_program_address(
                    &[&user.key.to_bytes()[..32], b"vault"],
                    program_id,
                );

                if *vault.key != vault_calculated {
                    msg!("User address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                let vault_space = std::mem::size_of::<state::Vault>();

                invoke_signed(
                    &system_instruction::create_account(
                        user.key,
                        vault.key,
                        solana_program::rent::Rent::get()?.minimum_balance(vault_space),
                        vault_space as u64,
                        program_id,
                    ),
                    &[user.clone(), vault.clone()],
                    &[&[&user.key.to_bytes()[..32], b"vault", &[vault_bump]]],
                )?;

                let user_data = state::Vault {
                    authority: *user.key,
                };

                user_data.serialize(&mut &mut vault.data.borrow_mut()[..])?;

                Ok(())
            }

            crate::instruction::BlabladurInstruction::Withdraw => {
                msg!("Withdrawing...");
                let accounts = &mut accounts.iter();
                let user = next_account_info(accounts)?;
                let vault = next_account_info(accounts)?;
                let state = next_account_info(accounts)?;

                let vault_data = crate::state::Vault::try_from_slice(&vault.data.borrow())?;
                let state_data = crate::state::State::try_from_slice(&state.data.borrow())?;

                if vault.owner != program_id {
                    msg!("Vault is not owned by the program");
                    return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
                }

                if state.owner != program_id {
                    msg!("State is not owned by the program");
                    return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
                }

                if !user.is_signer {
                    msg!("User is not a signer");
                    return Err(
                        solana_program::program_error::ProgramError::MissingRequiredSignature,
                    );
                }

                if *user.key != vault_data.authority {
                    msg!("User is not the vault authority");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                if state_data.frozen {
                    msg!("State is frozen");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                //TODO: disclaimer
                let (vault_calculated, _) = Pubkey::find_program_address(
                    &[&user.key.to_bytes()[..32], b"vault"],
                    program_id,
                );

                if vault_calculated != *vault.key {
                    msg!("Vault address is incorrect");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                let amount = vault.lamports();
                **vault.lamports.borrow_mut() -= amount;
                **user.lamports.borrow_mut() += amount;

                msg!("Bye bye cruel world!");
                Ok(())
            }

            crate::instruction::BlabladurInstruction::SetState { desired_state } => {
                let accounts = &mut accounts.iter();
                let admin = next_account_info(accounts)?;
                let config = next_account_info(accounts)?;
                let state = next_account_info(accounts)?;

                if config.owner != program_id {
                    msg!("Config is not owned by the program");
                    return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
                }

                if state.owner != program_id {
                    msg!("State is not owned by the program");
                    return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
                }

                if !admin.is_signer {
                    msg!("Admin is not a signer");
                    return Err(
                        solana_program::program_error::ProgramError::MissingRequiredSignature,
                    );
                }

                let config_data = state::Config::try_from_slice(&config.data.borrow())?;

                if config_data.admin != *admin.key {
                    msg!("Admin is not the admin of the contract");
                    return Err(solana_program::program_error::ProgramError::InvalidArgument);
                }

                let desired_state = match desired_state {
                    crate::state::FreezeState::Frozen => state::State { frozen: true },
                    crate::state::FreezeState::Unfrozen => state::State { frozen: false },
                };

                desired_state.serialize(&mut &mut state.data.borrow_mut()[..])?;

                msg!("State has changed to {:?}", desired_state.frozen);
                Ok(())
            }
        }
    }
}
