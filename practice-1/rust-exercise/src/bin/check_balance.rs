use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin check_balance <PUBKEY>");
        std::process::exit(1);
    }

    let pubkey = Pubkey::from_str(&args[1]).expect("Invalid pubkey");

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let lamports = client.get_balance(&pubkey).expect("Failed to get balance");
    let sol = lamports as f64 / 1_000_000_000.0;

    println!("Balance: {} SOL", sol);
}
