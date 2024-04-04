use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum BlabladurInstruction {
    /// Welcomes a user to Blabladur Solana Security Labs
    ///
    /// Takes 0 accounts
    ///
    /// Data: String - name
    WelcomeInstruction(String),

    /// Fails the program
    ///
    /// Takes 0 accounts
    ///
    /// This is used to show how to handle different instructions
    NoOp,
}
