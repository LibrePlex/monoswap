/* eslint-disable @typescript-eslint/no-unused-vars */

import {
  generateSigner,
  percentAmount,
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
import { createNft } from '@metaplex-foundation/mpl-token-metadata';
import { createUmi } from './_setup';
import {
  SwapMarker,
  createSwap,
  fetchSwapMarker,
  findSwapMarkerPda,
  swapNifty,
  swapNiftySPL,
  swapSPL,
} from '../src';

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
    authority: umi.identity.publicKey,
    owner: user.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Create a Nifty  asset owned by the authority.
  await create(umi, {
    asset: asset2,
    authority: umi.identity.publicKey,
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
  await swapNifty(umi, {
    authority: user,
    swapMarker,
    incomingAsset: asset1.publicKey,
    escrowedAsset: asset2.publicKey,
    niftyAssetProgram: ASSET_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Assets are reversed.
  escrowedAssetAccount = await fetchAsset(umi, asset1.publicKey);
  t.is(escrowedAssetAccount.owner, toPublicKey(swapMarker));

  // The asset1 is still owned by the user.
  assetAccount = await fetchAsset(umi, asset2.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: asset1.publicKey,
    externalAsset: asset2.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });

  // Swap back.
  await swapNifty(umi, {
    authority: user,
    swapMarker,
    incomingAsset: asset2.publicKey,
    escrowedAsset: asset1.publicKey,
    niftyAssetProgram: ASSET_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Back to the original state.
  escrowedAssetAccount = await fetchAsset(umi, asset2.publicKey);
  t.is(escrowedAssetAccount.owner, toPublicKey(swapMarker));

  assetAccount = await fetchAsset(umi, asset1.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: asset2.publicKey,
    externalAsset: asset1.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });
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
  const mint1Amount = 100;
  const mint2Amount = 10;
  const decimals = 9;

  // Create mints.
  await createMintWithAssociatedToken(umi, {
    amount: mint1Amount,
    decimals,
    mint: mint1,
    mintAuthority: authority,
    owner: authority.publicKey,
  }).sendAndConfirm(umi);

  await createMintWithAssociatedToken(umi, {
    amount: mint2Amount,
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

  // Swap marker ata now for Mint2
  const swapMarkerMint2Ata = findAssociatedTokenPda(umi, {
    mint: mint2.publicKey,
    owner: toPublicKey(swapMarker),
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const authorityMint1Ata = findAssociatedTokenPda(umi, {
    mint: mint1.publicKey,
    owner: authority.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userMint1Ata = findAssociatedTokenPda(umi, {
    mint: mint1.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userMint2Ata = findAssociatedTokenPda(umi, {
    mint: mint2.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    swapMarkerAta: swapMarkerMint1Ata, // We transfer to this.
    incomingAsset: mint1.publicKey,
    authorityAta: authorityMint1Ata,
    externalAsset: mint2.publicKey,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
    incomingAmount: mint1Amount,
    externalAmount: mint2Amount,
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint1.publicKey,
    externalAsset: mint2.publicKey,
    escrowedAmount: BigInt(mint1Amount),
    externalAmount: BigInt(mint2Amount),
  });

  // The incoming asset is now escrowed in the swap.
  const swapMarkerAtaAccount = await fetchToken(umi, swapMarkerMint1Ata);
  t.is(swapMarkerAtaAccount.owner, toPublicKey(swapMarker));
  t.is(swapMarkerAtaAccount.amount, BigInt(mint1Amount));

  const userAtaAccount = await fetchToken(umi, userMint2Ata);
  t.is(userAtaAccount.amount, BigInt(mint2Amount));

  /* Swap the fungibles.

 The swap requires four ATAs:

 incomingAssetAta:           [authority, incoming asset]
 outgoingAssetAta            [authority, escrowed asset]
 swapMarkerIncomingAta:      [swap marker, incoming asset]
 swapMarkerEscrowedAssetAta: [swap marker, escrowed asset]

 The transfer:
 Incoming asset:
 incomingAssetAta --> swapMarkerIncomingAta

 Escrowed/outgoing asset:
 swapMarkerEscrowedAssetAta --> outgoingAssetAta
*/

  await swapSPL(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerEscrowedAta: swapMarkerMint1Ata,
    swapMarkerIncomingAta: swapMarkerMint2Ata,
    escrowedAsset: mint1.publicKey,
    incomingAsset: mint2.publicKey,
    outgoingAssetAta: userMint1Ata,
    incomingAssetAta: userMint2Ata,
    escrowedAssetProgram: SPL_TOKEN_PROGRAM_ID,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Mint 2 tokens are now now escrowed on the swap.
  const swapMarkerMint2AtaAccount = await fetchToken(umi, swapMarkerMint2Ata);
  t.is(swapMarkerMint2AtaAccount.owner, toPublicKey(swapMarker));
  t.is(swapMarkerMint2AtaAccount.amount, BigInt(mint2Amount));

  // Mint 1 tokens are owned by the user.
  const userMint1AtaAccount = await fetchToken(umi, userMint1Ata);
  t.is(userMint1AtaAccount.owner, toPublicKey(user.publicKey));
  t.is(userMint1AtaAccount.amount, BigInt(mint1Amount));

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint2.publicKey,
    externalAsset: mint1.publicKey,
    escrowedAmount: BigInt(mint2Amount),
    externalAmount: BigInt(mint1Amount),
  });

  // Swap back.
  await swapSPL(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerEscrowedAta: swapMarkerMint2Ata,
    swapMarkerIncomingAta: swapMarkerMint1Ata,
    escrowedAsset: mint2.publicKey,
    incomingAsset: mint1.publicKey,
    outgoingAssetAta: userMint2Ata,
    incomingAssetAta: userMint1Ata,
    escrowedAssetProgram: SPL_TOKEN_PROGRAM_ID,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Mint 1 tokens are now now escrowed on the swap again.
  const swapMarkerMint1AtaAccount = await fetchToken(umi, swapMarkerMint1Ata);
  t.is(swapMarkerMint1AtaAccount.owner, toPublicKey(swapMarker));
  t.is(swapMarkerMint1AtaAccount.amount, BigInt(mint1Amount));

  // Mint 2 tokens are owned by the user.
  const userMint2AtaAccount = await fetchToken(umi, userMint2Ata);
  t.is(userMint2AtaAccount.owner, toPublicKey(user.publicKey));
  t.is(userMint2AtaAccount.amount, BigInt(mint2Amount));

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint1.publicKey,
    externalAsset: mint2.publicKey,
    escrowedAmount: BigInt(mint1Amount),
    externalAmount: BigInt(mint2Amount),
  });
});

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
    authority: umi.identity.publicKey,
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

  const authorityAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: authority.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    swapMarkerAta,
    incomingAsset: mint.publicKey, // Fungible incoming to be escrowed in the contract
    authorityAta,
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

  await swapNiftySPL(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    incomingAsset: asset.publicKey, // Nifty asset is incoming to be escrowed in the contract
    escrowedAsset: mint.publicKey, // Fungible asset to be sent back to the user
    swapMarkerAta,
    authorityAta: userAta,
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

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: asset.publicKey,
    externalAsset: mint.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(amount),
  });

  // Swap back.
  await swapNiftySPL(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerAta,
    incomingAsset: mint.publicKey,
    authorityAta: userAta,
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

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint.publicKey,
    externalAsset: asset.publicKey,
    escrowedAmount: BigInt(amount),
    externalAmount: BigInt(1),
  });
});

test('it can swap nifty-to-mplx-legacy and back', async (t) => {
  // Create a swap where a Metaplex legacy NFT is escrowed and entangled with an
  // external Nifty asset owned by a different user.

  const umi = await createUmi();

  // Nifty asset keypair
  const asset = generateSigner(umi);
  // Owner of the Nifty asset
  const user = generateSigner(umi);
  // Creates swap and Metaplex NFT
  const authority = generateSigner(umi);

  // Create a Nifty  asset owned by the user.
  await create(umi, {
    asset,
    authority: umi.identity.publicKey,
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

  const mint = generateSigner(umi);
  await createNft(umi, {
    mint,
    tokenOwner: authority.publicKey,
    name: 'My NFT',
    uri: 'https://example.com/my-nft.json',
    sellerFeeBasisPoints: percentAmount(5.5),
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

  const authorityAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: authority.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  const userAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: user.publicKey,
    tokenProgramId: SPL_TOKEN_PROGRAM_ID,
  });

  await createSwap(umi, {
    payer: umi.identity,
    namespace: authority,
    authority,
    swapMarker,
    swapMarkerAta,
    incomingAsset: mint.publicKey, // Fungible incoming to be escrowed in the contract
    authorityAta,
    externalAsset: asset.publicKey, // Nifty asset to be entangled with the fungibles
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
    incomingAmount: 1, // 1 Metaplex NFT
    externalAmount: 1, // 1 Nifty asset
  }).sendAndConfirm(umi);

  // Then the swap was created with the correct data.
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint.publicKey,
    externalAsset: asset.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });

  // The fungible tokens are now escrowed in the swap.
  let swapMarkerAtaAccount = await fetchToken(umi, swapMarkerAta);
  t.is(swapMarkerAtaAccount.amount, BigInt(1));

  // The Nifty asset is still owned by the user.
  let assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap the fungibles back to the user in exchange for the Nifty asset.

  await swapNiftySPL(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    incomingAsset: asset.publicKey, // Nifty asset is incoming to be escrowed in the contract
    escrowedAsset: mint.publicKey, // Fungible asset to be sent back to the user
    swapMarkerAta,
    authorityAta: userAta,
    incomingAssetProgram: ASSET_PROGRAM_ID,
    escrowedAssetProgram: SPL_TOKEN_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // The fungible tokens are now owned by the user.
  const userAtaAccount = await fetchToken(umi, userAta);
  t.is(userAtaAccount.amount, BigInt(1));

  // The Nifty asset is now owned by the swap marker.
  const transferredAssetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(transferredAssetAccount.owner, toPublicKey(swapMarker));

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: asset.publicKey,
    externalAsset: mint.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });

  // Swap back.
  await swapNiftySPL(umi, {
    payer: umi.identity,
    authority: user,
    swapMarker,
    swapMarkerAta,
    incomingAsset: mint.publicKey,
    authorityAta: userAta,
    escrowedAsset: asset.publicKey,
    incomingAssetProgram: SPL_TOKEN_PROGRAM_ID,
    escrowedAssetProgram: ASSET_PROGRAM_ID,
    associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // The fungible tokens are now escrowed in the swap.
  swapMarkerAtaAccount = await fetchToken(umi, swapMarkerAta);
  t.is(swapMarkerAtaAccount.amount, BigInt(1));

  // The Nifty asset is owned by the user again.
  assetAccount = await fetchAsset(umi, asset.publicKey);
  t.is(assetAccount.owner, user.publicKey);

  // Swap Marker state is as expected
  t.like(await fetchSwapMarker(umi, swapMarker), <SwapMarker>{
    namespace: authority.publicKey,
    escrowedAsset: mint.publicKey,
    externalAsset: asset.publicKey,
    escrowedAmount: BigInt(1),
    externalAmount: BigInt(1),
  });
});
