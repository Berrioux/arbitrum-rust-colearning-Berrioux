use alloy::{
    primitives::{U256, utils::format_units, Address},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
};
use eyre::{Context, Result};

pub struct GasInfo {
    pub gas_price: u128,
    pub estimated_gas_limit: u64,
}

/// Get current gas price and estimate gas limit for a transfer
pub async fn get_gas_info(to_address: Address, amount_wei: U256) -> Result<GasInfo> {
    // 1. connect to Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse().wrap_err("Parse RPC url failed.")?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 2. query latest gas price and add 50% buffer for network fluctuations
    let base_gas_price = provider.get_gas_price().await.wrap_err("Query latest price failed.")?;
    let gas_price_with_buffer = base_gas_price + (base_gas_price / 2); // +50% buffer

    // 3. estimate gas limit for the transaction
    let tx = TransactionRequest::default()
        .to(to_address)
        .value(amount_wei);
    let gas_estimate = provider.estimate_gas(tx).await
        .wrap_err("Failed to estimate gas")?;
    
    // Add 20% buffer to gas estimate
    let gas_limit_with_buffer = gas_estimate + (gas_estimate / 5);

    Ok(GasInfo {
        gas_price: gas_price_with_buffer,
        estimated_gas_limit: gas_limit_with_buffer,
    })
}

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