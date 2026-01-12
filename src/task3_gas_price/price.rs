use alloy::{
    primitives::{U256, utils::format_units},
    providers::{Provider, ProviderBuilder},
};
use eyre::{Context, Result};

pub async fn gas_price() -> Result<()> {
    // 1. connect to Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse().wrap_err("Parse RPC url failed.")?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 2. query latest price
    let gas_price = U256::from(provider.get_gas_price().await.wrap_err("Query latest price failed.")?);

    // 3. estimate transaction price
    let gas_limit = U256::from(21_000);     // Gas limit for standard ETH transfers

    let estimated_fee_wei = gas_price * gas_limit;
    let estimated_fee_gwei = format_units(estimated_fee_wei, "gwei")?;
    let estimated_fee_eth = format_units(estimated_fee_wei, "ether")?;

    println!("current gas price: {} Wei", gas_price);
    println!("estimated fee: {} Gwei", estimated_fee_gwei);
    println!("estimated fee: {} ETH", estimated_fee_eth);

    Ok(())
}