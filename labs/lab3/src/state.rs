use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Vault {
    pub authority: Pubkey,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Config {
    pub admin: Pubkey,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct State {
    pub frozen: bool,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum FreezeState {
    Frozen,
    Unfrozen,
}
