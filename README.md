This is homeworks of Arbitrum x HackQuest Rust colearning.

## MetaMask configuration

1. Add MetaMask extention to the brower

<img src="./img/1-metamask.png" width=50%>

2. Switch to Aribitrum Sepolia Testnet

<img src="./img/2-switch-to-arb-sepolia.png" width=50%>

3. Get test coin from faucet

<img src="./img/3-faucet.png" width=50%>

4. Ready for the journey!

<img src="./img/4-got-test-coin.png" width=50%>

## Task1: Hello Web3

In `src/task1_hello_web3/hello.rs`, we connect to Aribitrum Sepolia Testnet, get the latest block number, and invoke the smart contract HelloWeb3. By running `cargo run`, we have the output:

<img src="./img/task1-output.png">

## Task2: Balance Query

In `src/task2_balance_query/balance.rs`, we connect to Aribitrum Sepolia Testnet, query the balance of the address, convert it from Wei to ETH and finally return. The running result is:

<img src="./img/task2-output.png">

## Task3: Gas Price

In `src/task3_gas_price/price.rs`, we get the latest gas price, estimate the fee of a transfer, and present the result in Gwei and ETH.

<img src="./img/task3-output.png">

## Task4: Transfer

In `src/task4_transfer/transfer.rs`, we transfer `amount_eth` ETH to address `to_address_str`.

<img src="./img/task4-output.png">

Note that we manually set set price and gas limit. To make sure the transaction will be accepted, we add a +50% buffer on gas price and +20% buffer on gas limit. In practice, the automatic configuration is already enough.
