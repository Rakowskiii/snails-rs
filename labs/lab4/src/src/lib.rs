pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use solana_program_test::{processor, tokio, ProgramTest};
    use solana_sdk::signer::Signer;
    use solana_sdk::{instruction::Instruction, transaction::Transaction};

    use super::*;

    #[tokio::test]
    async fn hello_world() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) =
            ProgramTest::new("hello_world", program_id, processor!(process_instruction))
                .start()
                .await;

        let accounts = vec![];

        let ix = Instruction::new_with_borsh(program_id, &(), accounts);

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

        tx.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(tx).await.unwrap();
    }
}
