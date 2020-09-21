# CosmWasm IPFS Contract
This is a contract that allows users to upload an IPFS address (String) and list all IPFS addresses stored in the contract state. 

This contract is not meant to scale or be used in any production environment. 

## Using the contract

To run the tests:
```
cargo test
```

To compile:
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.9.0
```

Run the following command to access the cosmJS helper repl:
```
npx @cosmjs/cli --init https://raw.githubusercontent.com/CosmWasm/testnets/master/coralnet/cli_helper.ts
```

Deployment: 
```
const fredSeed = loadOrCreateMnemonic("fred.key");
const {address: fredAddr, client: fredClient} = await connect(fredSeed, {});
hitFaucet(defaultFaucetUrl, fredAddr, "SHELL")

const wasm = fs.readFileSync('contract.wasm');
const up = await fredClient.upload(wasm, {builder: "cosmwasm/rust-optimizer:0.9.0"});
const initMsg = {ipfs_address = 'aaaa-bbbbb-ccccc-eeeeee-ipfs-address'};
const { codeId } = up;
const { contractAddress } = await fredClient.instantiate(codeId, initMsg, "Test", { memo: "memo"});
```

Execute:
```
const addFileMsg = {add_file_address: {ipfs_address: "aaaa-bbbbb-ccccc-ddddd-ipfs-address"}};
fredClient.execute(contractAddress, addFileMsg);
```

Query:
```
fredClient.queryContractSmart(contractAddress, {list_all: {}});
```
