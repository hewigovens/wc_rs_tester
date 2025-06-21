use alloy_signer_local::{MnemonicBuilder, coins_bip39::English};
use anyhow::Result;
use wallet_core_rs::{
    tw_encoding::hex::{self, ToHex},
    tw_hash,
    tw_keypair::tw::{PrivateKey, PublicKeyType},
};

fn main() -> Result<()> {
    // Generate a random wallet (24 word phrase) at custom derivation path.
    let wallet = MnemonicBuilder::<English>::default()
        .word_count(24)
        .derivation_path("m/44'/60'/0'/0/0")?
        .build_random()?;

    println!("Random wallet: {}", wallet.address());

    let bytes = wallet.to_bytes();
    println!("Random wallet private key: {}", bytes.to_hex());
    let key = PrivateKey::new(bytes.to_vec()).unwrap();

    let public_key = key
        .get_public_key_by_type(PublicKeyType::Secp256k1Extended)
        .unwrap();
    println!(
        "Random wallet public key: {}",
        hex::encode(public_key.to_bytes(), false)
    );

    let hash = tw_hash::sha3::keccak256(&public_key.to_bytes()[1..]);
    println!(
        "Random wallet public key hash: {}",
        hex::encode(hash, false)
    );

    Ok(())
}
