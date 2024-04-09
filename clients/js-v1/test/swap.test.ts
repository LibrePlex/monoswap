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
  swap,
} from '../src';

test('it can swap nifty-to-fungible and back', async (t) => {
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
  let swapMarkerAtaAccount = await fetchToken(umi, swapMarkerAta);
  t.is(swapMarkerAtaAccount.amount, BigInt(amount));

  // The Nifty asset is still owned by the user.
  let assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap the fungibles back to the user in exchange for the Nifty asset.

  // We need the user's ATA so we can send the fungibles back to them.
  const userAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await swap(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerAux: swapMarkerAta,
    incomingAsset: asset.publicKey, // Nifty asset is incoming to be escrowed in the contract
    escrowedAsset: mint.publicKey, // Fungible asset to be sent back to the user
    escrowedAssetAux: userAta,
    incomingAssetProgram: ASSET_PROGRAM_ID,
    escrowedAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // The fungible tokens are now owned by the user.
  const userAtaAccount = await fetchToken(umi, userAta);
  t.is(userAtaAccount.amount, BigInt(amount));

  // The Nifty asset is now owned by the swap marker.
  const transferredAssetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(transferredAssetAccount.owner, toPublicKey(swapMarker));

  // Swap back.
  await swap(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerAux: swapMarkerAta,
    incomingAsset: mint.publicKey,
    incomingAssetAux: userAta,
    escrowedAsset: asset.publicKey,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    escrowedAssetProgram: ASSET_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // The fungible tokens are now escrowed in the swap.
  swapMarkerAtaAccount = await fetchToken(umi, swapMarkerAta);
  t.is(swapMarkerAtaAccount.amount, BigInt(amount));

  // The Nifty asset is owned by the user again.
  assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, user.publicKey);
});

test('it can swap nifty-to-nifty and back', async (t) => {
  // Create a swap where a Nifty asset is escrowed and entangled with another
  // Nifty asset owned by a different user.

  const umi = await createUmi();

  // Nifty asset keypair
  const asset1 = generateSigner(umi);
  // Owner of the Nifty asset
  const user = generateSigner(umi);

  // Authority creates the swap and another Nifty asset
  const authority = generateSigner(umi);
  const asset2 = generateSigner(umi);

  // Create a Nifty  asset owned by the user.
  await create(umi, {
    asset: asset1,
    owner: user.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Create a Nifty  asset owned by the authority.
  await create(umi, {
    asset: asset2,
    owner: authority.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Create the swap.
  const swapMarker = findSwapMarkerPda(umi, {
    namespace: authority.publicKey,
    asset1: asset1.publicKey,
    asset2: asset2.publicKey,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    incomingAsset: asset2.publicKey,
    externalAsset: asset1.publicKey,
    incomingAssetProgram: ASSET_PROGRAM_ID,
    incomingAmount: 1, // 1 Nifty asset
    externalAmount: 1, // 1 Nifty asset
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: asset2.publicKey,
    externalAsset: asset1.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });

  // The asset2 is now owned by the swap.
  let escrowedAssetAccount = await fetchAsset(umi, asset2.publicKey);
  t.is(escrowedAssetAccount.owner, toPublicKey(swapMarker));

  // The asset1 is still owned by the user.
  let assetAccount = await fetchAsset(umi, asset1.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap them.
  await swap(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    incomingAsset: asset1.publicKey,
    escrowedAsset: asset2.publicKey,
    incomingAssetProgram: ASSET_PROGRAM_ID,
    escrowedAssetProgram: ASSET_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Assets are reversed.
  escrowedAssetAccount = await fetchAsset(umi, asset1.publicKey);
  t.is(escrowedAssetAccount.owner, toPublicKey(swapMarker));

  // The asset1 is still owned by the user.
  assetAccount = await fetchAsset(umi, asset2.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap back.
  await swap(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    incomingAsset: asset2.publicKey,
    escrowedAsset: asset1.publicKey,
    incomingAssetProgram: ASSET_PROGRAM_ID,
    escrowedAssetProgram: ASSET_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Back to the original state.
  escrowedAssetAccount = await fetchAsset(umi, asset2.publicKey);
  t.is(escrowedAssetAccount.owner, toPublicKey(swapMarker));

  assetAccount = await fetchAsset(umi, asset1.publicKey);
  t.is(assetAccount.owner, user.publicKey);
});

test('it swap fungible-to-fungible and back', async (t) => {
  // Create a swap where one set of fungibles are escrowed and entangled with
  // external Fungible tokens owned by a different user.

  const umi = await createUmi();

  // Creator of the fungible to be escrowed and authority creating the swap
  const authority = generateSigner(umi);
  // Fungible to be escrowed
  const mint1 = generateSigner(umi);

  // Owner of the external fungible
  const user = generateSigner(umi);
  // Fungible to be entangled
  const mint2 = generateSigner(umi);

  // Create a fungible token and mint some to the authority.
  const incomingAmount = 100;
  const externalAmount = 10;
  const decimals = 9;

  // Create mints.
  await createMintWithAssociatedToken(umi, {
    amount: incomingAmount,
    decimals,
    mint: mint1,
    mintAuthority: authority,
    owner: authority.publicKey,
  }).sendAndConfirm(umi);

  await createMintWithAssociatedToken(umi, {
    amount: externalAmount,
    decimals,
    mint: mint2,
    mintAuthority: user,
    owner: user.publicKey,
  }).sendAndConfirm(umi);

  // Create the swap.
  const swapMarker = findSwapMarkerPda(umi, {
    namespace: authority.publicKey,
    asset1: mint1.publicKey,
    asset2: mint2.publicKey,
  });

  const swapMarkerMint1Ata = findAssociatedTokenPda(umi, {
    mint: mint1.publicKey,
    owner: toPublicKey(swapMarker),
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const sourceAta = findAssociatedTokenPda(umi, {
    mint: mint1.publicKey,
    owner: authority.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    swapMarkerAux: swapMarkerMint1Ata, // We transfer to this.
    incomingAsset: mint1.publicKey,
    incomingAssetAux: sourceAta,
    externalAsset: mint2.publicKey,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
    incomingAmount,
    externalAmount,
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint1.publicKey,
    externalAsset: mint2.publicKey,
    escrowedAmount: BigInt(incomingAmount),
    externalAmount: BigInt(externalAmount),
  });

  // The incoming asset is now escrowed in the swap.
  const swapMarkerAtaAccount = await fetchToken(umi, swapMarkerMint1Ata);
  t.is(swapMarkerAtaAccount.owner, toPublicKey(swapMarker));
  t.is(swapMarkerAtaAccount.amount, BigInt(incomingAmount));

  // The external fungible tokens are still owned by the user.
  const userMint2Ata = findAssociatedTokenPda(umi, {
    mint: mint2.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userAtaAccount = await fetchToken(umi, userMint2Ata);
  t.is(userAtaAccount.amount, BigInt(externalAmount));

  // Swap the fungibles.

  // Swap marker ata now for Mint2
  const swapMarkerMint2Ata = findAssociatedTokenPda(umi, {
    mint: mint2.publicKey,
    owner: toPublicKey(swapMarker),
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await swap(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerAux: swapMarkerMint2Ata, // New escrow ATA--transfer mint2 *to* this
    incomingAsset: mint2.publicKey,
    incomingAssetAux: userMint2Ata, // Transfer *from* this
    escrowedAsset: mint1.publicKey,
    escrowedAssetAux: swapMarkerMint1Ata, // Current escrow ATA: transfer mint1 *from* this
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    escrowedAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);
});
