import { Context, Pda, PublicKey } from '@metaplex-foundation/umi';
import {
  publicKey as publicKeySerializer,
  string,
} from '@metaplex-foundation/umi/serializers';

export function findSwapMarkerPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    /** The address of the pool owner */
    namespace: PublicKey;
    /** The address of the pool owner */
    asset1: PublicKey;
    /** The address of the pool owner */
    asset2: PublicKey;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'monoswap',
    'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G'
  );

  // Compare the first byte of asset1 and asset2 and find the smaller one to
  // maintain consistent seed ordering regardless of which asset is incoming versus
  // escrowed.
  const a1Bytes = publicKeySerializer().serialize(seeds.asset1);
  const a2Bytes = publicKeySerializer().serialize(seeds.asset2);
  const a1 = a1Bytes[0];
  const a2 = a2Bytes[0];
  const smaller = a1 < a2 ? a1Bytes : a2Bytes;
  const larger = a1 < a2 ? a2Bytes : a1Bytes;

  return context.eddsa.findPda(programId, [
    string({ size: 'variable' }).serialize('swap_marker'),
    publicKeySerializer().serialize(seeds.namespace),
    smaller,
    larger,
  ]);
}
