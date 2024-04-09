use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use w3solana::pda_helper::PdaHelper;

use crate::instruction::PageData;

pub fn put_content(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    path: &String,
    body: &Vec<u8>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let content_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let system_program_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;

    let (content_pda, bump_seed) =
        PdaHelper::new(program_id.clone()).find_program_address_by_text(&path);

    assert!(content_account.key == &content_pda);
    let raw_page_data = &PageData::RawData { data: body.clone() }.try_to_vec()?;
    let data_size = raw_page_data.len();
    if content_account.data_is_empty() {
        let rent: Rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(data_size);
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                &content_pda,
                rent_lamports,
                data_size as u64,
                program_id,
            ),
            &[
                payer.clone(),
                content_account.clone(),
                system_program_account.clone(),
            ],
            &[&[path.as_bytes(), &[bump_seed]]],
        )?;
        let mut pda_data = content_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(raw_page_data);
    } else {
        let need_rents = Rent::get()?.minimum_balance(data_size);
        if content_account.lamports() < need_rents {
            let amount: u64 = need_rents - content_account.lamports();
            let transfer_ix = system_instruction::transfer(payer.key, content_account.key, amount);
            invoke(&transfer_ix, &[payer.clone(), content_account.clone()])?;
        }
        content_account.realloc(data_size, true)?;
        let mut pda_data = content_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(&raw_page_data);
    }

    Ok(())
}
