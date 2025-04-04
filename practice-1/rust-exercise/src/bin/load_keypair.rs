use dotenv::dotenv;
use serde_json;
use solana_sdk::signature::{Keypair, Signer};
use std::env;

fn main() {
    dotenv().ok();

    let raw = env::var("SECRET_KEY").expect("SECRET_KEY is not provided");

    let bytes: Vec<u8> = serde_json::from_str(&raw).expect("Failed to parse key array");

    let keypair = Keypair::from_bytes(&bytes).expect("Invalid keypair bytes");

    println!("Public Key: {}", keypair.pubkey());
}
