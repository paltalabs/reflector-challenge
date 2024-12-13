Set up the environment variables

```
cp .env.example .env
```

Fill the .env file with the admin private key and the aggregator address.

Run in this order

```
docker compose up -d
bash run.sh
yarn
yarn deployXRPToken # Create XRP Soroban Token and save address in .soroban folder
yarn mintAndCreateLP # Mint XRP Tokens and create LP with native Testnet XLM in Soroswap
yarn deployHodl # Deploy Hodl strategies for XRP & XLM contracts and save address in .soroban folder
yarn deployFactory # Deploy the defindex factory
yarn deployVault # Deploys a vault with 2 assets and one strategy each
```

Now you can Swap Tokens on Soroswap in
https://app.soroswap.finance/swap/CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC/{YOUR DEPLOYED XRP ADDRESS}

Make sure select "Testnet" on your Freighter Wallet

In contracts folder we have:
- reflector.wasm, compiled from https://github.com/reflector-network/reflector-contract

