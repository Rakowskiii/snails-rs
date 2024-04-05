use hello_world::{instruction::BlabladurInstruction, process_instruction};
use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::{instruction::Instruction, transaction::Transaction};

#[tokio::test]
async fn test_proper_flow() {
    let program_id = Pubkey::new_unique();

    let name = String::from("Blablador");

    let (mut banks_client, payer, recent_blockhash) =
        ProgramTest::new("hello_world", program_id, processor!(process_instruction))
            .start()
            .await;

    let accounts = vec![];

    let ix = Instruction::new_with_borsh(
        program_id,
        &BlabladurInstruction::WelcomeInstruction(name),
        accounts,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

    tx.sign(&[&payer], recent_blockhash);
    assert!(banks_client.process_transaction(tx).await.is_ok());
}

#[tokio::test]
async fn hack() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) =
        ProgramTest::new("hello_world", program_id, processor!(process_instruction))
            .start()
            .await;

    let name = String::from("hacker");

    // --   HACK   --
    // Change the instruction to force program to fail
    let blabla_ix = BlabladurInstruction::WelcomeInstruction(name);
    // -- END HACK --

    let accounts = vec![];

    let ix = Instruction::new_with_borsh(program_id, &blabla_ix, accounts);

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

    tx.sign(&[&payer], recent_blockhash);
    assert!(banks_client.process_transaction(tx).await.is_err());
}
