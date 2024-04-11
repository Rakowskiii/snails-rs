use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum BlabladurInstruction {
    /// Initialises the contract by creating initial config and vault.
    ///
    /// These represent the parameters that will be included from client side
    ///
    /// `[w]` - writable, `[s]` - signer
    ///
    /// 0. `[s]` Payer account
    ///
    /// 1. `[s]` Contract config account (keypair)
    ///
    /// 2. `[w]` Contract vault account (PDA seed = "vault")
    ///
    /// 3. `[]` System program
    Initialise,
    /// Closes the vault, sending all SOL to admin.
    ///
    /// These represent the parameters that will be included from client side
    ///
    /// `[w]` - writable, `[s]` - signer
    ///
    /// 0. `[s]` Payer account
    ///
    /// 1. `[]` Contract config account
    ///
    /// 2. `[w]` Contract vault account
    CloseContract,
}
