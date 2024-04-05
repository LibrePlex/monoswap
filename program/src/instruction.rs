use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum MonoswapInstruction {
    /// Create paired swap assets.
    #[account(0, writable, signer, name="payer", desc = "Account to pay for the creation of the account")]
    #[account(1, signer, name="namespace", desc = "Indexing namespace of the marker")]
    #[account(2, signer, name="authority", desc = "Authority to transfer incoming asset")]
    #[account(3, writable, name="swap_marker", desc = "Escrows the asset and encodes state about the swap")]
    #[account(4, writable, name="swap_marker_aux", desc = "Auxiliary account for the swap marker: e.g. ATA")]
    #[account(5, name="incoming_asset", desc = "The asset to be escrowed for the swap")]
    #[account(6, writable, optional, name="incoming_asset_aux", desc = "Associated account for the incoming asset, e.g. token account")]
    #[account(7, name="external_asset", desc = "External asset connected to the incoming asset")]
    #[account(8, name="incoming_asset_program", desc = "Transfer Program ID of the incoming asset")]
    #[account(9, name="associated_token_program", desc = "The SPL associated token program account program")]
    #[account(10, name="system_program", desc = "System program account")]
    CreateSwap(CreateSwapArgs),

    /// Swap paired assets.
    ///  #[account(2, writable, name="swap_marker", desc = "Escrows the asset and encodes state about the swap")]
    #[account(0, signer, name="authority", desc = "Authority to transfer incoming asset")]
    #[account(1, writable, name="swap_marker", desc = "Escrows the asset and encodes state about the swap")]
    #[account(2, writable, name="escrowed_asset", desc = "The asset to be escrowed for the swap")]
    #[account(3, name="incoming_asset", desc = "External asset connected to the incoming asset")]
    #[account(4, writable, name="swap_marker_aux", desc = "Auxiliary account for the swap marker: e.g. ATA")]
    #[account(5, optional, name="escrowed_asset_aux", desc = "Associated account for the incoming asset, e.g. token account")]
    #[account(6, optional, name="incoming_asset_aux", desc = "Associated account for the external asset, e.g. token account")]
    #[account(7, name="escrowed_asset_program", desc = "Transfer Program ID of the incoming asset")]
    #[account(8, name="incoming_asset_program", desc = "Transfer Program ID of the external asset")]
    Swap,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateSwapArgs {
    /// Amount of incoming asset to be escrowed.
    pub incoming_amount: u64,
    /// Amount of external asset.
    pub external_amount: u64,
}
