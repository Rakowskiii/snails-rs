use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Config {
    pub admin: Pubkey,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Board {
    pub sign: [[u8; 5]; 5],
    pub turn: u8,        // 26
    pub stake: u32,      //30
    pub winner: u8,      // 31
    pub game_status: u8, // 32
    pub players: [Pubkey; 2],
}
