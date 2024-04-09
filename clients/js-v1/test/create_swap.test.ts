/* eslint-disable @typescript-eslint/no-unused-vars */

import {
  generateSigner,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import test from 'ava';
import {
  ASSET_PROGRAM_ID,
  Asset,
  Discriminator,
  Standard,
  State,
  create,
  fetchAsset,
} from '@nifty-oss/asset';
import {
  SPL_TOKEN_PROGRAM_ID,
  createMintWithAssociatedToken,
  SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  findAssociatedTokenPda,
  fetchToken,
} from '@metaplex-foundation/mpl-toolbox';
import { createUmi } from './_setup';
import {
  SwapMarker,
  createSwap,
  fetchSwapMarker,
  findSwapMarkerPda,
} from '../src';

test('it can create a nifty-to-fungible asset swap', async (t) => {
  // Create a swap where the fungibles are escrowed and entangled with an
  // external Nifty asset owned by a different user.

  const umi = await createUmi();

  // Nifty asset keypair
  const asset = generateSigner(umi);
  // Owner of the Nifty asset
  const user = generateSigner(umi);
  // Creates swap and fungible token
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
  const amount = 10;
  const decimals = 2;

  const mint = generateSigner(umi);

  await createMintWithAssociatedToken(umi, {
    amount,
    decimals,
    mint,
    mintAuthority: authority,
    owner: authority.publicKey,
  }).sendAndConfirm(umi);

  // Create the swap.
  const swapMarker = findSwapMarkerPda(umi, {
    namespace: authority.publicKey,
    asset1: asset.publicKey,
    asset2: mint.publicKey,
  });

  const swapMarkerAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: toPublicKey(swapMarker),
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const ata = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: authority.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    swapMarkerAux: swapMarkerAta,
    incomingAsset: mint.publicKey, // Fungible incoming to be escrowed in the contract
    incomingAssetAux: ata,
    externalAsset: asset.publicKey, // Nifty asset to be entangled with the fungibles
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
    incomingAmount: amount, // 10 fungible tokens
    externalAmount: 1, // 1 Nifty asset
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint.publicKey,
    externalAsset: asset.publicKey,
    escrowedAmount: BigInt(amount),
    externalAmount: BigInt(1),
  });

  // The fungible tokens are now escrowed in the swap.
  const swapMarkerAtaAccount = await fetchToken(umi, swapMarkerAta);
  t.is(swapMarkerAtaAccount.amount, BigInt(amount));

  // The Nifty asset is still owned by the user.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, user.publicKey);
});

test('it can create a fungible-to-nifty asset swap', async (t) => {
  // Create a swap where the Nifty asset is escrowed and entangled with
  // external Fungible tokens owned by a different user.

  const umi = await createUmi();

  // Nifty asset keypair
  const asset = generateSigner(umi);
  // Owner of the Nifty asset and authority creating the swap
  const authority = generateSigner(umi);
  // Creator and owner of the fungible token
  const user = generateSigner(umi);

  // Create a Nifty asset.
  await create(umi, {
    asset,
    owner: authority.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: authority.publicKey,
    authority: umi.identity.publicKey,
  });

  // Create a fungible token and mint some to the authority.
  const amount = 10;
  const decimals = 2;

  const mint = generateSigner(umi);

  await createMintWithAssociatedToken(umi, {
    amount,
    decimals,
    mint,
    mintAuthority: user,
    owner: user.publicKey,
  }).sendAndConfirm(umi);

  // Create the swap.
  const swapMarker = findSwapMarkerPda(umi, {
    namespace: authority.publicKey,
    asset1: asset.publicKey,
    asset2: mint.publicKey,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    // swapMarkerAux, // Not needed because no Swap Marker ATA for Nifty asset
    incomingAsset: asset.publicKey, // Nifty asset to be escrowed
    // incomingAssetAux, // Not needed because no Nifty group on this asset
    externalAsset: mint.publicKey, // Fungibles to be entangled with the Nifty asset
    incomingAssetProgram: ASSET_PROGRAM_ID,
    // associatedTokenProgram, // Not needed because no fungible ata creation
    incomingAmount: 1, // 1 Nifty asset
    externalAmount: amount, // 10 fungible tokens
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: asset.publicKey, // Nifty asset is escrowed
    externalAsset: mint.publicKey, // Fungibles are entangled
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(amount),
  });

  // The Nifty asset is now escrowed in the swap.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, toPublicKey(swapMarker));

  // The fungible tokens are still owned by the user.
  const userAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userAtaAccount = await fetchToken(umi, userAta);
  t.is(userAtaAccount.amount, BigInt(amount));
});

test('it can create a nifty-to-nifty asset swap', async (t) => {
  // Create a swap where a Nifty asset is escrowed and entangled with another
  // Nifty asset owned by a different user.

  const umi = await createUmi();

  // Nifty asset keypair
  const asset = generateSigner(umi);
  // Owner of the Nifty asset
  const user = generateSigner(umi);

  // Authority creates the swap and another Nifty asset
  const authority = generateSigner(umi);
  const escrowedAsset = generateSigner(umi);

  // Create a Nifty  asset owned by the user.
  await create(umi, {
    asset,
    owner: user.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Create a Nifty  asset owned by the authority.
  await create(umi, {
    asset: escrowedAsset,
    owner: authority.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Create the swap.
  const swapMarker = findSwapMarkerPda(umi, {
    namespace: authority.publicKey,
    asset1: asset.publicKey,
    asset2: escrowedAsset.publicKey,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    incomingAsset: escrowedAsset.publicKey,
    externalAsset: asset.publicKey,
    incomingAssetProgram: ASSET_PROGRAM_ID,
    incomingAmount: 1, // 1 Nifty asset
    externalAmount: 1, // 1 Nifty asset
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: escrowedAsset.publicKey,
    externalAsset: asset.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });

  // The escrowed Nifty asset is now owned by the swap.
  const escrowedAssetAccount = await fetchAsset(umi, escrowedAsset.publicKey);
  t.is(escrowedAssetAccount.owner, toPublicKey(swapMarker));

  // The Nifty asset is still owned by the user.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, user.publicKey);
});

test('it can create a fungible-to-fungible asset swap', async (t) => {
  // Create a swap where one set of fungibles are escrowed and entangled with
  // external Fungible tokens owned by a different user.

  const umi = await createUmi();

  // Creator of the fungible to be escrowed and authority creating the swap
  const authority = generateSigner(umi);
  // Fungible to be escrowed
  const incomingMint = generateSigner(umi);

  // Owner of the external fungible
  const user = generateSigner(umi);
  // Fungible to be entangled
  const externalMint = generateSigner(umi);

  // Create a fungible token and mint some to the authority.
  const incomingAmount = 100;
  const externalAmount = 10;
  const decimals = 9;

  // Create mints.
  await createMintWithAssociatedToken(umi, {
    amount: incomingAmount,
    decimals,
    mint: incomingMint,
    mintAuthority: authority,
    owner: authority.publicKey,
  }).sendAndConfirm(umi);

  await createMintWithAssociatedToken(umi, {
    amount: externalAmount,
    decimals,
    mint: externalMint,
    mintAuthority: user,
    owner: user.publicKey,
  }).sendAndConfirm(umi);

  // Create the swap.
  const swapMarker = findSwapMarkerPda(umi, {
    namespace: authority.publicKey,
    asset1: incomingMint.publicKey,
    asset2: externalMint.publicKey,
  });

  const swapMarkerAta = findAssociatedTokenPda(umi, {
    mint: incomingMint.publicKey,
    owner: toPublicKey(swapMarker),
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const sourceAta = findAssociatedTokenPda(umi, {
    mint: incomingMint.publicKey,
    owner: authority.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    swapMarkerAux: swapMarkerAta,
    incomingAsset: incomingMint.publicKey,
    incomingAssetAux: sourceAta,
    externalAsset: externalMint.publicKey,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
    incomingAmount,
    externalAmount,
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: incomingMint.publicKey,
    externalAsset: externalMint.publicKey,
    escrowedAmount: BigInt(incomingAmount),
    externalAmount: BigInt(externalAmount),
  });

  // The incoming asset is now escrowed in the swap.
  const swapMarkerAtaAccount = await fetchToken(umi, swapMarkerAta);
  t.is(swapMarkerAtaAccount.owner, toPublicKey(swapMarker));
  t.is(swapMarkerAtaAccount.amount, BigInt(incomingAmount));

  // The external fungible tokens are still owned by the user.
  const userAta = findAssociatedTokenPda(umi, {
    mint: externalMint.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userAtaAccount = await fetchToken(umi, userAta);
  t.is(userAtaAccount.amount, BigInt(externalAmount));
});
