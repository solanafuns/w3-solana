use {
    crate::{instruction::InstructionData, process::config, process::process, process::trunk},
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
            InstructionData::NameMapping {
                name,
                program,
                default_page,
            } => {
                msg!("Name Mapping: {:?}", name);
                config::name_config(program_id, accounts, &name, program, default_page)?
            }
            InstructionData::PutContent { path, body } => {
                process::put_content(program_id, accounts, &path, &body)?
            }
            InstructionData::PutTrunkContent {
                path,
                trunk_no,
                body,
            } => {
                msg!("Put Trunk Content: {:?} trunk_no: {}", path, trunk_no);
                trunk::put_trunk_content(program_id, accounts, &path, trunk_no, &body)?
            }
        },
        Err(err) => {
            msg!("Error: {:?}", err);
        }
    }
    Ok(())
}
