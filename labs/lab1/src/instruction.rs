use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum BlabladurInstruction {
    /// Deposits the amount to the chosen vault.
    /// Anyone can deposit to the vault.
    ///
    /// These represent the parameters that will be included from client side
    ///
    /// `[w]` - writable, `[s]` - signer
    ///
    /// 0. `[w]` Chosen Vault to which the player wants to add
    ///
    Deposit { amount: u32 },

    /// Initialises the vault for the player
    ///
    /// These represent the parameters that will be included from client side
    ///
    /// `[w]` - writable, `[s]` - signer
    ///
    /// 0. `[s]` Player wanting to join the game
    /// 1. `[w]` Player's vault to initialise for the game seed = [player_pubkey]
    InitialiseVault,
}
