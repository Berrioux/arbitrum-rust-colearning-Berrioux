use eyre::{Context, Result};
use alloy::primitives::Address;
use std::env;

mod task1_hello_web3;
mod task2_balance_query;
mod task3_gas_price;
mod task4_transfer;

#[tokio::main]  // asynchronous main function
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // task1: hello web3
    println!("-------- Task 1 --------");
    task1_hello_web3::hello::hello_web3().await?;

    // task2: balance query
    println!("-------- Task 2 --------");
    let my_addr_str = env::var("ADDRESS").wrap_err("ADDRESS not set in .env file")?;
    let my_addr: Address = my_addr_str.parse()?;
    match task2_balance_query::balance::get_eth_balance(my_addr).await {
        Ok(balance) => println!("The balance of address {} is: {} ETH", my_addr, balance),
        Err(e) => eprintln!("Error: {}", e),
    }

    // task3: gas price
    println!("-------- Task 3 --------");
    task3_gas_price::price::gas_price().await?;

    // task4: transfer eth
    println!("-------- Task 4 --------");
    let to_addr_str = env::var("TARGET_ADDRESS").wrap_err("TARGET_ADDRESS not set in .env file")?;
    let amount = "0.001"; // in ETH
    task4_transfer::transfer::transfer(&to_addr_str, amount).await?;

    println!("-------- End --------");
    Ok(())
}