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
    #[account(4, writable, name="incoming_asset", desc = "The asset to be escrowed for the swap")]
    #[account(5, name="external_asset", desc = "External asset connected to the incoming asset")]
    #[account(6, optional, writable, name="nifty_asset_group", desc = "Group account for the nifty asset, if applicable")]
    #[account(7, optional, writable, name="swap_marker_ata", desc = "ATA account for the swap marker, if applicable")]
    #[account(8, optional, writable, name="authority_ata", desc = "ATA account for the authority, if applicable")]
    #[account(9, name="incoming_asset_program", desc = "Transfer Program ID of the incoming asset")]
    #[account(10, optional, name="associated_token_program", desc = "The SPL associated token program account program")]
    #[account(11, name="system_program", desc = "System program account")]
    CreateSwap(CreateSwapArgs),

    /// Swap paired fungible assets.
    #[account(0, writable, signer, name="payer", desc = "Account to pay for ATA creation")]
    #[account(1, signer, name="authority", desc = "Authority to transfer incoming asset")]
    #[account(2, writable, name="swap_marker", desc = "Escrows the asset and encodes state about the swap")]
    #[account(3, writable, name="escrowed_asset", desc = "The currently escrowed asset")]
    #[account(4, writable, name="incoming_asset", desc = "External asset being swapped for the escrowed asset")]
    #[account(5, writable, name="swap_marker_escrowed_ata", desc = "The ATA for the escrowed asset and Swap Marker")]
    #[account(6, writable, name="swap_marker_incoming_ata", desc = "The ATA for the incoming asset and Swap Marker")]
    #[account(7, writable, name="outgoing_asset_ata", desc = "ATA for the escrowed asset and authority")]
    #[account(8, writable, name="incoming_asset_ata", desc = "ATA for the incoming asset and authority")]
    #[account(9, name="escrowed_asset_program", desc = "Transfer Program ID of the incoming asset")]
    #[account(10, name="incoming_asset_program", desc = "Transfer Program ID of the external asset")]
    #[account(11, optional, name="associated_token_program", desc = "The SPL associated token program account program")]
    #[account(12, name="system_program", desc = "System program account")]
    SwapSPL,

    /// Swap paired nifty assets.
    #[account(0, signer, name="authority", desc = "Authority to transfer incoming asset")]
    #[account(1, writable, name="swap_marker", desc = "Escrows the asset and encodes state about the swap")]
    #[account(2, writable, name="escrowed_asset", desc = "The currently escrowed asset")]
    #[account(3, writable, name="incoming_asset", desc = "External asset being swapped for the escrowed asset")]
    #[account(4, optional, writable, name="escrowed_asset_group", desc = "Group account for the escrowed asset, if applicable")]
    #[account(5, optional, writable, name="incoming_asset_group", desc = "Group account for the incoming asset, if applicable")]
    #[account(6, name="nifty_asset_program", desc = "Nifty asset program account")]
    SwapNifty,

    /// Swap paired nifty-spl assets.
    #[account(0, writable, signer, name="payer", desc = "Account to pay for ATA creation")]
    #[account(1, signer, name="authority", desc = "Authority to transfer incoming asset")]
    #[account(2, writable, name="swap_marker", desc = "Escrows the asset and encodes state about the swap")]
    #[account(3, writable, name="escrowed_asset", desc = "The currently escrowed asset")]
    #[account(4, writable, name="incoming_asset", desc = "External asset being swapped for the escrowed asset")]
    #[account(5, optional, writable, name="nifty_asset_group", desc = "Group account for the nifty asset, if applicable")]
    #[account(6, writable, name="swap_marker_ata", desc = "ATA account for the swap marker, if applicable")]
    #[account(7, writable, name="authority_ata", desc = "ATA account for the authority, if applicable")]
    #[account(8, name="escrowed_asset_program", desc = "Transfer Program ID of the incoming asset")]
    #[account(9, name="incoming_asset_program", desc = "Transfer Program ID of the external asset")]
    #[account(10, optional, name="associated_token_program", desc = "The SPL associated token program account program")]
    #[account(11, name="system_program", desc = "System program account")]
    SwapNiftySPL,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateSwapArgs {
    /// Amount of incoming asset to be escrowed.
    pub incoming_amount: u64,
    /// Amount of external asset.
    pub external_amount: u64,
}
