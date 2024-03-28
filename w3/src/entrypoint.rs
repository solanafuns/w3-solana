use {
    crate::{instruction::InstructionData, process},
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, msg,
        pubkey::Pubkey,
    },
};

declare_id!("9pW59BsNCqtQC1xucwTXYS4Qe9qz5AgSy2jajE63odQb");

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Program ID: {:?}", program_id);
    match InstructionData::try_from_slice(instruction_data) {
        Ok(instruction_data) => match instruction_data {
            InstructionData::PutContent { path, body } => {
                process::put_content(program_id, accounts, &path, &body)?
            }
            InstructionData::NameMapping { name, program } => {
                msg!("Name Mapping: {:?}", name);
                process::name_config(program_id, accounts, &name, program)?
            }
        },
        Err(err) => {
            msg!("Error: {:?}", err);
        }
    }
    Ok(())
}
