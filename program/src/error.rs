use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MonoswapError {
    /// 0 - Error deserializing an account
    #[error("Error deserializing an account")]
    DeserializationError,
    /// 1 - Error serializing an account
    #[error("Error serializing an account")]
    SerializationError,
    /// 2 - Invalid program owner
    #[error("Invalid program owner. This likely mean the provided account does not exist")]
    InvalidProgramOwner,
    /// 3 - Invalid PDA derivation
    #[error("Invalid PDA derivation")]
    InvalidPda,
    /// 4 - Expected empty account
    #[error("Expected empty account")]
    ExpectedEmptyAccount,
    /// 5 - Expected non empty account
    #[error("Expected non empty account")]
    ExpectedNonEmptyAccount,
    /// 6 - Expected signer account
    #[error("Expected signer account")]
    ExpectedSignerAccount,
    /// 7 - Expected writable account
    #[error("Expected writable account")]
    ExpectedWritableAccount,
    /// 8 - Account mismatch
    #[error("Account mismatch")]
    AccountMismatch,
    /// 9 - Invalid account key
    #[error("Invalid account key")]
    InvalidAccountKey,
    /// 10 - Numerical overflow
    #[error("Numerical overflow")]
    NumericalOverflow,
    /// 11 - Invalid system program account
    #[error("Invalid system program account")]
    InvalidSystemProgram,
    /// 12 - IninitalizedMint
    #[error("IninitalizedMint")]
    IninitalizedMint,
    /// 13 - Missing Nifty Group account
    #[error("Missing Nifty Group account")]
    MissingNiftyGroup,
    /// 14 - Missing incoming asset aux
    #[error("Missing incoming asset aux")]
    MissingIncomingAssetAux,
    /// 15 - Missing external asset aux
    #[error("Missing escrowed asset aux")]
    MissingEscrowedAssetAux,
    /// 16 - Missing swap marker aux
    #[error("Missing swap marker aux")]
    MissingSwapMarkerAux,
    /// 17 - Unsupported asset type
    #[error("Unsupported asset type")]
    UnsupportedAssetType,
}

impl PrintProgramError for MonoswapError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MonoswapError> for ProgramError {
    fn from(e: MonoswapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MonoswapError {
    fn type_of() -> &'static str {
        "Mpl Project Name Error"
    }
}
