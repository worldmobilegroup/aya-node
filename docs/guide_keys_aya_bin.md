# Generate your keys using Aya-Node binary

Keys can be generated and inspected with the aya-node binary

Generate key:
```bash
./target/release/aya-node key generate
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

You should make a copy of these keys and keep them safe!

Inspect key: (example below is for inspecting ALICE's seed phrase - replace it with your own seed )
```bash
./target/release/aya-node key inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
```