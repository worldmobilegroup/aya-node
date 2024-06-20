const { ApiPromise, WsProvider } = require('@polkadot/api');
const { hexToString } = require('@polkadot/util');

async function main() {
    // WebSocket provider with reconnection logic
    const wsProvider = new WsProvider('ws://localhost:9944', false);
    
    wsProvider.on('connected', () => {
        console.log('Connected to the node');
    });
    
    wsProvider.on('disconnected', () => {
        console.log('Disconnected from the node, attempting to reconnect...');
        setTimeout(() => wsProvider.connect(), 1000); // Attempt to reconnect after 1 second
    });
    
    wsProvider.on('error', (error) => {
        console.error('WebSocket error:', error.message);
    });
    
    wsProvider.on('connecting', () => {
        console.log('Connecting to the node...');
    });

    console.log('Connecting to WebSocket...');
    await wsProvider.connect();
    console.log('WebSocket connected');

    console.log('Creating API instance...');
    const api = await ApiPromise.create({ provider: wsProvider });
    console.log('API instance created');

    // Ensure the node is reachable
    try {
        const [chain, nodeName, nodeVersion] = await Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version()
        ]);

        console.log(`Connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    } catch (error) {
        console.error('Error connecting to the chain:', error.message);
        return;
    }

    // Subscribe to new blocks
    api.rpc.chain.subscribeNewHeads(async (lastHeader) => {
        console.log(`Chain is at block: #${lastHeader.number}`);

        // Fetch block details
        const signedBlock = await api.rpc.chain.getBlock(lastHeader.hash);
        const allEvents = await api.query.system.events.at(lastHeader.hash);

        signedBlock.block.extrinsics.forEach((extrinsic, index) => {
            const { method: { method, section } } = extrinsic;
            const events = allEvents.filter(({ phase }) =>
                phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index)
            );

            console.log(`\nExtrinsic ${index}: ${section}.${method}:: ${extrinsic.meta.documentation}`);

            // Log detailed extrinsic info
            console.log(`Extrinsic details: ${JSON.stringify(extrinsic.toHuman(), null, 2)}`);

            // Log the transaction hash
            console.log(`Transaction hash: ${extrinsic.hash.toHex()}`);

            // Decode the extrinsic if it's from offchain worker
            if (section === 'offchainWorker' && method === 'submitTransaction') {
                const decoded = api.createType('Extrinsic', extrinsic.toHex());
                console.log(`Decoded extrinsic: ${decoded}`);

                // Assuming the payload is in a specific format, decode the payload
                const payload = decoded.args[0].toHuman();
                console.log(`Decoded payload: ${hexToString(payload)}`);
            }

            events.forEach(({ event }) => {
                console.log(`\t${event.section}.${event.method}:: ${event.meta.documentation}`);
            });
        });
    });
}

main().catch((error) => {
    console.error('Error in main function:', error.message);
});
