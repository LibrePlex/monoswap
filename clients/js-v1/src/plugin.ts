import { UmiPlugin } from '@metaplex-foundation/umi';
import { createMonoswapProgram } from './generated';

export const monoswap = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createMonoswapProgram(), false);
  },
});
