use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{pubkey::Pubkey, program_pack::IsInitialized};

/// Define the type of state stored in 
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolStorageAccount {
    pub pool_authority: Pubkey,

    pub total_staked: u64,
    pub user_count: u64,
    pub rewards_per_token: u64,

    pub is_initialized: bool,
}

impl IsInitialized for PoolStorageAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}