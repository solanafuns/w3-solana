use crate::instruction::NameConfig;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::{self, Pubkey},
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

pub fn name_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: &String,
    program: pubkey::Pubkey,
    default_page: String,
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
        default_page,
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
