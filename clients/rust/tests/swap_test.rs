#![cfg(feature = "test-sbf")]

use borsh::BorshDeserialize;
use libreplex_monoswap_client::{
    accounts::{SwapMarker, SwapSeeds},
    instructions::{CreateSwapBuilder, SwapNiftySPLBuilder},
};
use nifty_asset::accounts::Asset;
use solana_program_test::tokio;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub mod helpers;
use helpers::*;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token_2022::state::Account;

#[tokio::test]
async fn create() {
    let mut context = program_test().start_with_context().await;

    // Authority creates the fungible and the nifty swap account
    let authority_signer = Keypair::new();
    let authority = authority_signer.pubkey();

    // User owns the Nifty asset and does the swapping
    let user_signer = Keypair::new();
    let user = user_signer.pubkey();

    // Fund the authority and user
    airdrop(&mut context, &authority, 1_000_000_000)
        .await
        .unwrap();
    airdrop(&mut context, &user, 1_000_000_000).await.unwrap();

    let FungibleTest { mint, ata } =
        create_fungible_token(&mut context, &authority_signer, 10, TokenProgram::Legacy)
            .await
            .unwrap();

    let AssetTest { asset } = create_nifty_asset(&mut context, &user_signer, user)
        .await
        .unwrap();

    let swap_marker = SwapMarker::find_pda(SwapSeeds {
        namespace: authority,
        asset1: mint,
        asset2: asset,
    })
    .0;

    let user_ata = get_associated_token_address_with_program_id(&user, &mint, &spl_token::ID);
    let swap_marker_ata =
        get_associated_token_address_with_program_id(&swap_marker, &mint, &spl_token::ID);

    println!("Swap marker: {:?}", swap_marker);
    println!(
        "Swap marker aux: {:?}",
        get_associated_token_address_with_program_id(&swap_marker, &mint, &spl_token::ID)
    );
    println!("Mint: {:?}", mint);
    println!("Asset: {:?}", asset);
    println!("ATA: {:?}", ata);
    println!("Authority: {:?}", authority);
    println!("Payer: {:?}", context.payer.pubkey());

    let ix = CreateSwapBuilder::new()
        .payer(context.payer.pubkey())
        .namespace(authority)
        .authority(authority)
        .swap_marker(swap_marker)
        .swap_marker_ata(Some(swap_marker_ata))
        .incoming_asset(mint)
        .authority_ata(Some(ata))
        .external_asset(asset)
        .incoming_asset_program(spl_token::ID)
        .associated_token_program(Some(spl_associated_token_account::ID))
        .incoming_amount(10)
        .external_amount(1)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then an account was created with the correct data.

    let account = context.banks_client.get_account(swap_marker).await.unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    assert_eq!(account.data.len(), SwapMarker::LEN);

    let mut account_data = account.data.as_ref();
    let swap_marker_account = SwapMarker::deserialize(&mut account_data).unwrap();
    assert_eq!(swap_marker_account.namespace, authority);
    assert_eq!(swap_marker_account.escrowed_asset, mint);
    assert_eq!(swap_marker_account.external_asset, asset);

    // Fungibles are escrowed by the swap marker.
    let swap_marker_ata_account = context
        .banks_client
        .get_account(swap_marker_ata)
        .await
        .unwrap()
        .unwrap();
    let marker = unpack::<Account>(&swap_marker_ata_account.data).unwrap();

    assert_eq!(marker.amount, 10);
    assert_eq!(marker.mint, mint);
    assert_eq!(marker.owner, swap_marker);

    // Nifty asset is escrowed by the user still.
    let asset_account = context
        .banks_client
        .get_account(asset)
        .await
        .unwrap()
        .unwrap();
    let asset_data = Asset::deserialize(&mut asset_account.data.as_ref()).unwrap();

    assert_eq!(asset_data.owner, user);

    // Swap Nifty asset for fungibles.
    let ix = SwapNiftySPLBuilder::new()
        .payer(context.payer.pubkey())
        .authority(user) // User has the authority to transfer the incoming nifty asset
        .swap_marker(swap_marker)
        .swap_marker_ata(swap_marker_ata) // ATA where fungibles are escrowed
        .incoming_asset(asset) // Nifty asset incoming
        .nifty_asset_group(None) // No Nifty group
        .escrowed_asset(mint) // Escrowed fungibles mint
        .authority_ata(user_ata) // Escrowed mint ata owned by the user
        .incoming_asset_program(nifty_asset::ID) // Nifty program needed to transfer the asset
        .escrowed_asset_program(spl_token::ID) // SPL token program needed to transfer the fungibles
        .associated_token_program(Some(spl_associated_token_account::ID))
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &user_signer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Nifty asset is escrowed by the swap marker.
    let asset_account = context
        .banks_client
        .get_account(asset)
        .await
        .unwrap()
        .unwrap();
    let asset_data = Asset::deserialize(&mut asset_account.data.as_ref()).unwrap();

    assert_eq!(asset_data.owner, swap_marker);

    // Fungibles are now owned by the user.
    let user_ata_account = context
        .banks_client
        .get_account(user_ata)
        .await
        .unwrap()
        .unwrap();
    let uata = unpack::<Account>(&user_ata_account.data).unwrap();

    assert_eq!(uata.amount, 10);
    assert_eq!(uata.mint, mint);
    assert_eq!(uata.owner, user);
}
