use eyre::{Result};
use alloy::primitives::Address;

mod task1_hello_web3;
mod task2_balance_query;
mod task3_gas_price;

#[tokio::main]  // asynchronous main function
async fn main() -> Result<()> {
    // task1: hello web3
    println!("-------- Task 1 --------");
    task1_hello_web3::hello::hello_web3().await?;

    // task2: balance query
    println!("-------- Task 2 --------");
    let my_addr: Address = "0x724F1E6C5cdc958eB6D305090c6cAfa79F11A39b".parse()?;
    match task2_balance_query::balance::get_eth_balance(my_addr).await {
        Ok(balance) => println!("The balance of address {} is: {} ETH", my_addr, balance),
        Err(e) => eprintln!("Error: {}", e),
    }

    // task3: gas price
    println!("-------- Task 3 --------");
    task3_gas_price::price::gas_price().await?;

    println!("-------- End --------");
    Ok(())
}