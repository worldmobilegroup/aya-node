const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });

  const blockHash = await api.rpc.chain.getBlockHash(2); // Replace with your block number
  const signedBlock = await api.rpc.chain.getBlock(blockHash);
  const allRecords = await api.query.system.events.at(blockHash);

  console.log(`Block hash: ${blockHash}`);
  console.log(`Block number: ${signedBlock.block.header.number}`);

  signedBlock.block.extrinsics.forEach(({ method: { method, section }, isSigned, args }, index) => {
    console.log(`Extrinsic ${index}: ${section}.${method}:: ${isSigned ? 'signed' : 'unsigned'}`);
    console.log(`Arguments: ${args.map((a) => a.toString()).join(', ')}`);
  });

  process.exit(0);
}

main().catch(console.error);
