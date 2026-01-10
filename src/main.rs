use alloy::providers::{Provider, ProviderBuilder};
use eyre::{Context, Result};
use alloy::sol;
use alloy::primitives::Address;

sol! {
    #[sol(rpc)]
    contract HelloWeb3 {
        function hello_web3() pure public returns(string memory);
    }
}

#[tokio::main]  // asynchronous main function
async fn main() -> Result<()> {
    // 1. public rpc address of Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse().wrap_err("parse RPC url failed.")?;

    // 2. create a provider
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 3. interaction: get the latest block number
    let latest_block_number = provider.get_block_number().await.wrap_err("failed to connect to Arbitrum Sepolia.")?;

    println!("Hello Web3!");
    println!("Connected to Arbitrum Sepolia Testnet!");
    println!("The latest block number is: {}", latest_block_number);

    // 4. invoke the contract
    let contract_addr: Address = "0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90".parse().wrap_err("failed to parse contract address.")?;
    let contract = HelloWeb3::new(contract_addr, provider);
    let result = contract.hello_web3().call().await.wrap_err("failed to invoke contract.")?;

    println!("Invoked contract HelloWeb3, the result is: {}", result);

    Ok(())
}