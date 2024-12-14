import { Address, Asset, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { toolkitLoader } from "./toolkit";
import { invokeContract, airdropAccount, invokeCustomContract } from "soroban-toolkit";
import fs from "fs";

const args = process.argv.slice(2);
if (args.length !== 1) {
  console.error("Usage: yarn start <network>");
  process.exit(1);
}


const network = args[0];
const toolkit = toolkitLoader.getNetworkToolkit(network);
const xrp_token = new Address(toolkit.addressBook.getContractId("XRP"));

async function main() {
  const vaultAddress = toolkit.addressBook.getContractId("vault");
  console.log("-------------------------------------------------------");
  console.log("Vault address:", vaultAddress);
  console.log("-------------------------------------------------------");

  //Mintear XRP y XLM para el usuario 100_0_000_000
  const newUser = Keypair.random()
  const mintAmount = nativeToScVal(100_0_000_000, { type: "i128" });
  const mintParams: xdr.ScVal[] = [new Address(newUser.publicKey()).toScVal(), mintAmount];

  await invokeContract(toolkit, "XRP", "mint", mintParams);
  console.log("Minted XRP to user:", newUser.publicKey().toString());
  await airdropAccount(toolkit, newUser);


  //Deposit XRP y XLM al vault

  const depositAmount = nativeToScVal(100_000_000, { type: "i128" });
  let amountsDesired = [depositAmount, depositAmount];
  let amountsMin = [depositAmount, depositAmount];
  const depositParams: xdr.ScVal[] = [
    xdr.ScVal.scvVec(amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))),
    xdr.ScVal.scvVec(amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))),
    new Address(newUser.publicKey()).toScVal(),
    xdr.ScVal.scvBool(false)
  ];

  await invokeContract(toolkit, "vault", "deposit", depositParams, false, newUser);

  const investmentArgs: any[] = [
    {
      asset: xrp_token,
      strategy_investments: [
        {
          strategy: new Address(toolkit.addressBook.getContractId("hodl_xrp")),
          amount: 100_000_000,
        },
      ],
    },
    {
      asset: new Address(Asset.native().contractId(toolkit.passphrase)),
      strategy_investments: [
        {
          strategy: new Address(toolkit.addressBook.getContractId("hodl_xlm")),
          amount: 100_000_000,
        },
      ],
    },
  ];

  const mappedParam = xdr.ScVal.scvVec(
    investmentArgs.map((entry) =>
      xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("asset"),
          val: entry.asset.toScVal(), // Convert asset address to ScVal
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("strategy_allocations"),
          val: xdr.ScVal.scvVec(
            entry.strategy_investments.map((investment: any) =>
              xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                  key: xdr.ScVal.scvSymbol("amount"),
                  val: nativeToScVal(BigInt(investment.amount), { type: "i128" }), // Ensure i128 conversion
                }),
                new xdr.ScMapEntry({
                  key: xdr.ScVal.scvSymbol("strategy_address"),
                  val: investment.strategy.toScVal(), // Convert strategy address
                }),
              ])
            )
          ),
        }),
      ])
    )
  );

  await invokeContract(toolkit, "vault", "invest", [mappedParam], false, toolkit.admin);

  const invested_funds_after_investment = await invokeContract(toolkit, "vault", "fetch_current_invested_funds", [], true);
  const parsed_invested_funds_after_investment = scValToNative(invested_funds_after_investment.result.retval);
  console.log('parsed result', parsed_invested_funds_after_investment)

  const valuesAfterInvest = Object.values(parsed_invested_funds_after_investment);

  //Set tmanager as vault manager
  const tmanagerAddress = toolkit.addressBook.getContractId("tmanager");
  const setAdminParams: xdr.ScVal[] = [new Address(tmanagerAddress).toScVal()];
  const set_manager = await invokeContract(toolkit, "vault", "set_manager", setAdminParams);
  console.log("Set tmanager as admin of vault", set_manager);

  // get_pair address
  const addressesJson = fs
    .readFileSync(`./public/${network}.contracts.json`)
    .toString();
  const addresses = JSON.parse(addressesJson);
  const getPairResult = await invokeCustomContract(toolkit, addresses.soroswap_factory, "get_pair",
    [new Address(addresses.xlm).toScVal(),
    new Address(toolkit.addressBook.getContractId("XRP")).toScVal()
    ],
    true
  )
  const pairString = scValToNative(getPairResult.result.retval)
  console.log("pairAdress", pairString)
  //Rebalance
  await invokeContract(toolkit, "tmanager", "rebalance", [
    (new Address(addresses.soroswap_router).toScVal()),
    new Address(pairString).toScVal()
  ], false, newUser);

  const invested_funds_after_rebalance = await invokeContract(toolkit, "vault", "fetch_current_invested_funds", [], true);
  const parsed_invested_funds_after_rebalance = scValToNative(invested_funds_after_rebalance.result.retval);
  const valuesAfterRebalance = Object.values(parsed_invested_funds_after_rebalance);

  console.table({
    XRP: {
      "Before rebalance": valuesAfterInvest[0],
      "After rebalance": valuesAfterRebalance[0]
    },
    XLM: {
      "Before rebalance": valuesAfterInvest[1],
      "After rebalance": valuesAfterRebalance[1]
    }

  })
}

main();
