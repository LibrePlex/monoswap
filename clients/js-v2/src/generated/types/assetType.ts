/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getScalarEnumDecoder,
  getScalarEnumEncoder,
} from '@solana/codecs';

export enum AssetType {
  Invalid,
  NiftyAsset,
  SplToken,
  MplxpNFT,
  MplxCore,
}

export type AssetTypeArgs = AssetType;

export function getAssetTypeEncoder(): Encoder<AssetTypeArgs> {
  return getScalarEnumEncoder(AssetType);
}

export function getAssetTypeDecoder(): Decoder<AssetType> {
  return getScalarEnumDecoder(AssetType);
}

export function getAssetTypeCodec(): Codec<AssetTypeArgs, AssetType> {
  return combineCodec(getAssetTypeEncoder(), getAssetTypeDecoder());
}
