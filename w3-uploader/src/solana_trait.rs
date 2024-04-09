use {
    log::{error, info},
    solana_program::pubkey::Pubkey,
    solana_sdk::{instruction::Instruction, signature::Keypair, transaction::Transaction},
};

use crate::client::W3Client;
pub trait SolanaTransaction {
    fn send_instruction(&self, payer: &Pubkey, singers: &Vec<&Keypair>, instruction: Instruction);
}

impl SolanaTransaction for W3Client {
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
