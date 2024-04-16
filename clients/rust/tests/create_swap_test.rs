#![cfg(feature = "test-sbf")]

use borsh::BorshDeserialize;
use libreplex_monoswap_client::{
    accounts::{SwapMarker, SwapSeeds},
    instructions::CreateSwapBuilder,
};
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
async fn create_swap() {
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
    let ata_account = context
        .banks_client
        .get_account(swap_marker_ata)
        .await
        .unwrap()
        .unwrap();
    let ata = unpack::<Account>(&ata_account.data).unwrap();

    assert_eq!(ata.amount, 10);
    assert_eq!(ata.mint, mint);
    assert_eq!(ata.owner, swap_marker);
}
