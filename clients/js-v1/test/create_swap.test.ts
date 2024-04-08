/* eslint-disable @typescript-eslint/no-unused-vars */

import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  Standard,
  State,
  create,
  fetchAsset,
} from '@nifty-oss/asset';
import {
  createMint,
  // createSplTokenProgram,
  // createSplAssociatedTokenProgram,
} from '@metaplex-foundation/mpl-toolbox';
import { createUmi } from './_setup';

test('it can create a nifty-to-nifty asset swap', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();

  // Nifty asset keypair
  const asset = generateSigner(umi);
  // Owner of the asset
  const user = generateSigner(umi);
  const authority = generateSigner(umi);

  // Create a Nifty  asset owned by the user.
  await create(umi, {
    asset,
    owner: user.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: user.publicKey,
    authority: umi.identity.publicKey,
  });

  // Create a fungible token and mint some to the authority.
  // const amount = 10;
  const decimals = 2;

  const mint = generateSigner(umi);

  await createMint(umi, {
    mint,
    mintAuthority: authority.publicKey,
    decimals,
  }).sendAndConfirm(umi);

  // await createMintWithAssociatedToken(umi, {
  //   amount,
  //   decimals,
  //   mint,
  //   mintAuthority: authority,
  //   owner: authority.publicKey,
  // }).sendAndConfirm(umi);
});
