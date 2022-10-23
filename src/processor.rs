use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction, system_program,
    sysvar::{rent::Rent, Sysvar},
};

use crate::instruction::Instruction;
use crate::state::{PoolStorageAccount, UserStorageAccount, USER_STORAGE_TOTAL_BYTES};
use crate::{error::StakingError, state};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::Initialize { rewards_per_token } => {
            msg!("Initialize pool");
            process_initialize_pool(program_id, accounts, rewards_per_token)
        }
        Instruction::CreateUser {} => {
            msg!("Create signer");
            process_create_user(program_id, accounts)
        }
        _ => Err(StakingError::InvalidInstruction.into()),
    }
}

fn process_initialize_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rewards_per_token: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    if !signer.is_signer {
        return Err(StakingError::InvalidSigner.into());
    }

    let storage = next_account_info(accounts_iter)?;
    if storage.owner != program_id {
        return Err(StakingError::InvalidOwner.into());
    }

    let mut user_storage_data = PoolStorageAccount::try_from_slice(&storage.data.borrow())?;
    if user_storage_data.is_initialized() {
        return Err(StakingError::AlreadyInitialized.into());
    }

    user_storage_data.pool_authority = *signer.key;
    user_storage_data.total_staked = 0u64;
    user_storage_data.user_count = 0u64;
    user_storage_data.rewards_per_token = rewards_per_token;
    user_storage_data.is_initialized = true;

    user_storage_data.serialize(&mut &mut storage.data.borrow_mut()[..])?;

    msg!("Staking pool is initialized {:#?}", user_storage_data);

    Ok(())
}

fn process_create_user(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    if !signer.is_signer {
        return Err(StakingError::InvalidSigner.into());
    }

    let user_storage_pda_account = next_account_info(accounts_iter)?;
    let (pda, bump_seed) = Pubkey::find_program_address(&[signer.key.as_ref()], program_id);
    if pda != *user_storage_pda_account.key {
        return Err(StakingError::InvalidUserStoragePda.into());
    }

    let system_program_account = next_account_info(accounts_iter)?;
    if !system_program::check_id(system_program_account.key) {
        return Err(StakingError::SystemProgramMismatch.into());
    }

    let rent_lamports = Rent::get()?.minimum_balance(USER_STORAGE_TOTAL_BYTES);
    msg!(
        "signer have to pay {} lamports for rent exemption of {} bytes",
        rent_lamports,
        state::USER_STORAGE_TOTAL_BYTES
    );

    invoke_signed(
        &system_instruction::create_account(
            signer.key,
            user_storage_pda_account.key,
            rent_lamports,
            USER_STORAGE_TOTAL_BYTES.try_into().unwrap(),
            program_id,
        ),
        &[
            signer.clone(),
            user_storage_pda_account.clone(),
            system_program_account.clone(),
        ],
        &[&[signer.key.as_ref(), &[bump_seed]]],
    )?;

    let mut user_storage_data =
        UserStorageAccount::try_from_slice(&user_storage_pda_account.data.borrow())?;
    if user_storage_data.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    user_storage_data.staked = 0;
    user_storage_data.last_stake_timestamp = 0i64;
    user_storage_data.is_initialized = true;

    user_storage_data.serialize(&mut &mut user_storage_pda_account.data.borrow_mut()[..])?;

    let pool_storage_account = next_account_info(accounts_iter)?;
    let mut pool_storage_data =
        PoolStorageAccount::try_from_slice(&pool_storage_account.data.borrow())?;
    if !pool_storage_data.is_initialized() {
        return Err(StakingError::NotInitialized.into());
    }

    pool_storage_data.user_count += 1;

    pool_storage_data.serialize(&mut &mut pool_storage_account.data.borrow_mut()[..])?;

    Ok(())
}
