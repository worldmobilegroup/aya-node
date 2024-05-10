const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Adjust the URL to your node's WebSocket endpoint
    try {
        const api = await ApiPromise.create({ provider: wsProvider });

        // Fetch and log available RPC methods
        const methods = await api.rpc.rpc.methods();
        console.log("Available methods:", methods.map(method => method.method).join(', '));

        // Attempt to call the custom RPC method
        const result = await api.rpc.cardanoFollower.submitCardanoEvent(JSON.stringify({data: "Test event data"}));
        console.log('Event submission result:', result.toHuman());
    } catch (error) {
        console.error('Error:', error);
    }
}

main().catch(console.error);
