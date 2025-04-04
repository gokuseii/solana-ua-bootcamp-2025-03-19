use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

fn main() {
    let keypair = Keypair::new();
    println!("Private Key: {:?}", keypair.to_bytes());
    println!("Public Key: {}", keypair.pubkey());
}
