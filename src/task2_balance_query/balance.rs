use alloy::{
    primitives::{utils::format_units, Address},
    providers::{Provider, ProviderBuilder},
};
use eyre::{Context, Result};

pub async fn get_eth_balance(target_address: Address) -> Result<String> {
    // 1. public rpc address of Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse().wrap_err("Parse RPC url failed.")?;

    // 2. create a provider
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 3. query balance (returns a U256 Type Wei)
    let balance_wei = provider
        .get_balance(target_address)
        .await
        .wrap_err("Failed to query balance.")?;

    // 4. Convert to readable format (from Wei to ETH)
    let balance_eth = format_units(balance_wei, "ether")?;

    Ok(balance_eth)
}
