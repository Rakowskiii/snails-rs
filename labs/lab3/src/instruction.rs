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
    /// 1. `[w]` PDA of config (seed = "config")
    ///
    /// 2. `[w]` PDA of state (seed = "state")
    ///
    /// 3. `[]` System program
    InitialiseProgram,

    /// Initializes a vault for a user, creating a new account owned by the program.
    ///
    /// These represent the parameters that will be included from client side
    ///
    /// 0. `[s]` User account who will own the vault. Must be a signer.
    ///
    /// 1. `[w]` PDA of the user's vault (seed = user's public key + "vault"), to be initialized by this instruction.
    ///
    /// 2. `[]` System program, used to create the vault account.
    InitialiseVault,

    /// Allows the user to withdraw all lamports from their vault.
    ///
    /// Accounts expected:
    ///
    /// 0. `[s]` User account attempting the withdrawal. Must be a signer and the authority of the vault.
    ///
    /// 1. `[w]` Vault account from which lamports are withdrawn. Must be owned by the program.
    ///
    /// 2. `[]` State account to check if the withdrawal is currently allowed (not frozen).
    ///
    /// 3. `[]` System program, used for the lamport transfer.
    Withdraw,

    /// Sets the freeze state of the program, controlling whether withdrawals are allowed.
    ///
    /// Accounts expected:
    ///
    /// 0. `[s]` Admin account responsible for changing the state. Must be a signer.
    /// 1. `[]` Config account containing administrative settings. Must be owned by the program.
    /// 2. `[w]` State account whose freeze state is being changed. Must be owned by the program.
    /// 3. `[]` System program - not directly used but listed for completeness.
    ///
    /// Parameters:
    /// - `desired_state`: The desired state (Frozen/Unfrozen) to set.
    SetState {
        desired_state: crate::state::FreezeState,
    },
}
