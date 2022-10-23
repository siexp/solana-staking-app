use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Instruction {
    /// Accounts Expected:
    ///
    /// 0. `[signer]` Pool Owner Wallet Account
    /// 1. `[writable]` Pool Storage Account
    Initialize { rewards_per_token: u64 },
    /// Accounts Expected:
    ///
    /// 0. `[signer]` User Wallet Account
    /// 1. `[writable]` User Storage PDA Account
    /// 2. `[writable]` Pool Storage Account
    /// 3. `[]` System Program
    CreateUser {},
    /// Accounts Expected:
    ///
    /// 0. `[signer]` User Wallet Account
    /// 1. `[writable]` User Storage PDA Account
    /// 2. `[writable]` Pool Storage Account
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` ATA to Debit (User)
    /// 5. `[writable]` ATA to Credit (Pool)
    /// 6. `[]` Token Program
    Stake { amount: u64 },
    /// Accounts Expected:
    ///
    /// 0. `[signer]` User Wallet Account
    /// 1. `[writable]` User Storage PDA Account
    /// 2. `[writable]` Pool Storage Account
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` ATA to Debit (Pool)
    /// 5. `[writable]` ATA to Credit (User)
    /// 6. `[]` Token Program
    Unstake { amount: u64 },
    /// Accounts Expected:
    ///
    /// 0. `[signer]` User Wallet Account
    /// 1. `[writable]` User Storage PDA Account
    /// 2. `[writable]` Pool Storage Account
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` ATA to Debit (Pool)
    /// 5. `[writable]` ATA to Credit (User)
    /// 6. `[]` Token Program
    Claim {},
}
