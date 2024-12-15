# Trustless Portfolio Manager using Reflector by PaltaLabs

Smart Contract that helps DeFindex users maintain a 50/50 balanced investment in a safe and trustless way using a decentralized Reflector Network Oracle.  
- **Video:**  https://youtu.be/Mv0JJCSSlx0
- **Repo:** [https://github.com/paltalabs/reflector-challenge](https://github.com/paltalabs/reflector-challenge)  
- **Trustees Manager Contract:** [CCXBRLREKJOBXH4TTLIRY6QTZI6LLUPZZWSSOINVJ66C3A755LDLKTIS](https://stellar.expert/explorer/testnet/contract/CCXBRLREKJOBXH4TTLIRY6QTZI6LLUPZZWSSOINVJ66C3A755LDLKTIS)  
- **Invocation:** [365355688017920](https://stellar.expert/explorer/testnet/tx/365355688017920)  
- **Admin Account:** [GCXGJSBPVPLBCL5PJTYZCG2QPFYUBXURGJSGNOYAWQ3TB4FMTZ6GUWVX](https://stellar.expert/explorer/testnet/account/GCXGJSBPVPLBCL5PJTYZCG2QPFYUBXURGJSGNOYAWQ3TB4FMTZ6GUWVX)  
- **User Account:** [GB6DCKSDJJ3H4QO6E4OHGOMFAIEHAQTANXFVQ2R4AVGTQE4RF3U27I73](https://stellar.expert/explorer/testnet/account/GB6DCKSDJJ3H4QO6E4OHGOMFAIEHAQTANXFVQ2R4AVGTQE4RF3U27I73)  
- **Vault:** [CBZZWNAH4OA53LMJKGPZCP5OZ27LAXVFXS6WWXKXHCLTW2ASZBOVXECR](https://stellar.expert/explorer/testnet/contract/CBZZWNAH4OA53LMJKGPZCP5OZ27LAXVFXS6WWXKXHCLTW2ASZBOVXECR)  
- **XRP Hodl Strategy:** [CBZLZX5XIJ7YB7XXACSUSBX5OUUHANVCUH5UWDA5EB2M3TAORFYU4O6B](https://stellar.expert/explorer/testnet/contract/CBZLZX5XIJ7YB7XXACSUSBX5OUUHANVCUH5UWDA5EB2M3TAORFYU4O6B)  
- **XLM Hodl Strategy:** [CDNWVYVXMU36D4X7SP5FC6BZICAGOVIYRWDTVRQNXU3F2HA4ZUQDTCIK](https://stellar.expert/explorer/testnet/contract/CDNWVYVXMU36D4X7SP5FC6BZICAGOVIYRWDTVRQNXU3F2HA4ZUQDTCIK)


Made with ♥️ by PaltaLabs 🥑 

## Problem

Many users seek to maintain a balanced portfolio. For instance, a user might aim to hold 50% of their portfolio's value in XLM and 50% in XRP. Achieving this balance requires frequent rebalancing. For example, if the price of XLM rises significantly, the user would need to sell some XLM and buy XRP to restore the 50:50 ratio in terms of USD. This manual process is not only time-consuming but also complex, especially if the user wants to invest these assets in a lending protocol or similar platforms. Additionally, frequent transactions increase risks, as users must keep their wallets connected to the internet, which exposes them to potential security threats.

## Solution

We propose a **trustless portfolio manager** that combines the strengths of **Reflector**, **DeFindex**, and **Soroswap** to automate portfolio rebalancing.

- Reflector ensures accurate pricing data and protects against vulnerabilities in on-chain price feeds, such as flash loan attack risks.
- DeFindex enables multiple users to maintain a managed portfolio within the same configured Vault.
- Soroswap facilitates asset exchanges using an Automated Market Maker model.

The Trustless Portfolio Manager is implemented as a Smart Contract on Soroban, designed to rebalance a DeFindex Vault based on predefined ratios. This manager's rebalance function can be triggered by a bot or any interested party in a trustless manner; one only needs a wallet with just enough XLM to cover gas fees.

The Defindex Vault: We use a DeFindex vault configured wih two assets: XLM and XRP, each of the asset is configured with a very symple "HODL" Strategy, that just holds the asset on belhalf of the user, however this Strategy can be something more complex like a leveraged position or automated yield farming.

When triggered, the function:
1. Verifies the current asset prices on Reflector.  
2. Constructs a series of instructions to execute the rebalancing process.  
3. Executes trades to maintain the desired asset allocation, ensuring a balanced portfolio.

By automating these processes, the trustless manager rebalances the vault of several users, eliminates the need for users to manually rebalance their portfolios, reduces risks associated with constant wallet connectivity, and simplifies complex interactions with decentralized finance (DeFi) protocols.

## References

- [Reflector.Network Documentation](https://reflector.network/docs)  
- [DeFindex Whitepaper](https://docs.defindex.io/whitepaper/10-whitepaper/01-introduction)  
- [Soroswap Documentation](https://docs.soroswap.finance/)  



# Development

## Setting up your local instance:
If you want to try the trustless rebalancer for yourself, you need to:

1. **Clone the repo:**
```bash
git clone https://github.com/paltalabs/reflector-challenge`
cd reflector-challenge
```
2. **Run the Docker Container:**

To try everything with exactly the same versions as the dev team, and to avoid installing specific software like Soroban CLI, use our Docker image. (If it’s your first time using a Soroban Docker image, this step will take a while.)

    docker compose up -d && bash run.sh

3. Set up the env:

```bash
cp .env.example .env
```

Fill it with the admin secret key. This will be the account that will deploy all the contracts and perform all the invocations. The scripts will handle the Friendbot!

4. **Install dependencies**:
```bash
    yarn install
```
5. **Build and deploy the contracts**:

To deploy your smart contracts, navigate to the contracts folder and build the files by running:

     cd contracts && make build

Once this process has finished, you can deploy each contract in the following exact order:

 ``` bash
yarn deployXRPToken # Create XRP Soroban Token and save the address in the .soroban folder
yarn mintAndCreateLP # Mint XRP Tokens and create LP with native Testnet XLM in Soroswap
yarn deployHodl # Deploy HODL strategies for XRP & XLM contracts and save the address in the .soroban folder
yarn deployFactory # Deploy the defindex factory
yarn deployVault # Deploy a vault with 2 assets and one strategy each
yarn deployTManager # Deploy the Trustless Portfolio Manager
 ```

Or deploy them all at once by running:

    yarn deployTestnet

## Testing the Trustless Portafolio Rebalancer:
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


# Future work and improvements:
- Use Soroswap Aggregator: Currently the Trustless Manager only knows about a specific LP pair in Soroswap. We could let the Trustless Manager to create more complex trades using Soroswap Aggregator in order to get even better prices.
- Invest all the funds given as return by Soroswap.
- Proection of Liquidity Pools manipulations: 1) Force that the caller of the rebalance function is an account and no a smart contract (flash loan attack), 2) Check that the LP has the same or similar price as the one given by the oracle.\
- Implement more complex strategies. In this example we just used simple "HODL" strategy, but we could use more complex strategies like leveraged positions, or auocompound yield farming.