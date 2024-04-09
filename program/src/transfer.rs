use nifty_asset::{accounts::Asset as NiftyAsset, instructions::TransferCpi as NiftyTransferCpi};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token_2022::{
    instruction::transfer_checked,
    state::{Account as TokenAccount, Mint},
};

use crate::{
    assertions::assert_same_pubkeys, processor::MonoswapError, state::TOKEN_PROGRAM_IDS,
    utils::unpack,
};

pub struct TransferNiftyParams<'a, 'b> {
    pub nifty_program_info: &'a AccountInfo<'a>,
    pub signer_info: &'a AccountInfo<'a>,
    pub asset_info: &'a AccountInfo<'a>,
    pub recipient_info: &'a AccountInfo<'a>,
    pub group_asset_opt_info: Option<&'a AccountInfo<'a>>,
    pub signer_seeds: &'b [&'b [&'b [u8]]],
}

pub fn check_and_transfer_nifty(params: TransferNiftyParams<'_, '_>) -> ProgramResult {
    let TransferNiftyParams {
        nifty_program_info,
        signer_info,
        asset_info,
        recipient_info,
        group_asset_opt_info,
        signer_seeds,
    } = params;

    // The incoming asset program is actually the Nifty program.
    assert_same_pubkeys(
        "incoming_asset_program",
        nifty_program_info,
        &nifty_asset::ID,
    )?;

    // Decode the Nifty asset.
    let asset = NiftyAsset::from_bytes(&asset_info.try_borrow_data()?)?;

    // If a group is present on the asset, the group asset account must be the aux account.a
    let group_asset_info_opt = if let Some(group_pub) = asset.group.to_option() {
        if group_asset_opt_info.is_none() {
            msg!("Nifty group asset is missing");
            return Err(MonoswapError::MissingNiftyGroup.into());
        }

        // We need a group so get it and make sure it matches the group stored on the Nifty account.
        let group_asset_info = group_asset_opt_info.unwrap();
        assert_same_pubkeys("nifty group asset", group_asset_info, &group_pub)?;

        Some(group_asset_info)
    } else {
        None
    };

    // Transfer Nifty asset from authority signer to the swap marker.
    NiftyTransferCpi {
        __program: nifty_program_info,
        asset: asset_info,
        signer: signer_info,
        recipient: recipient_info,
        group: group_asset_info_opt,
    }
    .invoke_signed(signer_seeds)?;
    Ok(())
}

pub struct TransferSplParams<'a, 'b> {
    pub spl_program_info: &'a AccountInfo<'a>,
    pub payer_info: &'a AccountInfo<'a>,
    pub mint_info: &'a AccountInfo<'a>,
    pub source_owner_info: &'a AccountInfo<'a>,
    pub destination_owner_info: &'a AccountInfo<'a>,
    pub source_ata_info: &'a AccountInfo<'a>,
    pub destination_ata_info: &'a AccountInfo<'a>,
    pub amount: u64,
    pub signer_seeds: &'b [&'b [&'b [u8]]],
}

pub fn check_and_transfer_spl(params: TransferSplParams<'_, '_>) -> ProgramResult {
    let TransferSplParams {
        spl_program_info,
        payer_info,
        mint_info,
        source_owner_info,
        destination_owner_info,
        source_ata_info,
        destination_ata_info,
        amount,
        signer_seeds,
    } = params;

    let mint = unpack::<Mint>(&mint_info.try_borrow_data()?)?;
    let source_ata = unpack::<TokenAccount>(&source_ata_info.try_borrow_data()?)?;

    // Checks.
    // The incoming asset program is actually one of the SPL token programs.
    if !TOKEN_PROGRAM_IDS.contains(spl_program_info.key) {
        msg!("Incoming asset program is not a valid SPL token program");
        return Err(MonoswapError::InvalidTokenProgram.into());
    }

    // Create destination ata, if necessary.
    if destination_ata_info.data_is_empty() {
        msg!("Creating destination ATA");
        // creating the associated token account
        invoke(
            &create_associated_token_account(
                payer_info.key,
                destination_owner_info.key,
                mint_info.key,
                spl_program_info.key,
            ),
            &[
                payer_info.clone(),
                mint_info.clone(),
                destination_owner_info.clone(),
                destination_ata_info.clone(),
            ],
        )?;
    }

    // ATA belongs to the mint.
    assert_same_pubkeys("mint", mint_info, &source_ata.mint)?;

    let account_infos = &[
        spl_program_info.clone(),
        source_owner_info.clone(),
        mint_info.clone(),
        source_ata_info.clone(),
        destination_ata_info.clone(),
    ];

    // Transfer SPL token from authority signer to the swap marker ATA.
    invoke_signed(
        &transfer_checked(
            spl_program_info.key,
            source_ata_info.key,
            mint_info.key,
            destination_ata_info.key,
            source_owner_info.key,
            &[],
            amount,
            mint.decimals,
        )?,
        account_infos,
        signer_seeds,
    )?;

    Ok(())
}
