import {computeAddress} from "ethers/lib/utils";
const ethers = require('ethers');

function parseArguments(): [string, number] {
    const args = process.argv.slice(2);

    if (args.length !== 2) {
        console.error("Expected exactly 2 arguments, but got " + args.length);
        process.exit(1);
    }

    let mnemonic = args[0];

    let derivationIndex = parseInt(args[1]);

    if (!Number.isInteger(derivationIndex) || derivationIndex < 0) {
        console.error("Derivation index should be a positive integer");
        process.exit(1);
    }

    return [mnemonic, derivationIndex];
}

let [mnemonic, derivationIndex] = parseArguments()
const DERIVATION_PATH_BASE = "m/44'/60'/0'/0/";

let masterNode = ethers.Wallet.fromMnemonic(`${mnemonic}`, `${DERIVATION_PATH_BASE}${derivationIndex}`);
console.log(`Compressed EVM key: \t\t${computeAddress(masterNode.publicKey)}`);
console.log("Private key: \t\t\t" + masterNode.privateKey);
