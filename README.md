# Trustless rebalancer:

## Setting up your local instance:
If you want to try trustless rebalancer for yourself you need to:

### Clone the repo:
In a new terminal run: `git clone https://github.com/paltalabs/reflector-challenge`
and navigate to the new folder. 

    cd reflector-challenge
### Run the container:
(If it´s your first time using a soroban docker image this step will take a while.)

    docker compose up -d && bash run.sh

### Set up the env:
Copy the .env.example and create a new .env file in the root folder of the project:

    cp .env.example .env

And fill the admin secret key using a soroban secret key format string, and the aggregator addresses using a soroban public address fromat string.

### Install dependencies:
Install the project dependencies using yarn:

    yarn install

### Build and deploy the contracts:
In order to deploy your smart contracts you should navigate to the contracts folder and build the files running:

     cd contracts && make build

Once this process has finished you can deploy each contract running in this exact order: 

 ``` bash
yarn deployXRPToken # Create XRP Soroban Token and save address in .soroban folder
yarn mintAndCreateLP # Mint XRP Tokens and create LP with native Testnet XLM in Soroswap
yarn deployHodl # Deploy Hodl strategies for XRP & XLM contracts and save address in .soroban folder
yarn deployFactory # Deploy the defindex factory
yarn deployVault # Deploys a vault with 2 assets and one strategy each
 ```

Or deploy them all at once by running:

    yarn deployTestnet

## Testing the trustless rebalancer:
Once you have deployed all the contracts you can run
``` bash
    #usage: yarn start <network>
    yarn start testnet
```
This script will:
	- Mint XLM and XRP to a test account.
	- Deposit XLM and XRP into a defindex vault.
	- Invest XLM and XRP in their own HODL strategies.
	- Set the Trustless Manager contract as the vault manager.
	- Call the rebalance method in the vault.
	- Display a table with the results.