use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{entrypoint, msg};

mod instruction;
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

#[cfg(test)]
mod tests {
    use solana_program_test::{processor, tokio, ProgramTest};
    use solana_sdk::signer::Signer;
    use solana_sdk::{instruction::Instruction, transaction::Transaction};

    use super::*;

    #[tokio::test]
    async fn hello_world() {
        let program_id = Pubkey::new_unique();

        let name = String::from("Blablador");

        let (mut banks_client, payer, recent_blockhash) =
            ProgramTest::new("hello_world", program_id, processor!(process_instruction))
                .start()
                .await;

        let accounts = vec![];

        let ix = Instruction::new_with_borsh(
            program_id,
            &instruction::BlabladurInstruction::WelcomeInstruction(name),
            accounts,
        );

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

        tx.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(tx).await.unwrap();
    }
}
