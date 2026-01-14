use alloy::{
    network::EthereumWallet,
    primitives::{utils::parse_ether, Address},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::{Context, Result};
use std::env;
use crate::task3_gas_price::price::get_gas_info;

pub async fn transfer(to_address_str: &str, amount_eth: &str) -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // 1. get private key from env
    let private_key = env::var("PRIVATE_KEY").wrap_err("PRIVATE_KEY not set in .env file")?;

    // 2. parse recipient address and amount
    let to_address: Address = to_address_str.parse().wrap_err("Invalid recipient address")?;
    let amount_wei = parse_ether(amount_eth).wrap_err("Invalid amount")?;

    // 3. create signer and get sender address
    let signer: PrivateKeySigner = private_key.parse().wrap_err("Invalid private key")?;
    let from_address = signer.address();
    println!("transfer from addr: {}", from_address);
    println!("transfer to addr: {}", to_address);

    // 4. create provider with wallet signer
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse().wrap_err("Parse RPC url failed")?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(rpc_url);

    // 5. get gas info from task3 module (with buffers for safety)
    let gas_info = get_gas_info(to_address, amount_wei).await?;
    println!("Gas price (with 50% buffer): {} wei", gas_info.gas_price);
    println!("Estimated gas limit (with 20% buffer): {}", gas_info.estimated_gas_limit);
    
    // 6. construct transaction request with manual gas configuration
    let tx = TransactionRequest::default()
        .to(to_address)
        .value(amount_wei)
        // set gas price and gas limit manually
        .gas_price(gas_info.gas_price)
        .gas_limit(gas_info.estimated_gas_limit);

    // 7. send transaction
    println!("sending transaction...");
    let pending_tx = provider.send_transaction(tx).await.wrap_err("Transaction failed to send")?;
    let tx_hash = pending_tx.tx_hash();
    println!("transaction submitted! Hash: {}", tx_hash);

    // 8. wait for confirmation (Receipt)
    println!("waiting for confirmation...");
    let receipt = pending_tx.get_receipt().await.wrap_err("Failed to get receipt")?;
    
    println!("Transfer successful!");
    if let Some(block_number) = receipt.block_number {
        println!("Block number: {}", block_number);
    }
    println!("Gas used: {}", receipt.gas_used);

    Ok(())
}