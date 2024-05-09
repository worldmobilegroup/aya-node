#!/usr/bin/env bash
set -e

if [[ $# -ne 1 ]]; then
    MNEMONIC="bottom drive obey lake curtain smoke basket hold race lonely fit walk"
else
    if [[ ! $(echo $1 | wc -w) =~ ^(12|24)$ ]]; then
        echo "Please provide a valid string of 12 or 24 words!"
        exit 1
    else
        MNEMONIC=$1
    fi
fi

get_public_key_ss58() {
   subkey inspect ${1} "${2}" | grep "Public key (SS58)" | awk '{ print $4 }'
}

get_account_id() {
   subkey inspect ${1} "${2}"
}

echo "Processing account:"
echo; echo "sr25519"; echo
get_account_id '--scheme sr25519' "$MNEMONIC"
echo; echo "ed25519"; echo
get_account_id '--scheme ed25519' "$MNEMONIC"
echo
npx ts-node tools/keys/index.ts "${MNEMONIC}" "0"
echo; echo "------------"; echo