use {
    log::{error, info},
    solana_program::pubkey::Pubkey,
    solana_sdk::{instruction::Instruction, signature::Keypair, transaction::Transaction},
};

use crate::client::W3Client;
pub trait SolanaTransaction {
    fn send_instruction(&self, payer: &Pubkey, singers: &Vec<&Keypair>, instruction: Instruction);
    fn find_program_address(&self, seeds: &[&[u8]]) -> (Pubkey, u8);
    fn find_program_address_by_text(&self, seed_text: &str) -> (Pubkey, u8);
}

impl SolanaTransaction for W3Client {
    fn find_program_address_by_text(&self, seed_text: &str) -> (Pubkey, u8) {
        let mut seeds: Vec<&[u8]> = Vec::new();
        if seed_text.len() > 32 {
            for chunk in seed_text.as_bytes().chunks(32) {
                seeds.push(chunk);
            }
        } else {
            seeds.push(seed_text.as_bytes());
        }
        Pubkey::find_program_address(seeds.as_ref(), &self.program)
    }

    fn find_program_address(&self, seeds: &[&[u8]]) -> (Pubkey, u8) {
        Pubkey::find_program_address(seeds, &self.program)
    }

    fn send_instruction(&self, payer: &Pubkey, singers: &Vec<&Keypair>, instruction: Instruction) {
        info!("instruction data len : {:?}", instruction.data.len());

        let blockhash = self.connection.get_latest_blockhash().unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[instruction], Some(&payer), singers, blockhash);

        match self.connection.send_and_confirm_transaction(&transaction) {
            Ok(tx) => {
                info!("send transaction tx : {:?}", tx);
            }
            Err(e) => {
                error!("send transaction error : {:?}", e);
            }
        }
    }
}
