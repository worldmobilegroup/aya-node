curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"submit_event","params":[{"id":0,"data":"{\"type\":\"EpochChange\",\"data\":{\"last_epoch\":574,\"last_blockhash\":\"de3ea4083d96987a9a3d2f1df14a009fdb548f7063a40fb707d2b87ca471cc5d\",\"last_slot\":49679976,\"new_epoch\":575,\"new_slot\":49680007,\"new_blockhash\":\"18f9fe3cce213d40f8f16e16e73dad7dd28cf394d7e25c720cc83324ca8fa560\",\"epoch_nonce\":\"8972981c2fa11e815ab0b89e7c1e1603fe30b2c4d4eb6becaf109bf2fd912a22\",\"extra_entropy\":null}}","timestamp":0,"block_height":0}, 1],"id":1}'
curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"list_all_events","params":[],"id":1}'

