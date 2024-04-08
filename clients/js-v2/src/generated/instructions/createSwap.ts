/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { BASE_ACCOUNT_SIZE } from '@solana/accounts';
import { Address } from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  mapEncoder,
} from '@solana/codecs';
import {
  IAccountMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
  ReadonlySignerAccount,
  WritableAccount,
  WritableSignerAccount,
} from '@solana/instructions';
import { IAccountSignerMeta, TransactionSigner } from '@solana/signers';
import { getSwapMarkerSize } from '../accounts';
import { MONOSWAP_PROGRAM_ADDRESS } from '../programs';
import {
  IInstructionWithByteDelta,
  ResolvedAccount,
  getAccountMetaFactory,
} from '../shared';

export type CreateSwapInstruction<
  TProgram extends string = typeof MONOSWAP_PROGRAM_ADDRESS,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountNamespace extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountSwapMarker extends string | IAccountMeta<string> = string,
  TAccountSwapMarkerAux extends string | IAccountMeta<string> = string,
  TAccountIncomingAsset extends string | IAccountMeta<string> = string,
  TAccountIncomingAssetAux extends string | IAccountMeta<string> = string,
  TAccountExternalAsset extends string | IAccountMeta<string> = string,
  TAccountIncomingAssetProgram extends string | IAccountMeta<string> = string,
  TAccountAssociatedTokenProgram extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountNamespace extends string
        ? ReadonlySignerAccount<TAccountNamespace> &
            IAccountSignerMeta<TAccountNamespace>
        : TAccountNamespace,
      TAccountAuthority extends string
        ? ReadonlySignerAccount<TAccountAuthority> &
            IAccountSignerMeta<TAccountAuthority>
        : TAccountAuthority,
      TAccountSwapMarker extends string
        ? WritableAccount<TAccountSwapMarker>
        : TAccountSwapMarker,
      TAccountSwapMarkerAux extends string
        ? WritableAccount<TAccountSwapMarkerAux>
        : TAccountSwapMarkerAux,
      TAccountIncomingAsset extends string
        ? WritableAccount<TAccountIncomingAsset>
        : TAccountIncomingAsset,
      TAccountIncomingAssetAux extends string
        ? WritableAccount<TAccountIncomingAssetAux>
        : TAccountIncomingAssetAux,
      TAccountExternalAsset extends string
        ? ReadonlyAccount<TAccountExternalAsset>
        : TAccountExternalAsset,
      TAccountIncomingAssetProgram extends string
        ? ReadonlyAccount<TAccountIncomingAssetProgram>
        : TAccountIncomingAssetProgram,
      TAccountAssociatedTokenProgram extends string
        ? ReadonlyAccount<TAccountAssociatedTokenProgram>
        : TAccountAssociatedTokenProgram,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CreateSwapInstructionData = {
  discriminator: number;
  incomingAmount: bigint;
  externalAmount: bigint;
};

export type CreateSwapInstructionDataArgs = {
  incomingAmount: number | bigint;
  externalAmount: number | bigint;
};

export function getCreateSwapInstructionDataEncoder(): Encoder<CreateSwapInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['incomingAmount', getU64Encoder()],
      ['externalAmount', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 0 })
  );
}

export function getCreateSwapInstructionDataDecoder(): Decoder<CreateSwapInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['incomingAmount', getU64Decoder()],
    ['externalAmount', getU64Decoder()],
  ]);
}

export function getCreateSwapInstructionDataCodec(): Codec<
  CreateSwapInstructionDataArgs,
  CreateSwapInstructionData
> {
  return combineCodec(
    getCreateSwapInstructionDataEncoder(),
    getCreateSwapInstructionDataDecoder()
  );
}

export type CreateSwapInput<
  TAccountPayer extends string = string,
  TAccountNamespace extends string = string,
  TAccountAuthority extends string = string,
  TAccountSwapMarker extends string = string,
  TAccountSwapMarkerAux extends string = string,
  TAccountIncomingAsset extends string = string,
  TAccountIncomingAssetAux extends string = string,
  TAccountExternalAsset extends string = string,
  TAccountIncomingAssetProgram extends string = string,
  TAccountAssociatedTokenProgram extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** Account to pay for the creation of the account */
  payer: TransactionSigner<TAccountPayer>;
  /** Indexing namespace of the marker */
  namespace: TransactionSigner<TAccountNamespace>;
  /** Authority to transfer incoming asset */
  authority: TransactionSigner<TAccountAuthority>;
  /** Escrows the asset and encodes state about the swap */
  swapMarker: Address<TAccountSwapMarker>;
  /** Auxiliary account for the swap marker: e.g. ATA */
  swapMarkerAux?: Address<TAccountSwapMarkerAux>;
  /** The asset to be escrowed for the swap */
  incomingAsset: Address<TAccountIncomingAsset>;
  /** Associated account for the incoming asset, e.g. token account */
  incomingAssetAux?: Address<TAccountIncomingAssetAux>;
  /** External asset connected to the incoming asset */
  externalAsset: Address<TAccountExternalAsset>;
  /** Transfer Program ID of the incoming asset */
  incomingAssetProgram: Address<TAccountIncomingAssetProgram>;
  /** The SPL associated token program account program */
  associatedTokenProgram?: Address<TAccountAssociatedTokenProgram>;
  /** System program account */
  systemProgram?: Address<TAccountSystemProgram>;
  incomingAmount: CreateSwapInstructionDataArgs['incomingAmount'];
  externalAmount: CreateSwapInstructionDataArgs['externalAmount'];
};

export function getCreateSwapInstruction<
  TAccountPayer extends string,
  TAccountNamespace extends string,
  TAccountAuthority extends string,
  TAccountSwapMarker extends string,
  TAccountSwapMarkerAux extends string,
  TAccountIncomingAsset extends string,
  TAccountIncomingAssetAux extends string,
  TAccountExternalAsset extends string,
  TAccountIncomingAssetProgram extends string,
  TAccountAssociatedTokenProgram extends string,
  TAccountSystemProgram extends string,
>(
  input: CreateSwapInput<
    TAccountPayer,
    TAccountNamespace,
    TAccountAuthority,
    TAccountSwapMarker,
    TAccountSwapMarkerAux,
    TAccountIncomingAsset,
    TAccountIncomingAssetAux,
    TAccountExternalAsset,
    TAccountIncomingAssetProgram,
    TAccountAssociatedTokenProgram,
    TAccountSystemProgram
  >
): CreateSwapInstruction<
  typeof MONOSWAP_PROGRAM_ADDRESS,
  TAccountPayer,
  TAccountNamespace,
  TAccountAuthority,
  TAccountSwapMarker,
  TAccountSwapMarkerAux,
  TAccountIncomingAsset,
  TAccountIncomingAssetAux,
  TAccountExternalAsset,
  TAccountIncomingAssetProgram,
  TAccountAssociatedTokenProgram,
  TAccountSystemProgram
> &
  IInstructionWithByteDelta {
  // Program address.
  const programAddress = MONOSWAP_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payer: { value: input.payer ?? null, isWritable: true },
    namespace: { value: input.namespace ?? null, isWritable: false },
    authority: { value: input.authority ?? null, isWritable: false },
    swapMarker: { value: input.swapMarker ?? null, isWritable: true },
    swapMarkerAux: { value: input.swapMarkerAux ?? null, isWritable: true },
    incomingAsset: { value: input.incomingAsset ?? null, isWritable: true },
    incomingAssetAux: {
      value: input.incomingAssetAux ?? null,
      isWritable: true,
    },
    externalAsset: { value: input.externalAsset ?? null, isWritable: false },
    incomingAssetProgram: {
      value: input.incomingAssetProgram ?? null,
      isWritable: false,
    },
    associatedTokenProgram: {
      value: input.associatedTokenProgram ?? null,
      isWritable: false,
    },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  // Bytes created or reallocated by the instruction.
  const byteDelta: number = [getSwapMarkerSize() + BASE_ACCOUNT_SIZE].reduce(
    (a, b) => a + b,
    0
  );

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.namespace),
      getAccountMeta(accounts.authority),
      getAccountMeta(accounts.swapMarker),
      getAccountMeta(accounts.swapMarkerAux),
      getAccountMeta(accounts.incomingAsset),
      getAccountMeta(accounts.incomingAssetAux),
      getAccountMeta(accounts.externalAsset),
      getAccountMeta(accounts.incomingAssetProgram),
      getAccountMeta(accounts.associatedTokenProgram),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCreateSwapInstructionDataEncoder().encode(
      args as CreateSwapInstructionDataArgs
    ),
  } as CreateSwapInstruction<
    typeof MONOSWAP_PROGRAM_ADDRESS,
    TAccountPayer,
    TAccountNamespace,
    TAccountAuthority,
    TAccountSwapMarker,
    TAccountSwapMarkerAux,
    TAccountIncomingAsset,
    TAccountIncomingAssetAux,
    TAccountExternalAsset,
    TAccountIncomingAssetProgram,
    TAccountAssociatedTokenProgram,
    TAccountSystemProgram
  >;

  return Object.freeze({ ...instruction, byteDelta });
}

export type ParsedCreateSwapInstruction<
  TProgram extends string = typeof MONOSWAP_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Account to pay for the creation of the account */
    payer: TAccountMetas[0];
    /** Indexing namespace of the marker */
    namespace: TAccountMetas[1];
    /** Authority to transfer incoming asset */
    authority: TAccountMetas[2];
    /** Escrows the asset and encodes state about the swap */
    swapMarker: TAccountMetas[3];
    /** Auxiliary account for the swap marker: e.g. ATA */
    swapMarkerAux?: TAccountMetas[4] | undefined;
    /** The asset to be escrowed for the swap */
    incomingAsset: TAccountMetas[5];
    /** Associated account for the incoming asset, e.g. token account */
    incomingAssetAux?: TAccountMetas[6] | undefined;
    /** External asset connected to the incoming asset */
    externalAsset: TAccountMetas[7];
    /** Transfer Program ID of the incoming asset */
    incomingAssetProgram: TAccountMetas[8];
    /** The SPL associated token program account program */
    associatedTokenProgram?: TAccountMetas[9] | undefined;
    /** System program account */
    systemProgram: TAccountMetas[10];
  };
  data: CreateSwapInstructionData;
};

export function parseCreateSwapInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedCreateSwapInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 11) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  const getNextOptionalAccount = () => {
    const accountMeta = getNextAccount();
    return accountMeta.address === MONOSWAP_PROGRAM_ADDRESS
      ? undefined
      : accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      payer: getNextAccount(),
      namespace: getNextAccount(),
      authority: getNextAccount(),
      swapMarker: getNextAccount(),
      swapMarkerAux: getNextOptionalAccount(),
      incomingAsset: getNextAccount(),
      incomingAssetAux: getNextOptionalAccount(),
      externalAsset: getNextAccount(),
      incomingAssetProgram: getNextAccount(),
      associatedTokenProgram: getNextOptionalAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCreateSwapInstructionDataDecoder().decode(instruction.data),
  };
}
