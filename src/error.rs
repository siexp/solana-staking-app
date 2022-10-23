use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Invalid signer
    #[error("Invalid signer")]
    InvalidSigner,
    /// Invalid owner
    #[error("Invalid owner")]
    InvalidOwner,
    /// Account already initialized
    #[error("Account already initialized")]
    AlreadyInitialized,
    /// Invalid user storage PDA account
    #[error("Invalid user storage PDA")]
    InvalidUserStoragePda,
    /// Invalid SystemProgram account
    #[error("Invalid SystemProgram account")]
    SystemProgramMismatch,
    /// Account is not initialized
    #[error("Account is not initialized")]
    NotInitialized,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
