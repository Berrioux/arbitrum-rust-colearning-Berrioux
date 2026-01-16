use alloy::{
    primitives::Address,
    providers::{ProviderBuilder},
    sol,
};
use std::env;
use eyre::{Context, Result};

// load WETH ABI
sol!(
    #[sol(rpc)]
    WETHInstance,
    "./src/task5_contract/weth_abi.json" 
);

pub async fn interact_with_weth() -> Result<()> {
    // connect to Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // (proxy) contract address of WETH (Arbitrum Sepolia)
    let contract_addr: Address = "0x980b62da83eff3d4576c647993b0c1d7faf17c73".parse()?;

    // create contract instance
    let contract = WETHInstance::new(contract_addr, provider);

    // Now we can invoke all the methods defined in the WETH ABI
    println!("========== basic info ==========");
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;
    let total_supply = contract.totalSupply().call().await?;
    
    println!("Token Name: {}", name);
    println!("Token Symbol: {}", symbol);
    println!("Decimals: {}", decimals);
    println!("Total Supply: {}", total_supply);

    // Query L2 gateway and L1 address (these are specific to the implementation contract)
    println!("========== L2 specific info ==========");
    let l2_gateway = contract.l2Gateway().call().await?;
    let l1_address = contract.l1Address().call().await?;
    
    println!("L2 Gateway: {:?}", l2_gateway);
    println!("L1 Address: {:?}", l1_address);

    // Query the balance of a specific address
    dotenvy::dotenv().ok();
    let my_addr_str = env::var("ADDRESS").wrap_err("ADDRESS not set in .env file")?;
    let my_addr: Address = my_addr_str.parse()?;
    let balance = contract.balanceOf(my_addr).call().await?;
    println!("========== balance query ==========");
    println!("Address {} balance: {}", my_addr, balance);

    Ok(())
}