// Import the Polkadot API library
const { ApiPromise, WsProvider } = require('@polkadot/api');

// Define the WebSocket endpoint
const wsProvider = new WsProvider('ws://127.0.0.1:9944');

async function main() {
    const api = await ApiPromise.create({ provider: wsProvider });

    // Subscribe to system events
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events:`);

        // Loop through the events and print them
        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            // Show what we are busy with
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\t\t${event.meta.documentation ? event.meta.documentation.toString() : 'No documentation'}`);

            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data, index) => {
                const type = types[index] ? types[index].type : 'Unknown';
                const value = data.toHuman ? data.toHuman() : JSON.stringify(data);
                console.log(`\t\t\t${type}: ${value}`);
            });
        });
    });

    // Subscribe to pending extrinsics
    api.rpc.author.pendingExtrinsics((extrinsics) => {
        console.log(`\nReceived ${extrinsics.length} pending extrinsics:`);

        extrinsics.forEach((extrinsic, index) => {
            console.log(`\nExtrinsic ${index + 1}:`);
            console.log(extrinsic.toHuman ? extrinsic.toHuman() : JSON.stringify(extrinsic));
        });
    });
}

main().catch(console.error);
