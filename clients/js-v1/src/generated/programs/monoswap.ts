/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import { getMonoswapErrorFromCode, getMonoswapErrorFromName } from '../errors';

export const MONOSWAP_PROGRAM_ID =
  'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G' as PublicKey<'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G'>;

export function createMonoswapProgram(): Program {
  return {
    name: 'monoswap',
    publicKey: MONOSWAP_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getMonoswapErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getMonoswapErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getMonoswapProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('monoswap', clusterFilter);
}

export function getMonoswapProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'monoswap',
    MONOSWAP_PROGRAM_ID,
    clusterFilter
  );
}