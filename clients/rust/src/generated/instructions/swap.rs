//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Swap {
    /// Account to pay for any accounts that need to be created
    pub payer: solana_program::pubkey::Pubkey,
    /// Authority to transfer incoming asset
    pub authority: solana_program::pubkey::Pubkey,
    /// Escrows the asset and encodes state about the swap
    pub swap_marker: solana_program::pubkey::Pubkey,
    /// The asset to be escrowed for the swap
    pub escrowed_asset: solana_program::pubkey::Pubkey,
    /// External asset connected to the incoming asset
    pub incoming_asset: solana_program::pubkey::Pubkey,
    /// Auxiliary account for the swap marker: e.g. ATA
    pub swap_marker_aux: solana_program::pubkey::Pubkey,
    /// Associated account for the incoming asset, e.g. token account
    pub escrowed_asset_aux: Option<solana_program::pubkey::Pubkey>,
    /// Associated account for the external asset, e.g. token account
    pub incoming_asset_aux: Option<solana_program::pubkey::Pubkey>,
    /// Transfer Program ID of the incoming asset
    pub escrowed_asset_program: solana_program::pubkey::Pubkey,
    /// Transfer Program ID of the external asset
    pub incoming_asset_program: solana_program::pubkey::Pubkey,
    /// The SPL associated token program account program
    pub associated_token_program: solana_program::pubkey::Pubkey,
    /// System program account
    pub system_program: solana_program::pubkey::Pubkey,
}

impl Swap {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.swap_marker,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.escrowed_asset,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.incoming_asset,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.swap_marker_aux,
            false,
        ));
        if let Some(escrowed_asset_aux) = self.escrowed_asset_aux {
            accounts.push(solana_program::instruction::AccountMeta::new(
                escrowed_asset_aux,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        if let Some(incoming_asset_aux) = self.incoming_asset_aux {
            accounts.push(solana_program::instruction::AccountMeta::new(
                incoming_asset_aux,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.escrowed_asset_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.incoming_asset_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = SwapInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MONOSWAP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct SwapInstructionData {
    discriminator: u8,
}

impl SwapInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

/// Instruction builder for `Swap`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[signer]` authority
///   2. `[writable]` swap_marker
///   3. `[writable]` escrowed_asset
///   4. `[writable]` incoming_asset
///   5. `[writable]` swap_marker_aux
///   6. `[writable, optional]` escrowed_asset_aux
///   7. `[writable, optional]` incoming_asset_aux
///   8. `[]` escrowed_asset_program
///   9. `[]` incoming_asset_program
///   10. `[]` associated_token_program
///   11. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct SwapBuilder {
    payer: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    swap_marker: Option<solana_program::pubkey::Pubkey>,
    escrowed_asset: Option<solana_program::pubkey::Pubkey>,
    incoming_asset: Option<solana_program::pubkey::Pubkey>,
    swap_marker_aux: Option<solana_program::pubkey::Pubkey>,
    escrowed_asset_aux: Option<solana_program::pubkey::Pubkey>,
    incoming_asset_aux: Option<solana_program::pubkey::Pubkey>,
    escrowed_asset_program: Option<solana_program::pubkey::Pubkey>,
    incoming_asset_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Account to pay for any accounts that need to be created
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// Authority to transfer incoming asset
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// Escrows the asset and encodes state about the swap
    #[inline(always)]
    pub fn swap_marker(&mut self, swap_marker: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_marker = Some(swap_marker);
        self
    }
    /// The asset to be escrowed for the swap
    #[inline(always)]
    pub fn escrowed_asset(&mut self, escrowed_asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.escrowed_asset = Some(escrowed_asset);
        self
    }
    /// External asset connected to the incoming asset
    #[inline(always)]
    pub fn incoming_asset(&mut self, incoming_asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.incoming_asset = Some(incoming_asset);
        self
    }
    /// Auxiliary account for the swap marker: e.g. ATA
    #[inline(always)]
    pub fn swap_marker_aux(
        &mut self,
        swap_marker_aux: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.swap_marker_aux = Some(swap_marker_aux);
        self
    }
    /// `[optional account]`
    /// Associated account for the incoming asset, e.g. token account
    #[inline(always)]
    pub fn escrowed_asset_aux(
        &mut self,
        escrowed_asset_aux: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.escrowed_asset_aux = escrowed_asset_aux;
        self
    }
    /// `[optional account]`
    /// Associated account for the external asset, e.g. token account
    #[inline(always)]
    pub fn incoming_asset_aux(
        &mut self,
        incoming_asset_aux: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.incoming_asset_aux = incoming_asset_aux;
        self
    }
    /// Transfer Program ID of the incoming asset
    #[inline(always)]
    pub fn escrowed_asset_program(
        &mut self,
        escrowed_asset_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.escrowed_asset_program = Some(escrowed_asset_program);
        self
    }
    /// Transfer Program ID of the external asset
    #[inline(always)]
    pub fn incoming_asset_program(
        &mut self,
        incoming_asset_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.incoming_asset_program = Some(incoming_asset_program);
        self
    }
    /// The SPL associated token program account program
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program account
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Swap {
            payer: self.payer.expect("payer is not set"),
            authority: self.authority.expect("authority is not set"),
            swap_marker: self.swap_marker.expect("swap_marker is not set"),
            escrowed_asset: self.escrowed_asset.expect("escrowed_asset is not set"),
            incoming_asset: self.incoming_asset.expect("incoming_asset is not set"),
            swap_marker_aux: self.swap_marker_aux.expect("swap_marker_aux is not set"),
            escrowed_asset_aux: self.escrowed_asset_aux,
            incoming_asset_aux: self.incoming_asset_aux,
            escrowed_asset_program: self
                .escrowed_asset_program
                .expect("escrowed_asset_program is not set"),
            incoming_asset_program: self
                .incoming_asset_program
                .expect("incoming_asset_program is not set"),
            associated_token_program: self
                .associated_token_program
                .expect("associated_token_program is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `swap` CPI accounts.
pub struct SwapCpiAccounts<'a, 'b> {
    /// Account to pay for any accounts that need to be created
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority to transfer incoming asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Escrows the asset and encodes state about the swap
    pub swap_marker: &'b solana_program::account_info::AccountInfo<'a>,
    /// The asset to be escrowed for the swap
    pub escrowed_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// External asset connected to the incoming asset
    pub incoming_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Auxiliary account for the swap marker: e.g. ATA
    pub swap_marker_aux: &'b solana_program::account_info::AccountInfo<'a>,
    /// Associated account for the incoming asset, e.g. token account
    pub escrowed_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Associated account for the external asset, e.g. token account
    pub incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Transfer Program ID of the incoming asset
    pub escrowed_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Transfer Program ID of the external asset
    pub incoming_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL associated token program account program
    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program account
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `swap` CPI instruction.
pub struct SwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account to pay for any accounts that need to be created
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority to transfer incoming asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Escrows the asset and encodes state about the swap
    pub swap_marker: &'b solana_program::account_info::AccountInfo<'a>,
    /// The asset to be escrowed for the swap
    pub escrowed_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// External asset connected to the incoming asset
    pub incoming_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Auxiliary account for the swap marker: e.g. ATA
    pub swap_marker_aux: &'b solana_program::account_info::AccountInfo<'a>,
    /// Associated account for the incoming asset, e.g. token account
    pub escrowed_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Associated account for the external asset, e.g. token account
    pub incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Transfer Program ID of the incoming asset
    pub escrowed_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Transfer Program ID of the external asset
    pub incoming_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL associated token program account program
    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program account
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> SwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SwapCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            authority: accounts.authority,
            swap_marker: accounts.swap_marker,
            escrowed_asset: accounts.escrowed_asset,
            incoming_asset: accounts.incoming_asset,
            swap_marker_aux: accounts.swap_marker_aux,
            escrowed_asset_aux: accounts.escrowed_asset_aux,
            incoming_asset_aux: accounts.incoming_asset_aux,
            escrowed_asset_program: accounts.escrowed_asset_program,
            incoming_asset_program: accounts.incoming_asset_program,
            associated_token_program: accounts.associated_token_program,
            system_program: accounts.system_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.swap_marker.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.escrowed_asset.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.incoming_asset.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.swap_marker_aux.key,
            false,
        ));
        if let Some(escrowed_asset_aux) = self.escrowed_asset_aux {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *escrowed_asset_aux.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        if let Some(incoming_asset_aux) = self.incoming_asset_aux {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *incoming_asset_aux.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.escrowed_asset_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.incoming_asset_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = SwapInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MONOSWAP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.swap_marker.clone());
        account_infos.push(self.escrowed_asset.clone());
        account_infos.push(self.incoming_asset.clone());
        account_infos.push(self.swap_marker_aux.clone());
        if let Some(escrowed_asset_aux) = self.escrowed_asset_aux {
            account_infos.push(escrowed_asset_aux.clone());
        }
        if let Some(incoming_asset_aux) = self.incoming_asset_aux {
            account_infos.push(incoming_asset_aux.clone());
        }
        account_infos.push(self.escrowed_asset_program.clone());
        account_infos.push(self.incoming_asset_program.clone());
        account_infos.push(self.associated_token_program.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Swap` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[signer]` authority
///   2. `[writable]` swap_marker
///   3. `[writable]` escrowed_asset
///   4. `[writable]` incoming_asset
///   5. `[writable]` swap_marker_aux
///   6. `[writable, optional]` escrowed_asset_aux
///   7. `[writable, optional]` incoming_asset_aux
///   8. `[]` escrowed_asset_program
///   9. `[]` incoming_asset_program
///   10. `[]` associated_token_program
///   11. `[]` system_program
pub struct SwapCpiBuilder<'a, 'b> {
    instruction: Box<SwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SwapCpiBuilderInstruction {
            __program: program,
            payer: None,
            authority: None,
            swap_marker: None,
            escrowed_asset: None,
            incoming_asset: None,
            swap_marker_aux: None,
            escrowed_asset_aux: None,
            incoming_asset_aux: None,
            escrowed_asset_program: None,
            incoming_asset_program: None,
            associated_token_program: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Account to pay for any accounts that need to be created
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// Authority to transfer incoming asset
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// Escrows the asset and encodes state about the swap
    #[inline(always)]
    pub fn swap_marker(
        &mut self,
        swap_marker: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_marker = Some(swap_marker);
        self
    }
    /// The asset to be escrowed for the swap
    #[inline(always)]
    pub fn escrowed_asset(
        &mut self,
        escrowed_asset: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.escrowed_asset = Some(escrowed_asset);
        self
    }
    /// External asset connected to the incoming asset
    #[inline(always)]
    pub fn incoming_asset(
        &mut self,
        incoming_asset: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.incoming_asset = Some(incoming_asset);
        self
    }
    /// Auxiliary account for the swap marker: e.g. ATA
    #[inline(always)]
    pub fn swap_marker_aux(
        &mut self,
        swap_marker_aux: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_marker_aux = Some(swap_marker_aux);
        self
    }
    /// `[optional account]`
    /// Associated account for the incoming asset, e.g. token account
    #[inline(always)]
    pub fn escrowed_asset_aux(
        &mut self,
        escrowed_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.escrowed_asset_aux = escrowed_asset_aux;
        self
    }
    /// `[optional account]`
    /// Associated account for the external asset, e.g. token account
    #[inline(always)]
    pub fn incoming_asset_aux(
        &mut self,
        incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.incoming_asset_aux = incoming_asset_aux;
        self
    }
    /// Transfer Program ID of the incoming asset
    #[inline(always)]
    pub fn escrowed_asset_program(
        &mut self,
        escrowed_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.escrowed_asset_program = Some(escrowed_asset_program);
        self
    }
    /// Transfer Program ID of the external asset
    #[inline(always)]
    pub fn incoming_asset_program(
        &mut self,
        incoming_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.incoming_asset_program = Some(incoming_asset_program);
        self
    }
    /// The SPL associated token program account program
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    /// System program account
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = SwapCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer.expect("payer is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            swap_marker: self
                .instruction
                .swap_marker
                .expect("swap_marker is not set"),

            escrowed_asset: self
                .instruction
                .escrowed_asset
                .expect("escrowed_asset is not set"),

            incoming_asset: self
                .instruction
                .incoming_asset
                .expect("incoming_asset is not set"),

            swap_marker_aux: self
                .instruction
                .swap_marker_aux
                .expect("swap_marker_aux is not set"),

            escrowed_asset_aux: self.instruction.escrowed_asset_aux,

            incoming_asset_aux: self.instruction.incoming_asset_aux,

            escrowed_asset_program: self
                .instruction
                .escrowed_asset_program
                .expect("escrowed_asset_program is not set"),

            incoming_asset_program: self
                .instruction
                .incoming_asset_program
                .expect("incoming_asset_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct SwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_marker: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    escrowed_asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_marker_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    escrowed_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    escrowed_asset_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_asset_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
