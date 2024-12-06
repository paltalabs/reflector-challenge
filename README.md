Set up the environment variables

```
cp .env.example .env
```

```
docker compose up -d
bash run.sh
yarn
yarn deployXRPToken # Create XRP Soroban Token and save address in .soroban folder
yarn mintAndCreateLP # Mint XRP Tokens and create LP with native Testnet XLM in Soroswap

```