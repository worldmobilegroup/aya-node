# Generate your keys using subkey

## Setup your environment

Configure Rust Nightly

```bash
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Check Rust Nightly installation:

```bash
rustup +nightly show
```

## Install subkey

Please follow the official documentation to install
subkey: [Subkey Docs](https://docs.substrate.io/reference/command-line-tools/subkey/)

## Setup your environment

From the `polkadot-sdk` repository folder, copy the compiled subkey binary to `/usr/bin` to be able to use it from any
location in the system:

```bash
sudo cp ./target/release/subkey /usr/bin/
```

Check subkey is installed:

```bash
subkey --version
```

You can delete the polkadot-sdk repository when subkey works as expected. You will not need it anymore in this guide.


## Create keys

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

The command `subkey inspect` will produce the same reuslt but the mnemonic is given as input parameter.

```bash 
subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
```

> [!NOTE]  Additional Information:
> *You can derive accounts like this:*
> ```bash 
> subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"
> ```
> It will produce the well known account of Alice. `//` Does a hard derivation `/` a soft derivation
