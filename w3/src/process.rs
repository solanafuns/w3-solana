use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::{self, Pubkey},
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

use crate::instruction::NameConfig;

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
    let (content_pda, bump_seed) = Pubkey::find_program_address(&[path.as_bytes()], program_id);
    assert!(content_account.key == &content_pda);
    if content_account.data_is_empty() {
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
                content_account.clone(),
                system_program_account.clone(),
            ],
            &[&[path.as_bytes(), &[bump_seed]]],
        )?;

        let mut pda_data = content_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(&body);
    } else {
        let new_size = body.len();
        let need_rents = Rent::get()?.minimum_balance(new_size);
        if content_account.lamports() < need_rents {
            let amount: u64 = need_rents - content_account.lamports();
            let dest_starting_lamports = payer.lamports();

            **payer.lamports.borrow_mut() = dest_starting_lamports.checked_add(amount).unwrap();
            **content_account.lamports.borrow_mut() -= amount;
        }
        content_account.realloc(new_size, true)?;

        let mut pda_data = content_account.try_borrow_mut_data()?;
        pda_data.copy_from_slice(&body);
    }

    Ok(())
}

pub fn name_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: &String,
    program: pubkey::Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let config_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let system_program_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    // let my_seed_helper = SeedHelper::new(".w3-solana-name".to_string());
    let base_seed = ".w3-solana-name";
    let (config_pda, bump_seed) =
        Pubkey::find_program_address(&[base_seed.as_bytes(), name.as_bytes()], program_id);
    assert!(config_account.data_is_empty());

    let now = Clock::get()?;

    let rent: Rent = Rent::get()?;
    let config_data = NameConfig {
        name: name.clone(),
        program,
        creator: payer.key.clone(),
        created_at: now.unix_timestamp as u64,
    };
    let data_size = config_data.to_bytes().len();
    let rent_lamports = rent.minimum_balance(data_size);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            &config_pda,
            rent_lamports,
            data_size as u64,
            program_id,
        ),
        &[
            payer.clone(),
            config_account.clone(),
            system_program_account.clone(),
        ],
        &[&[base_seed.as_bytes(), name.as_bytes(), &[bump_seed]]],
    )?;

    let mut pda_data = config_account.try_borrow_mut_data()?;
    pda_data.copy_from_slice(&config_data.to_bytes());

    Ok(())
}
