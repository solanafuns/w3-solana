use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use w3solana::pda_helper::PdaHelper;

use crate::instruction::PageData;

pub fn put_trunk_content(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    path: &String,
    trunk_no: u8,
    body: &Vec<u8>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let meta_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let trunk_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let system_program_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;

    let pda_helper = PdaHelper::new(program_id.clone());

    {
        msg!("do trunk meta_account update !!!");
        let (meta_pda, bump_seed) = pda_helper.find_program_address_by_text(&path);
        msg!("meta_pda: {:?}", meta_pda);
        msg!("meta_account.key: {:?}", meta_account.key);
        assert!(meta_account.key == &meta_pda);

        let meta_page = &PageData::TrunkPage { trunks: trunk_no };
        let meta_data = meta_page.try_to_vec()?;

        if meta_account.data_is_empty() {
            let rent: Rent = Rent::get()?;
            let data_size = meta_data.len();
            let rent_lamports = rent.minimum_balance(data_size);
            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    &meta_pda,
                    rent_lamports,
                    data_size as u64,
                    program_id,
                ),
                &[
                    payer.clone(),
                    meta_account.clone(),
                    system_program_account.clone(),
                ],
                &[&[path.as_bytes(), &[bump_seed]]],
            )?;
        }
        let mut pda_data = meta_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(&meta_data);
    }

    let (content_pda, bump_seed) =
        pda_helper.find_program_address_by_text_suffix(&path, &[trunk_no]);

    assert!(trunk_account.key == &content_pda);

    if trunk_account.data_is_empty() {
        let rent: Rent = Rent::get()?;
        let data_size = body.len();
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
                trunk_account.clone(),
                system_program_account.clone(),
            ],
            &[&[path.as_bytes(), &[trunk_no], &[bump_seed]]],
        )?;

        let mut pda_data = trunk_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(&body);
    } else {
        let new_size = body.len();
        let need_rents = Rent::get()?.minimum_balance(new_size);
        if trunk_account.lamports() < need_rents {
            let amount: u64 = need_rents - trunk_account.lamports();
            let transfer_ix = system_instruction::transfer(payer.key, trunk_account.key, amount);
            invoke(&transfer_ix, &[payer.clone(), trunk_account.clone()])?;
        }
        trunk_account.realloc(new_size, true)?;

        let mut pda_data = trunk_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(&body);
    }

    Ok(())
}
