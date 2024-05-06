# Join the World Mobile Dev-Net

Welcome, lets start!

## Minimal Server Requirnments

OS: Ubuntu 22.04

CPU: 2x vCPU

Memory: 8 GB

Storage: ~250 GB

## Setting Up OS

Login to your server and access the terminal, for example using SSH connection. 

Install dependencies: 

```bash
sudo apt update
sudo apt install -y git clang curl libssl-dev llvm libudev-dev make protobuf-compiler pkg-config
```

Install Rust: 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Proceed with option 1 "Standard Installation" in the prompt, just hit enter. 

Get Rust enviornment into terminal session:
```bash
source $HOME/.cargo/env
```

Check Rust is installed correctly
```bash
rustc --version
```

Configure Rust toolchain: 
```bash
rustup default stable
rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
rustup target add wasm32-unknown-unknown
rustup update
```

Configure Rust Nighly (Nighly is needed to build subkey, it can be skipped if you don't want to compile subkey)
```bash
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Check Rust Installation:
```bash
rustup show
rustup +nightly show
```

## Build AyA-Node from Source Code

We recommend to compile the aya-node not on a small virtual machine as this can take quite some time. Instead build the aya-node on your local machine and copy the binary to the server. 

Clone the AyA-Node Repository:
```bash
git clone https://github.com/worldmobilegroup/aya-node.git
```

Compile the AyA-Node:
```bash
cd aya-node
git checkout -b "my-wip-branch"
cargo build --release
```

## Prepare Key Setup
We recommend to use an air gapped machine to generate your keys. You can use subkey or the aya-node binary to generate keys. 

### Using subkey
Please follow the official Documentation to install subkey: [Subkey Docs](https://docs.substrate.io/reference/command-line-tools/subkey/)

Copy the compiled subkey binary to `/usr/bin`:
```bash
sudo cp ./target/release/subkey /usr/bin/
```

Check subkey is installed:
```bash
subkey --version
```

You can delete the polkadot-sdk repository when subkey works as expected. It is very big and not needed anymore. 

Creating a new key: 
```bash
subkey generate
```

Example Output: 
```
Secret phrase:       bottom drive obey lake curtain smoke basket hold race lonely fit walk
  Network ID:        substrate
  Secret seed:       0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
  Public key (hex):  0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
  Account ID:        0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
  Public key (SS58): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  SS58 Address:      5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

The command `subkey inspect` will produce the same reuslt but the mnemonic is given.
```bash 
subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
```


You can derive accounts like this:
```bash 
subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"
```

### Using Aya-Node Binary
Keys can also be generated and inspected with the aya-node binary

Generate:
```bash
./target/release/aya-node key generate
```

Inspect:
```bash
./target/release/aya-node key inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
```

## Get EVM Account and Keys

Subkey/Aya-Node unforutnatley do not give us all information, to generate also the EVM accounts use the validator_keys.sh script in the utils folder of the aya-node repository. See the Readme to learn more. (This step is optional if you did not derived an account)

Execute the script: 
```bash
./scripts/validator_keys.sh "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
``` 

Example Output: 
```
Processing account:

sr25519

Secret phrase:       bottom drive obey lake curtain smoke basket hold race lonely fit walk
  Network ID:        substrate
  Secret seed:       0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
  Public key (hex):  0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a
  Account ID:        0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a
  Public key (SS58): 5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV
  SS58 Address:      5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV

ed25519

Secret phrase:       bottom drive obey lake curtain smoke basket hold race lonely fit walk
  Network ID:        substrate
  Secret seed:       0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
  Public key (hex):  0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef
  Account ID:        0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef
  Public key (SS58): 5DFJF7tY4bpbpcKPJcBTQaKuCDEPCpiz8TRjpmLeTtweqmXL
  SS58 Address:      5DFJF7tY4bpbpcKPJcBTQaKuCDEPCpiz8TRjpmLeTtweqmXL

Compressed EVM key: 	0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac
Private key: 			0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133

------------
```

If you used a derived Address the sr25519 and the ed25519 private keys will be different.
When you use a derived address, make sure you are able to restore the address in a wallet like MetaMask or Talisman to sign transactions. 


## Setup Validator

### Adding Keys to the Validator

To setup the validator we need to add the secret keys to the keyring on the validator machine. On the machine running the validator, navigate to the `aya-node` repository folder.

We need to add three keys: the Aura, Grandpa and Im-Online key. Depending on the derivation path in the mnemonic the, secret seeds of the ed25519 and sr25519 scheme might differ! The AURA_KEY and IM_ONLINE_KEY is the sr25519 secret seed, the GRANDPA_KEY is the ed25519 secret seed. In this example the secret seeds are the same as we did not derived an account. 
You also need to set the base path of the node, the path were all data for the node is stored. We use "data/validator" make sure the path is accessible. 

Optional - Set keys to environment variables or enter directly at the `--suri` parameter:
```bash
export AURA_KEY=0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
export GRANDPA_KEY=0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
export IM_ONLINE_KEY=0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
```

```bash
./target/release/aya-node key insert \
    --base-path data/validator \
    --chain wm-devnet-chainspec.json \
    --key-type aura \
    --scheme sr25519 \
    --suri "${AURA_KEY}";

./target/release/aya-node key insert \
    --base-path data/validator \
    --chain wm-devnet-chainspec.json \
    --key-type gran \
    --scheme ed25519 \
    --suri "${GRANDPA_KEY}";
    
./target/release/aya-node key insert \
    --base-path data/validator \
    --chain wm-devnet-chainspec.json \
    --key-type imon \
    --scheme sr25519 \
    --suri "${IM_ONLINE_KEY}";
```

Check if keys were added:
```bash
ls -l data/validator/chains/aya_devnet/keystore/;
```

The command should output three files.


### Setting up systemd
We want that our validator starts automatically with the server and is restarted automatically. For that purpose we create a systemd service (Ubuntu 22.04).

First we create a startup script for the AyA-Node.

Make sure the path to the aya-node binary is correct in the command below. It is expected you cloned and compiled the repository on the validator machine in your users home folder. In this case the aya-node would be located in "/home/myuser/aya-node/target/release".

Set AyA Home Path to the folder were your aya-node binary is located: 

```bash
export AYA_HOME=/home/<MY_USER>/aya-node
```

Set `AYA_HOME` on server start: 
```bash
sudo bash -c "echo 'export AYA_HOME=/home/<MY_USER>/aya-node' >> /etc/bash.bashrc"
```

Create Startup Script: 

```bash
echo '#!/usr/bin/env bash' > start_aya_validator.sh
echo "${AYA_HOME}/target/release/aya-node \
    --base-path ${AYA_HOME}/data/validator \
    --validator \
    --chain ${AYA_HOME}/wm-devnet-chainspec.json \
    --port 30333 \
    --rpc-port 9944 \
    --log info \
    --bootnodes /ip4/35.189.75.1/tcp/30340/ws/p2p/12D3KooWEb8sRMz6JLzsJedPqvoHV11U2P72htRAiva3LN9GR5V6" >> start_aya_validator.sh
sudo chmod +x ./start_aya_validator.sh
```

Let us test that the validator starts by executing the script we just created
```bash
./start_aya_validator.sh
```

The validator should start syncing, if there is a problem check the path to the `aya-node` binary and the `wm-devnet-chainspec.json`.

Stop the aya-node again by hitting Ctrl+C.

### Creating systemd service
Next we create the systemd service:

```bash
sudo nano /etc/systemd/system/aya-node.service
```

Copy the following content to the file and make sure to adjust the PATH to the file `start_aya_validator.sh` we have created in the last step.
```
[Unit]
Description=AyA Node
After=network.target

[Service]
ExecStart=/PATH/TO/start_aya_validator.sh
User=root
Restart=always
RestartSec=90

[Install]
WantedBy=multi-user.target
```

Next we enable the service:
```bash
sudo systemctl enable aya-node.service
```

Start the node via systemd:
```bash
sudo systemctl start aya-node.service
```

Check the service is running: 
```bash
sudo systemctl status aya-node.service
```

If there is a problem check that all paths are fine first, that is the most common problem. 

You can look at the logs with:

```bash
sudo journalctl -u aya-node.service
```

If you want to follow the logs use: 

```bash
sudo journalctl -f -u aya-node.service
```

In case your systemd service is not working properly you can find debugging information in this log. 

If everything worked out you should have a running full node / validator which is syncing with the blockchain (but not validating blocks).
You can see in the logs that the node is importing blocks. 

If you just want to setup a full node you can stop here. 

### Session Keys

The aya-node is able to operate now, but the blockchain does not know of the validator yet. To make it possible for a validator to join the network as block producer it must commit its session keys and get added to the validator set by the ruling chain authority. The session keys are important, the rotation must be restricted to the operator only. Validators should for this purpose make sure no API of the validator is exposed to the public.

To get your session keys you need to start the node and make a local RPC call. If you followed the previous steps your node should be already running as systemd service. 

Check the status of your node service:

```bash
sudo systemctl status aya-node.service
```

Check if the node is importing blocks: 

```bash
sudo journalctl -f -u aya-node.service
```

If your node is running correctly you can trigger a key rotation via the local RPC interface using curl. Make sure you have added the AURA, GRANDPA and IMONLINE keys to your validator. Obtain Session Keys from Node API: 

```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' http://localhost:9944/
```

Example Output: 
```
0x04423b6990fa0a0c87b362d5d43411faefb8a54f0dfe5db10a90dd357d27d459aa1c76e6239a4bda1c3b07ea5785cad3e42e48dd8d2b89403dfa894aff6060e208adcf5c46ac4aab449a35ed71be861bd1242fedab04073f7d686fc138b59252
```

The output contains the AURA, GRANDPA and IMONLINE public keys in one large string, we need to splitt the string up. Each key has 32 bytes, the string is hex encoded so each key has 64 characters. Each of the keys needs a prefix '0x' to indecate it is hex encoded. The output starts already with '0x' so the first key starts behind the '0x' and is 64 characters long. The second key starts at the 65 character, it has no '0x' prependet yet so we do that. The next key starts a character 130 and also needs a '0x' prefix. All keys should have the same length (32byte or 64 characters) and have a 0x prepended. 

Tip: Copy one of the exampel keys below to a text file. Copy the Output string in the line below. Now you see where you need to split the string. Hit enter, prefix it with `0x`, repeat.
 

Example: 
```
0x04423b6990fa0a0c87b362d5d43411faefb8a54f0dfe5db10a90dd357d27d459 = Aura Key
0xaa1c76e6239a4bda1c3b07ea5785cad3e42e48dd8d2b89403dfa894aff6060e2 = Grandpa Key
0x08adcf5c46ac4aab449a35ed71be861bd1242fedab04073f7d686fc138b59252 = ImOnline Key
```

Put this key aside you will need them in the next step. 


## Connect to Polkadot-JS Development Front-End

### Connect to Public Remote Note
You can connect to a Polkadot-JS Front-End using our public remote node, be aware that the node uses a self-signed SSL certificate which you need to accept in order to use it. 
Accept the risk on this endpoint: [Link](https://35.189.75.1/)
After accepting you should see `Used HTTP Method is not allowed. POST or OPTIONS is required` on a blank page.

Now you should be able to connect to the RPC endpoint of the public node: 
[Front-End](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2F35.189.75.1%3A#/explorer)

### Setup Wallet and Restore Validator Account in Talisman
In the next step we want to add our validator account to a wallet and connect with the development frontend. You can use another account as the one generated before to register the session keys but you need to set the session keys which belong to the validator.

Please follow the instructions in the [Custom Account Guide](https://github.com/worldmobilegroup/aya-node/blob/main/docs/guide_custom_account.md) to install Talisman. 
Than use the mnemonic generated earlier for your validator to restore the wallet in Talisman. 

Add the following Network to Talisman (Polkadot):
![AyA-DevNet RPC Endpoint](assets/talisman_aya_devnet.png)

Refresh the Polkadot-JS development front-end, connect the new wallet in Talisman to the Polkadot-JS front end, you should see your account now and if the setup of the RPC endpoint worked correctly you are able to sign and submit transactions with that account.

### Get FERN
You need to get some FERN tokens from the faucet to pay for transaction fees. 



## Become a Validator

To become a validator two more steps are needed. First you need to submit your session keys and second you need to let us know about your validator, so we can add it to the authority set. It takes two epochs after you were added to the authority set before you are part of the active, block producing authority set. An epoch last for 24 hours. If one of the session keys does not match the ones active in your validator, your validator cannot produce blocks and will be kicked from the authority set. 
If you do not provide ImOnline messages during an epoch you are getting kicked two epochs later (this can happen when the IMONLINE key is wrong or your validator is offline). We can add you again so no worries, but we do not observe your validator status so you need to let us know.

### Submit Session Keys
To submit the session keys you obtained in the step before you need a working Polkadot-JS front-end and your account needs to have funds to pay for transaction fees. The wallet registering the session keys does not need to be the one generated for the validator, but the session keys must match! Only you (the operator) should know your session keys when you triggered a rotation and only you should be able to rotate session keys. It is not possible to register the same session keys twice.

Go to the [Front-End](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2F35.189.75.1%3A#/explorer) and navigate to Developer -> Extrinsics.
![DeveloperExtrinsic](assets/sessionKeys_developerExtrinsic.png)

Select the validator account you imported into Talisman before. 
Select the `session` pallet from the extrinsic category pull down menu. 
Select `setKeys(keys,proof)` from the function selector pull down menu.
Insert the Aura-Key into the first field
Insert the Grandpa-Key into the second field. 
Insert the ImOnline-Key into the third field. 
For the proof enter `0x`

![sessionkeys_setkeys](assets/sessionkeys_setKeys.png)

Submit the transaction and sign with your wallet. 
If the transaction was submitted sucessfully you can check that your keys have been set:
Go to: Developers -> Chain State 
Select the `session` pallet in the state query, select the function `nextKeys(AccountId20):Option<AyaRuntimeOpaqueSessionKeys>`.
Select your Account and submit the request by clicking on the `+`.

Example Output: 
```
{
  aura: 0x5812bfae0ffa1f76d0dda8209f134b89d7d166f8a94e10e3d083400321913e7f
  grandpa: 0xe37716375e16d07b1b1333345fe766dbc03a46bd0c2be2c1c03d83f2aa9c1ee7
  imOnline: 0x94c75738ba17c57a1bfae1181b688ff6fa28cdf862fe696714d8d6f175a00230
}
```

### Register Validator

You have done all it needs, the only thing left is to let us know about your validator. Go to this [Form]() and fill the information. We check twice a day (mornings and evenings CET) for new validators and add them to the authority set. It takes two epochs until your validator joins the active authority set, an epoch is 24 hours so it can take 3 to 4 days before your validator becomes active. 

# Securing my Validator
We have setup a plain validator in the previous steps and connected it directly to the network. It is better to have the validator behind a full node (or two) which is exposed to the public, but our validator should only connect to that full node and not allow connections from the outside. 

This setup is fairly easy in substrate based chains. You setup a full node which connects to the network in the way described in this guide. The key related steps can be ignored for a full node. When you setup your validator you do not give the public bootnode in the `--bootnodes` parameter but your own full node. With additonal measuremeants (e.g. cloud firewall or ufw) you can limit the connections to your validator. Only the p2p port (default 30333) needs to be open if you want to connect to the validator with another node. For example we could open the port 30333 only for the internal network IP address of our fullnode.
