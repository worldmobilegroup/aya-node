yes | ./target/release/aya-node purge-chain \
--base-path /tmp/alice \
--chain local 

yes | ./target/release/aya-node purge-chain \
--base-path /tmp/bob \
--chain local 

yes | ./target/release/aya-node purge-chain \
--base-path /tmp/charlie \
--chain local
