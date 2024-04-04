use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{entrypoint, msg};

mod instruction;
mod processor;
mod state;

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

#[cfg(test)]
mod tests {

    use super::*;
    use solana_program_test::{processor, tokio, ProgramTest};
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::Signer;

    #[tokio::test]
    async fn test_proper_flow() {
        let program_id = Pubkey::new_unique();

        let (banks_client, payer, recent_blockhash) =
            ProgramTest::new("overflow", program_id, processor!(process_instruction))
                .start()
                .await;

        let opponent = Keypair::new();

        let mut game =
            game::Game::new(payer, opponent, program_id, banks_client, recent_blockhash).await;

        game.deposit(500).await;
        assert_eq!(game.check_winner().await, game.opponent_pubkey());
        assert_eq!(game.check_player_vault_amount().await, 1000);
        assert_eq!(game.check_opponent_vault_amount().await, 500);
    }

    #[tokio::test]
    async fn hack() {
        let program_id = Pubkey::new_unique();
        let (banks_client, payer, recent_blockhash) =
            ProgramTest::new("overflow", program_id, processor!(process_instruction))
                .start()
                .await;

        let opponent = Keypair::new();

        let mut game =
            game::Game::new(payer, opponent, program_id, banks_client, recent_blockhash).await;

        // --   HACK   --

        // We can deposit any amount of money into the vault
        // Our opponent will deposit twice the amount into our vault
        // Can we find a way to win?
        let amount: u32 = 500;
        game.deposit(amount).await;

        let player_amount = game.check_player_vault_amount().await;
        let opponent_amount = game.check_opponent_vault_amount().await;
        dbg!(player_amount, opponent_amount);

        // -- END HACK --

        assert_eq!(game.check_winner().await, game.player_pubkey());
    }

    // Test utilities
    mod game {
        use super::*;
        use solana_program_test::BanksClient;
        use solana_sdk::hash::Hash;
        use solana_sdk::instruction::AccountMeta;
        use solana_sdk::native_token::LAMPORTS_PER_SOL;
        use solana_sdk::{instruction::Instruction, transaction::Transaction};
        use solana_sdk::{system_instruction, system_program};
        pub struct Game {
            player: Keypair,
            vault: Pubkey,
            opponent: Keypair,
            opponent_vault: Pubkey,
            client: BanksClient,
            program_id: Pubkey,
            recent_blockhash: Hash,
        }

        impl Game {
            pub async fn new(
                player: Keypair,
                opponent: Keypair,
                program_id: Pubkey,
                mut client: BanksClient,
                recent_blockhash: Hash,
            ) -> Self {
                // Fund opponent
                let ix = system_instruction::transfer(
                    &player.pubkey(),
                    &opponent.pubkey(),
                    LAMPORTS_PER_SOL,
                );

                let mut tx = Transaction::new_with_payer(&[ix], Some(&player.pubkey()));
                tx.sign(&[&player], recent_blockhash);
                client.process_transaction(tx).await.unwrap();

                let (vault, _) = Pubkey::find_program_address(
                    &[&player.pubkey().to_bytes()[..32], b"vault"],
                    &program_id,
                );
                let (opponent_vault, _) = Pubkey::find_program_address(
                    &[&opponent.pubkey().to_bytes()[..32], b"vault"],
                    &program_id,
                );

                // Initialise the vaults
                let accounts = vec![
                    AccountMeta::new(player.pubkey(), true),
                    AccountMeta::new(vault, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                ];

                let ix = Instruction::new_with_borsh(
                    program_id,
                    &instruction::BlabladurInstruction::InitialiseVault,
                    accounts,
                );
                let mut tx = Transaction::new_with_payer(&[ix], Some(&player.pubkey()));
                tx.sign(&[&player], recent_blockhash);
                client.process_transaction(tx).await.unwrap();

                let accounts = vec![
                    AccountMeta::new(opponent.pubkey(), true),
                    AccountMeta::new(opponent_vault, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                ];

                let ix = Instruction::new_with_borsh(
                    program_id,
                    &instruction::BlabladurInstruction::InitialiseVault,
                    accounts,
                );
                let mut tx = Transaction::new_with_payer(&[ix], Some(&opponent.pubkey()));
                tx.sign(&[&opponent], recent_blockhash);
                client.process_transaction(tx).await.unwrap();

                Self {
                    player,
                    vault,
                    opponent,
                    opponent_vault,
                    client,
                    program_id,
                    recent_blockhash,
                }
            }

            pub async fn check_winner(&mut self) -> Pubkey {
                let vault: state::Vault = self
                    .client
                    .get_account_data_with_borsh(self.vault)
                    .await
                    .unwrap();
                let opponent_vault: state::Vault = self
                    .client
                    .get_account_data_with_borsh(self.opponent_vault)
                    .await
                    .unwrap();

                if vault.amount < opponent_vault.amount {
                    self.player.pubkey()
                } else {
                    self.opponent.pubkey()
                }
            }

            pub async fn deposit(&mut self, amount: u32) {
                let accounts = vec![AccountMeta::new(self.opponent_vault, false)];

                let ix = Instruction::new_with_borsh(
                    self.program_id,
                    &instruction::BlabladurInstruction::Deposit { amount },
                    accounts,
                );

                let mut tx = Transaction::new_with_payer(&[ix], Some(&self.player.pubkey()));

                tx.sign(&[&self.player], self.recent_blockhash);
                self.client.process_transaction(tx).await.unwrap();

                // Every time we deposit to enemy, they deposit to us twice the amount
                let accounts = vec![AccountMeta::new(self.vault, false)];

                let ix = Instruction::new_with_borsh(
                    self.program_id,
                    &instruction::BlabladurInstruction::Deposit { amount: amount * 2 },
                    accounts,
                );

                let mut tx = Transaction::new_with_payer(&[ix], Some(&self.opponent.pubkey()));

                tx.sign(&[&self.opponent], self.recent_blockhash);
                self.client.process_transaction(tx).await.unwrap();
            }

            async fn check_vault_amount(&mut self, vault: Pubkey) -> u32 {
                let vault: state::Vault = self
                    .client
                    .get_account_data_with_borsh(vault)
                    .await
                    .unwrap();
                vault.amount
            }

            pub async fn check_opponent_vault_amount(&mut self) -> u32 {
                self.check_vault_amount(self.opponent_vault).await
            }

            pub async fn check_player_vault_amount(&mut self) -> u32 {
                self.check_vault_amount(self.vault).await
            }

            pub fn player_pubkey(&self) -> Pubkey {
                self.player.pubkey()
            }

            pub fn opponent_pubkey(&self) -> Pubkey {
                self.opponent.pubkey()
            }
        }
    }
}
