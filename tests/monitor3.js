const { ApiPromise, WsProvider } = require('@polkadot/api');
const { hexToString } = require('@polkadot/util');

async function main() {
    const wsProvider = new WsProvider('ws://localhost:9944', false);

    wsProvider.on('connected', () => {
        console.log('Connected to the node');
    });

    wsProvider.on('disconnected', () => {
        console.log('Disconnected from the node, attempting to reconnect...');
        setTimeout(() => wsProvider.connect(), 1000);
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

    const api = await ApiPromise.create({ provider: wsProvider });
    console.log('API instance created');

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

    api.rpc.chain.subscribeNewHeads(async (lastHeader) => {
        console.log(`Chain is at block: #${lastHeader.number}`);

        const signedBlock = await api.rpc.chain.getBlock(lastHeader.hash);
        const allEvents = await api.query.system.events.at(lastHeader.hash);

        signedBlock.block.extrinsics.forEach((extrinsic, index) => {
            const { method: { method, section } } = extrinsic;
            const events = allEvents.filter(({ phase }) =>
                phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index)
            );

            console.log(`\nExtrinsic ${index}: ${section}.${method}:: ${extrinsic.meta.documentation}`);
            console.log(`Extrinsic details: ${JSON.stringify(extrinsic.toHuman(), null, 2)}`);
            console.log(`Transaction hash: ${extrinsic.hash.toHex()}`);

            events.forEach(({ event }) => {
                console.log(`\t${event.section}.${event.method}:: ${event.meta.documentation}`);

                if (event.section === 'system' && event.method === 'ExtrinsicFailed') {
                    const [dispatchError] = event.data;
                    if (dispatchError.isModule) {
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { documentation, method, section } = decoded;
                        console.log(`\t\tError: ${section}.${method}: ${documentation.join(' ')}`);
                    } else {
                        console.log(`\t\tError: ${dispatchError.toString()}`);
                    }
                }
            });
        });
    });
}

main().catch((error) => {
    console.error('Error in main function:', error.message);
});
