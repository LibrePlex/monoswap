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
  // escrowed. If the first byte is the same, compare the second byte, and so on.
  const a1Bytes = publicKeySerializer().serialize(seeds.asset1);
  const a2Bytes = publicKeySerializer().serialize(seeds.asset2);

  let pda;

  for (let i = 0; i < a1Bytes.length; i++) {
    if (a1Bytes[i] < a2Bytes[i]) {
      pda = context.eddsa.findPda(programId, [
        string({ size: 'variable' }).serialize('swap_marker'),
        publicKeySerializer().serialize(seeds.namespace),
        a1Bytes,
        a2Bytes,
      ]);
      break;
    } else if (a1Bytes[i] > a2Bytes[i]) {
      pda = context.eddsa.findPda(programId, [
        string({ size: 'variable' }).serialize('swap_marker'),
        publicKeySerializer().serialize(seeds.namespace),
        a2Bytes,
        a1Bytes,
      ]);
      break;
    }
  }

  if (!pda) {
    throw new Error('Asset1 and Asset2 are the same');
  }

  return pda;
}
