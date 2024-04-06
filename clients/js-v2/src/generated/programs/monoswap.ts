/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import { getU8Encoder } from '@solana/codecs';
import { Program, ProgramWithErrors } from '@solana/programs';
import {
  MonoswapProgramError,
  MonoswapProgramErrorCode,
  getMonoswapProgramErrorFromCode,
} from '../errors';
import {
  ParsedCreateSwapInstruction,
  ParsedSwapInstruction,
} from '../instructions';
import { memcmp } from '../shared';

export const MONOSWAP_PROGRAM_ADDRESS =
  'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G' as Address<'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G'>;

export type MonoswapProgram =
  Program<'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G'> &
    ProgramWithErrors<MonoswapProgramErrorCode, MonoswapProgramError>;

export function getMonoswapProgram(): MonoswapProgram {
  return {
    name: 'monoswap',
    address: MONOSWAP_PROGRAM_ADDRESS,
    getErrorFromCode(code: MonoswapProgramErrorCode, cause?: Error) {
      return getMonoswapProgramErrorFromCode(code, cause);
    },
  };
}

export enum MonoswapAccount {
  SwapMarker,
  SwapSeeds,
}

export enum MonoswapInstruction {
  CreateSwap,
  Swap,
}

export function identifyMonoswapInstruction(
  instruction: { data: Uint8Array } | Uint8Array
): MonoswapInstruction {
  const data =
    instruction instanceof Uint8Array ? instruction : instruction.data;
  if (memcmp(data, getU8Encoder().encode(0), 0)) {
    return MonoswapInstruction.CreateSwap;
  }
  if (memcmp(data, getU8Encoder().encode(1), 0)) {
    return MonoswapInstruction.Swap;
  }
  throw new Error(
    'The provided instruction could not be identified as a monoswap instruction.'
  );
}

export type ParsedMonoswapInstruction<
  TProgram extends string = 'F7CfUwFLRk4J1wbzknbYff7ywhT6r871FCSwTHRWfr1G',
> =
  | ({
      instructionType: MonoswapInstruction.CreateSwap;
    } & ParsedCreateSwapInstruction<TProgram>)
  | ({
      instructionType: MonoswapInstruction.Swap;
    } & ParsedSwapInstruction<TProgram>);