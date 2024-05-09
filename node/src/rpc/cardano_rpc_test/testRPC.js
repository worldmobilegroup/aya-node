const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Adjust the URL to your node's WebSocket endpoint
    const api = await ApiPromise.create({ provider: wsProvider });

    try {
        // Replace 'submitCardanoEvent' with the actual method name if different
        const result = await api.rpc.cardanoFollower.submitCardanoEvent(JSON.stringify({data: "Test event data"}));
        console.log('Event submission result:', result.toHuman());
    } catch (error) {
        console.error('Error submitting event:', error);
    }
}

main().catch(console.error);
//curl -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' http://<Host-Machine-IP>:9944
