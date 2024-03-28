use borsh::{self, BorshDeserialize, BorshSerialize};
use solana_program::pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Mode {
    Auto = 0,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    PutContent {
        path: String,
        body: Vec<u8>,
    },
    NameMapping {
        name: String,
        program: pubkey::Pubkey,
    },
}

impl InstructionData {
    pub fn to_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(self).unwrap()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct NameConfig {
    pub name: String,
    pub program: pubkey::Pubkey,
    pub creator: pubkey::Pubkey,
    pub created_at: u64,
}

impl NameConfig {
    pub fn to_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(self).unwrap()
    }
}
