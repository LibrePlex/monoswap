pub mod assertions;
pub mod asset_detection;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod transfer;
pub mod utils;

pub use solana_program;

solana_program::declare_id!("MonoRPwMWxcsVEJV27jyEt1f5VoWg3szDBRYUenm221");

// Variable naming convention:
//
// Typical types found in processor functions:
// AccountInfo -- denoted with the suffix _info; e.g., authority_info
// Pubkey -- denoted with the suffix _pub; e.g., authority_pub
// Option<AccountInfo> -- denoted with the suffix _info_opt; e.g., authority_opt_info
// Option<Pubkey> -- denoted with the suffix _opt; e.g., authority_opt_pub
// Account structs such as Mint, Account, Asset -- no suffix; e.g., mint, token_account, asset
// AccountInfos stored found in the ctx struct are distinguishable by the path to them:
// ctx.accounts.authority.
