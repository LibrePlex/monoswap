//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateSwap {
    /// Account to pay for the creation of the account
    pub payer: solana_program::pubkey::Pubkey,
    /// Indexing namespace of the marker
    pub namespace: solana_program::pubkey::Pubkey,
    /// Authority to transfer incoming asset
    pub authority: solana_program::pubkey::Pubkey,
    /// Escrows the asset and encodes state about the swap
    pub swap_marker: solana_program::pubkey::Pubkey,
    /// Auxiliary account for the swap marker: e.g. ATA
    pub swap_marker_aux: Option<solana_program::pubkey::Pubkey>,
    /// The asset to be escrowed for the swap
    pub incoming_asset: solana_program::pubkey::Pubkey,
    /// Associated account for the incoming asset, e.g. token account
    pub incoming_asset_aux: Option<solana_program::pubkey::Pubkey>,
    /// External asset connected to the incoming asset
    pub external_asset: solana_program::pubkey::Pubkey,
    /// Transfer Program ID of the incoming asset
    pub incoming_asset_program: solana_program::pubkey::Pubkey,
    /// The SPL associated token program account program
    pub associated_token_program: Option<solana_program::pubkey::Pubkey>,
    /// System program account
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateSwap {
    pub fn instruction(
        &self,
        args: CreateSwapInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateSwapInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.namespace,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.swap_marker,
            false,
        ));
        if let Some(swap_marker_aux) = self.swap_marker_aux {
            accounts.push(solana_program::instruction::AccountMeta::new(
                swap_marker_aux,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.incoming_asset,
            false,
        ));
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
            self.external_asset,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.incoming_asset_program,
            false,
        ));
        if let Some(associated_token_program) = self.associated_token_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                associated_token_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateSwapInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MONOSWAP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CreateSwapInstructionData {
    discriminator: u8,
}

impl CreateSwapInstructionData {
    fn new() -> Self {
        Self { discriminator: 0 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateSwapInstructionArgs {
    pub incoming_amount: u64,
    pub external_amount: u64,
}

/// Instruction builder for `CreateSwap`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[signer]` namespace
///   2. `[signer]` authority
///   3. `[writable]` swap_marker
///   4. `[writable, optional]` swap_marker_aux
///   5. `[writable]` incoming_asset
///   6. `[writable, optional]` incoming_asset_aux
///   7. `[]` external_asset
///   8. `[]` incoming_asset_program
///   9. `[optional]` associated_token_program
///   10. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct CreateSwapBuilder {
    payer: Option<solana_program::pubkey::Pubkey>,
    namespace: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    swap_marker: Option<solana_program::pubkey::Pubkey>,
    swap_marker_aux: Option<solana_program::pubkey::Pubkey>,
    incoming_asset: Option<solana_program::pubkey::Pubkey>,
    incoming_asset_aux: Option<solana_program::pubkey::Pubkey>,
    external_asset: Option<solana_program::pubkey::Pubkey>,
    incoming_asset_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    incoming_amount: Option<u64>,
    external_amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateSwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Account to pay for the creation of the account
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// Indexing namespace of the marker
    #[inline(always)]
    pub fn namespace(&mut self, namespace: solana_program::pubkey::Pubkey) -> &mut Self {
        self.namespace = Some(namespace);
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
    /// `[optional account]`
    /// Auxiliary account for the swap marker: e.g. ATA
    #[inline(always)]
    pub fn swap_marker_aux(
        &mut self,
        swap_marker_aux: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.swap_marker_aux = swap_marker_aux;
        self
    }
    /// The asset to be escrowed for the swap
    #[inline(always)]
    pub fn incoming_asset(&mut self, incoming_asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.incoming_asset = Some(incoming_asset);
        self
    }
    /// `[optional account]`
    /// Associated account for the incoming asset, e.g. token account
    #[inline(always)]
    pub fn incoming_asset_aux(
        &mut self,
        incoming_asset_aux: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.incoming_asset_aux = incoming_asset_aux;
        self
    }
    /// External asset connected to the incoming asset
    #[inline(always)]
    pub fn external_asset(&mut self, external_asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.external_asset = Some(external_asset);
        self
    }
    /// Transfer Program ID of the incoming asset
    #[inline(always)]
    pub fn incoming_asset_program(
        &mut self,
        incoming_asset_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.incoming_asset_program = Some(incoming_asset_program);
        self
    }
    /// `[optional account]`
    /// The SPL associated token program account program
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.associated_token_program = associated_token_program;
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program account
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn incoming_amount(&mut self, incoming_amount: u64) -> &mut Self {
        self.incoming_amount = Some(incoming_amount);
        self
    }
    #[inline(always)]
    pub fn external_amount(&mut self, external_amount: u64) -> &mut Self {
        self.external_amount = Some(external_amount);
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
        let accounts = CreateSwap {
            payer: self.payer.expect("payer is not set"),
            namespace: self.namespace.expect("namespace is not set"),
            authority: self.authority.expect("authority is not set"),
            swap_marker: self.swap_marker.expect("swap_marker is not set"),
            swap_marker_aux: self.swap_marker_aux,
            incoming_asset: self.incoming_asset.expect("incoming_asset is not set"),
            incoming_asset_aux: self.incoming_asset_aux,
            external_asset: self.external_asset.expect("external_asset is not set"),
            incoming_asset_program: self
                .incoming_asset_program
                .expect("incoming_asset_program is not set"),
            associated_token_program: self.associated_token_program,
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateSwapInstructionArgs {
            incoming_amount: self
                .incoming_amount
                .clone()
                .expect("incoming_amount is not set"),
            external_amount: self
                .external_amount
                .clone()
                .expect("external_amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_swap` CPI accounts.
pub struct CreateSwapCpiAccounts<'a, 'b> {
    /// Account to pay for the creation of the account
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Indexing namespace of the marker
    pub namespace: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority to transfer incoming asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Escrows the asset and encodes state about the swap
    pub swap_marker: &'b solana_program::account_info::AccountInfo<'a>,
    /// Auxiliary account for the swap marker: e.g. ATA
    pub swap_marker_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The asset to be escrowed for the swap
    pub incoming_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Associated account for the incoming asset, e.g. token account
    pub incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// External asset connected to the incoming asset
    pub external_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Transfer Program ID of the incoming asset
    pub incoming_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL associated token program account program
    pub associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// System program account
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_swap` CPI instruction.
pub struct CreateSwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account to pay for the creation of the account
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Indexing namespace of the marker
    pub namespace: &'b solana_program::account_info::AccountInfo<'a>,
    /// Authority to transfer incoming asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Escrows the asset and encodes state about the swap
    pub swap_marker: &'b solana_program::account_info::AccountInfo<'a>,
    /// Auxiliary account for the swap marker: e.g. ATA
    pub swap_marker_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The asset to be escrowed for the swap
    pub incoming_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Associated account for the incoming asset, e.g. token account
    pub incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// External asset connected to the incoming asset
    pub external_asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Transfer Program ID of the incoming asset
    pub incoming_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL associated token program account program
    pub associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// System program account
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateSwapInstructionArgs,
}

impl<'a, 'b> CreateSwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateSwapCpiAccounts<'a, 'b>,
        args: CreateSwapInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            namespace: accounts.namespace,
            authority: accounts.authority,
            swap_marker: accounts.swap_marker,
            swap_marker_aux: accounts.swap_marker_aux,
            incoming_asset: accounts.incoming_asset,
            incoming_asset_aux: accounts.incoming_asset_aux,
            external_asset: accounts.external_asset,
            incoming_asset_program: accounts.incoming_asset_program,
            associated_token_program: accounts.associated_token_program,
            system_program: accounts.system_program,
            __args: args,
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
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.namespace.key,
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
        if let Some(swap_marker_aux) = self.swap_marker_aux {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *swap_marker_aux.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.incoming_asset.key,
            false,
        ));
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
            *self.external_asset.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.incoming_asset_program.key,
            false,
        ));
        if let Some(associated_token_program) = self.associated_token_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *associated_token_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MONOSWAP_ID,
                false,
            ));
        }
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
        let mut data = CreateSwapInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MONOSWAP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.namespace.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.swap_marker.clone());
        if let Some(swap_marker_aux) = self.swap_marker_aux {
            account_infos.push(swap_marker_aux.clone());
        }
        account_infos.push(self.incoming_asset.clone());
        if let Some(incoming_asset_aux) = self.incoming_asset_aux {
            account_infos.push(incoming_asset_aux.clone());
        }
        account_infos.push(self.external_asset.clone());
        account_infos.push(self.incoming_asset_program.clone());
        if let Some(associated_token_program) = self.associated_token_program {
            account_infos.push(associated_token_program.clone());
        }
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

/// Instruction builder for `CreateSwap` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[signer]` namespace
///   2. `[signer]` authority
///   3. `[writable]` swap_marker
///   4. `[writable, optional]` swap_marker_aux
///   5. `[writable]` incoming_asset
///   6. `[writable, optional]` incoming_asset_aux
///   7. `[]` external_asset
///   8. `[]` incoming_asset_program
///   9. `[optional]` associated_token_program
///   10. `[]` system_program
pub struct CreateSwapCpiBuilder<'a, 'b> {
    instruction: Box<CreateSwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateSwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateSwapCpiBuilderInstruction {
            __program: program,
            payer: None,
            namespace: None,
            authority: None,
            swap_marker: None,
            swap_marker_aux: None,
            incoming_asset: None,
            incoming_asset_aux: None,
            external_asset: None,
            incoming_asset_program: None,
            associated_token_program: None,
            system_program: None,
            incoming_amount: None,
            external_amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Account to pay for the creation of the account
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// Indexing namespace of the marker
    #[inline(always)]
    pub fn namespace(
        &mut self,
        namespace: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.namespace = Some(namespace);
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
    /// `[optional account]`
    /// Auxiliary account for the swap marker: e.g. ATA
    #[inline(always)]
    pub fn swap_marker_aux(
        &mut self,
        swap_marker_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.swap_marker_aux = swap_marker_aux;
        self
    }
    /// The asset to be escrowed for the swap
    #[inline(always)]
    pub fn incoming_asset(
        &mut self,
        incoming_asset: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.incoming_asset = Some(incoming_asset);
        self
    }
    /// `[optional account]`
    /// Associated account for the incoming asset, e.g. token account
    #[inline(always)]
    pub fn incoming_asset_aux(
        &mut self,
        incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.incoming_asset_aux = incoming_asset_aux;
        self
    }
    /// External asset connected to the incoming asset
    #[inline(always)]
    pub fn external_asset(
        &mut self,
        external_asset: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.external_asset = Some(external_asset);
        self
    }
    /// Transfer Program ID of the incoming asset
    #[inline(always)]
    pub fn incoming_asset_program(
        &mut self,
        incoming_asset_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.incoming_asset_program = Some(incoming_asset_program);
        self
    }
    /// `[optional account]`
    /// The SPL associated token program account program
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.associated_token_program = associated_token_program;
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
    #[inline(always)]
    pub fn incoming_amount(&mut self, incoming_amount: u64) -> &mut Self {
        self.instruction.incoming_amount = Some(incoming_amount);
        self
    }
    #[inline(always)]
    pub fn external_amount(&mut self, external_amount: u64) -> &mut Self {
        self.instruction.external_amount = Some(external_amount);
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
        let args = CreateSwapInstructionArgs {
            incoming_amount: self
                .instruction
                .incoming_amount
                .clone()
                .expect("incoming_amount is not set"),
            external_amount: self
                .instruction
                .external_amount
                .clone()
                .expect("external_amount is not set"),
        };
        let instruction = CreateSwapCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer.expect("payer is not set"),

            namespace: self.instruction.namespace.expect("namespace is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            swap_marker: self
                .instruction
                .swap_marker
                .expect("swap_marker is not set"),

            swap_marker_aux: self.instruction.swap_marker_aux,

            incoming_asset: self
                .instruction
                .incoming_asset
                .expect("incoming_asset is not set"),

            incoming_asset_aux: self.instruction.incoming_asset_aux,

            external_asset: self
                .instruction
                .external_asset
                .expect("external_asset is not set"),

            incoming_asset_program: self
                .instruction
                .incoming_asset_program
                .expect("incoming_asset_program is not set"),

            associated_token_program: self.instruction.associated_token_program,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateSwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    namespace: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_marker: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_marker_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_asset_aux: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    external_asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_asset_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    incoming_amount: Option<u64>,
    external_amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
