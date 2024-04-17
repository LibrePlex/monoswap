use nifty_asset::instructions::TransferCpi as NiftyTransferCpi;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};

use crate::{processor::MonoswapError, require};

pub trait AssetInterface<'a, 'b> {
    fn transfer(&self, recipient: &'a AccountInfo<'a>) -> ProgramResult;
}

pub struct TransferSigner<'a, 'b> {
    account: &'a AccountInfo<'a>,
    signers_seeds: &'b [&'b [&'b [u8]]],
}

pub struct NiftyAsset<'a, 'b> {
    asset: &'a AccountInfo<'a>,
    transfer_authority: TransferSigner<'a, 'b>,
    group: Option<&'a AccountInfo<'a>>,
    program: &'a AccountInfo<'a>,
}

pub struct NiftyAssetInputs<'a, 'b> {
    pub asset: &'a AccountInfo<'a>,
    pub transfer_authority: &'a AccountInfo<'a>,
    pub group: Option<&'a AccountInfo<'a>>,
    pub program: &'a AccountInfo<'a>,
    pub signers_seeds: &'b [&'b [&'b [u8]]],
}

impl<'a, 'b> NiftyAsset<'a, 'b> {
    pub fn new(inputs: NiftyAssetInputs<'a, 'b>) -> Result<Self, MonoswapError> {
        let NiftyAssetInputs {
            asset,
            transfer_authority,
            group,
            program,
            signers_seeds,
        } = inputs;

        require!(
            asset.owner == &nifty_asset::ID,
            MonoswapError::InvalidNiftyAsset,
            "Asset account is not owned by the Nifty program"
        );

        require!(
            !asset.data_is_empty(),
            MonoswapError::InvalidNiftyAsset,
            "Asset account is empty"
        );

        require!(
            transfer_authority.is_signer || !signers_seeds.is_empty(),
            MonoswapError::InvalidSigner,
            "Authority is not a signer"
        );

        let transfer_authority = TransferSigner {
            account: transfer_authority,
            signers_seeds,
        };

        Ok(Self {
            asset,
            transfer_authority,
            group,
            program,
        })
    }
}

impl<'a, 'b> AssetInterface<'a, 'b> for NiftyAsset<'a, 'b> {
    fn transfer(&self, recipient: &'a AccountInfo<'a>) -> ProgramResult {
        msg!("Transferring Nifty asset...");

        NiftyTransferCpi {
            __program: self.program,
            asset: self.asset,
            signer: self.transfer_authority.account,
            recipient,
            group: self.group,
        }
        .invoke_signed(self.transfer_authority.signers_seeds)
    }
}
