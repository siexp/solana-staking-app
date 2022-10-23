use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::UnixTimestamp, program_pack::IsInitialized, pubkey::Pubkey};

/// Defines schema of pool storage account
///
/// Storage size: 32 + 8 + 8 + 8 + 1 = 57 [reference](https://www.anchor-lang.com/docs/space)
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolStorageAccount {
    pub pool_authority: Pubkey,

    pub total_staked: u64,
    pub user_count: u64,
    pub rewards_per_token: u64,

    pub is_initialized: bool,
}
pub const POOL_STORAGE_TOTAL_BYTES: usize = 32 + 8 + 8 + 8 + 1;

impl IsInitialized for PoolStorageAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

/// Defines schema of user storage account
///
/// Storage size: 8 + 8 + 1 = 16 [reference](https://www.anchor-lang.com/docs/space)
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserStorageAccount {
    pub staked: u64,
    pub last_stake_timestamp: UnixTimestamp,

    pub is_initialized: bool,
}
pub const USER_STORAGE_TOTAL_BYTES: usize = 8 + 8 + 1;

impl IsInitialized for UserStorageAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
