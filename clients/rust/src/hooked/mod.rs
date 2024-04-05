use crate::accounts::{SwapMarker, SwapSeeds};
use solana_program::pubkey::Pubkey;

impl SwapMarker {
    pub fn find_pda(seeds: SwapSeeds) -> (Pubkey, u8) {
        let SwapSeeds {
            namespace,
            asset1,
            asset2,
        } = seeds;

        // The account with the smaller first byte is the first seed, the allows the swap marker
        // to be reversible in that it has the same order regardless of which asset is incoming
        // or escrowed.
        let (smaller, larger) =
            if asset1.to_bytes().first().unwrap() < asset2.to_bytes().first().unwrap() {
                (asset1, asset2)
            } else {
                (asset2, asset1)
            };
        Pubkey::find_program_address(
            &[
                b"swap_marker",
                &namespace.to_bytes(),
                smaller.as_ref(),
                larger.as_ref(),
            ],
            &crate::ID,
        )
    }
}
