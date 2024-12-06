```
docker compose up -d
bash run.sh
yarn
yarn deployXRPToken # Create XRP Soroban Token and save address in .soroban folder
yarn mintAndCreateLP # Mint XRP Tokens and create LP with native Testnet XLM in Soroswap
```

Now you can Swap Tokens on Soroswap in 
http://localhost:3000/swap/CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC/{YOUR DEPLOYED XRP ADDRESS}

Make sure select "Testnet" on your Freighter Wallet

