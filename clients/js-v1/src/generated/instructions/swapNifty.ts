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
export type SwapNiftyInstructionAccounts = {
  /** Authority to transfer incoming asset */
  authority?: Signer;
  /** Escrows the asset and encodes state about the swap */
  swapMarker: PublicKey | Pda;
  /** The currently escrowed asset */
  escrowedAsset: PublicKey | Pda;
  /** External asset being swapped for the escrowed asset */
  incomingAsset: PublicKey | Pda;
  /** Group account for the escrowed asset, if applicable */
  escrowedAssetGroup?: PublicKey | Pda;
  /** Group account for the incoming asset, if applicable */
  incomingAssetGroup?: PublicKey | Pda;
  /** Nifty asset program account */
  niftyAssetProgram: PublicKey | Pda;
};

// Data.
export type SwapNiftyInstructionData = { discriminator: number };

export type SwapNiftyInstructionDataArgs = {};

export function getSwapNiftyInstructionDataSerializer(): Serializer<
  SwapNiftyInstructionDataArgs,
  SwapNiftyInstructionData
> {
  return mapSerializer<
    SwapNiftyInstructionDataArgs,
    any,
    SwapNiftyInstructionData
  >(
    struct<SwapNiftyInstructionData>([['discriminator', u8()]], {
      description: 'SwapNiftyInstructionData',
    }),
    (value) => ({ ...value, discriminator: 2 })
  ) as Serializer<SwapNiftyInstructionDataArgs, SwapNiftyInstructionData>;
}

// Instruction.
export function swapNifty(
  context: Pick<Context, 'identity' | 'programs'>,
  input: SwapNiftyInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'monoswap',
    'MonojHG3jNB5W9TC8mZm49aJbRXxjsnPzgVWj9j9hu5'
  );

  // Accounts.
  const resolvedAccounts = {
    authority: {
      index: 0,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    swapMarker: {
      index: 1,
      isWritable: true as boolean,
      value: input.swapMarker ?? null,
    },
    escrowedAsset: {
      index: 2,
      isWritable: true as boolean,
      value: input.escrowedAsset ?? null,
    },
    incomingAsset: {
      index: 3,
      isWritable: true as boolean,
      value: input.incomingAsset ?? null,
    },
    escrowedAssetGroup: {
      index: 4,
      isWritable: true as boolean,
      value: input.escrowedAssetGroup ?? null,
    },
    incomingAssetGroup: {
      index: 5,
      isWritable: true as boolean,
      value: input.incomingAssetGroup ?? null,
    },
    niftyAssetProgram: {
      index: 6,
      isWritable: false as boolean,
      value: input.niftyAssetProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
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
  const data = getSwapNiftyInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
