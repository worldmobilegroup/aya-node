# scripts/derive_multi.sh

## Description

- Script to set up all accounts for a testnet

## Previous steps

- Go to `tools/keys` and run `npm i` to install all dependencies (this tool will get you the evm compressed 
key).
- Copy `config.json.example` to `config.json` (and modify if you want, the default values are the ones for devnet).

## Usage

```shell
./scripts/derive_multi.sh "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
```

The seed defaults to "bottom drive obey lake curtain smoke basket hold race lonely fit walk" (local testnet seed to 
get Alice and co.) if you do not provide one.

## Explanation

You will get the keys for every config you need to set up: 

- sr25519 public and private keys
- ed25519 public and private keys
- compressed EVM key

## Example output (default seed for local testnet and only Alice on the config file)

```shell
./scripts//derive_multi.sh 
Processing account: 0 - Alice

sr25519

Secret Key URI `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice` is account:
  Network ID:        substrate
  Secret seed:       0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
  Public key (hex):  0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
  Account ID:        0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
  Public key (SS58): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  SS58 Address:      5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

ed25519

Secret Key URI `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice` is account:
  Network ID:        substrate
  Secret seed:       0xabf8e5bdbe30c65656c0a3cbd181ff8a56294a69dfedd27982aace4a76909115
  Public key (hex):  0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee
  Account ID:        0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee
  Public key (SS58): 5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu
  SS58 Address:      5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu

Compressed EVM key:             0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac
Private key:                    0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133

------------
```

## Setup the initial nodes

Clarification:
- AURA_KEY is the sr25519 Secret seed.
- GRANDPA_KEY is the ed25519 Secret seed.
- IM_ONLINE_KEY is the sr25519 Secret seed.

### Frigga

```shell
export AURA_KEY=XXX
export GRANDPA_KEY=XXX
export IM_ONLINE_KEY=XXX

./target/release/aya-node key insert \
    --base-path data/frigga \
    --chain devnet \
    --key-type aura \
    --scheme sr25519 \
    --suri "${AURA_KEY}";
    
./target/release/aya-node key insert \
    --base-path data/frigga \
    --chain devnet \
    --key-type gran \
    --scheme ed25519 \
    --suri "${GRANDPA_KEY}";
    
./target/release/aya-node key insert \
    --base-path data/frigga \
    --chain devnet \
    --key-type imon \
    --scheme sr25519 \
    --suri "${IM_ONLINE_KEY}";
    
    
ls -l data/frigga/chains/aya_devnet/keystore/;

RUST_LOG=runtime=debug
    
./target/release/aya-node \
    --base-path data/frigga \
    --name Frigga \
    --validator \
    --chain devnet \
    --port 30333 \
    --rpc-port 9944 \
    --log info \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001
```

### Ullr

| :exclamation:  Change the `--bootnodes` flag to the correct path, this will only work it you set both nodes on the same server |
|--------------------------------------------------------------------------------------------------------------------------------|

```shell
export AURA_KEY=XXX
export GRANDPA_KEY=XXX
export IM_ONLINE_KEY=XXX

./target/release/aya-node key insert \
    --base-path data/ullr \
    --chain devnet \
    --key-type aura \
    --scheme sr25519 \
    --suri "${GRANDPA_KEY}";
    
./target/release/aya-node key insert \
    --base-path data/ullr \
    --chain devnet \
    --key-type gran \
    --scheme ed25519 \
    --suri "${AURA_KEY}";
    
./target/release/aya-node key insert \
    --base-path data/ullr \
    --chain devnet \
    --key-type imon \
    --scheme sr25519 \
    --suri "${IM_ONLINE_KEY}";
    
    
ls -l data/ullr/chains/aya_devnet/keystore/;

RUST_LOG=runtime=debug 
    
./target/release/aya-node \
    --base-path data/ullr \
    --chain devnet \
    --name Ullr \
    --validator \
    --port 30334 \
    --rpc-port 9945 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```