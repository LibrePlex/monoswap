/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type SwapSPLInstructionAccounts = {
  /** Account to pay for ATA creation */
  payer?: Signer;
  /** Authority to transfer incoming asset */
  authority?: Signer;
  /** Escrows the asset and encodes state about the swap */
  swapMarker: PublicKey | Pda;
  /** The currently escrowed asset */
  escrowedAsset: PublicKey | Pda;
  /** External asset being swapped for the escrowed asset */
  incomingAsset: PublicKey | Pda;
  /** The ATA for the escrowed asset and Swap Marker */
  swapMarkerEscrowedAta: PublicKey | Pda;
  /** The ATA for the incoming asset and Swap Marker */
  swapMarkerIncomingAta: PublicKey | Pda;
  /** ATA for the escrowed asset and authority */
  outgoingAssetAta: PublicKey | Pda;
  /** ATA for the incoming asset and authority */
  incomingAssetAta: PublicKey | Pda;
  /** Transfer Program ID of the incoming asset */
  escrowedAssetProgram: PublicKey | Pda;
  /** Transfer Program ID of the external asset */
  incomingAssetProgram: PublicKey | Pda;
  /** The SPL associated token program account program */
  associatedTokenProgram?: PublicKey | Pda;
  /** System program account */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type SwapSPLInstructionData = { discriminator: number };

export type SwapSPLInstructionDataArgs = {};

export function getSwapSPLInstructionDataSerializer(): Serializer<
  SwapSPLInstructionDataArgs,
  SwapSPLInstructionData
> {
  return mapSerializer<SwapSPLInstructionDataArgs, any, SwapSPLInstructionData>(
    struct<SwapSPLInstructionData>([['discriminator', u8()]], {
      description: 'SwapSPLInstructionData',
    }),
    (value) => ({ ...value, discriminator: 1 })
  ) as Serializer<SwapSPLInstructionDataArgs, SwapSPLInstructionData>;
}

// Instruction.
export function swapSPL(
  context: Pick<Context, 'identity' | 'payer' | 'programs'>,
  input: SwapSPLInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'monoswap',
    'MonoRPwMWxcsVEJV27jyEt1f5VoWg3szDBRYUenm221'
  );

  // Accounts.
  const resolvedAccounts = {
    payer: {
      index: 0,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    authority: {
      index: 1,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    swapMarker: {
      index: 2,
      isWritable: true as boolean,
      value: input.swapMarker ?? null,
    },
    escrowedAsset: {
      index: 3,
      isWritable: true as boolean,
      value: input.escrowedAsset ?? null,
    },
    incomingAsset: {
      index: 4,
      isWritable: true as boolean,
      value: input.incomingAsset ?? null,
    },
    swapMarkerEscrowedAta: {
      index: 5,
      isWritable: true as boolean,
      value: input.swapMarkerEscrowedAta ?? null,
    },
    swapMarkerIncomingAta: {
      index: 6,
      isWritable: true as boolean,
      value: input.swapMarkerIncomingAta ?? null,
    },
    outgoingAssetAta: {
      index: 7,
      isWritable: true as boolean,
      value: input.outgoingAssetAta ?? null,
    },
    incomingAssetAta: {
      index: 8,
      isWritable: true as boolean,
      value: input.incomingAssetAta ?? null,
    },
    escrowedAssetProgram: {
      index: 9,
      isWritable: false as boolean,
      value: input.escrowedAssetProgram ?? null,
    },
    incomingAssetProgram: {
      index: 10,
      isWritable: false as boolean,
      value: input.incomingAssetProgram ?? null,
    },
    associatedTokenProgram: {
      index: 11,
      isWritable: false as boolean,
      value: input.associatedTokenProgram ?? null,
    },
    systemProgram: {
      index: 12,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getSwapSPLInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
