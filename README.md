Set up the environment variables

```
cp .env.example .env
```

```
docker compose up -d
bash run.sh
yarn
yarn deployXRPToken # Create XRP Soroban Token and save address in .soroban folder
yarn deployHodl # Deploy Hodl strategies for XRP & XLM contracts and save address in .soroban folder
yarn mintAndCreateLP # Mint XRP Tokens and create LP with native Testnet XLM in Soroswap
```

Now you can Swap Tokens on Soroswap in 
https://app.soroswap.finance/swap/CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC/{YOUR DEPLOYED XRP ADDRESS}

Make sure select "Testnet" on your Freighter Wallet

