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

get_account_names() {
    CONFIG_FILE_PATH="config.json"
    if [ ! -f "$CONFIG_FILE_PATH" ]; then
        echo "Configuration file does not exist"
        # TODO: exit the script
        exit 2
    fi
    while IFS= read -r line; do
        ACCOUNT_NAMES+=("$line")
    done < <(jq -r '.accounts[]' "$CONFIG_FILE_PATH")
    if [ ${#ACCOUNT_NAMES[@]} -eq 0 ]; then
        echo "The config file does not have any account names"
        exit 3
    fi
    echo "${ACCOUNT_NAMES[@]}"
}

get_public_key_ss58() {
   subkey inspect ${1} "${2}" | grep "Public key (SS58)" | awk '{ print $4 }'
}

get_account_id() {
   subkey inspect ${1} "${2}"
}

IFS=' ' read -r -a account_names <<< "$(get_account_names)"

for index in "${!account_names[@]}"
do
  echo "Processing account: ${index} - ${account_names[$index]}"
  subkey inspect --scheme ed25519 "$MNEMONIC//${account_names[$index]}" | grep "Secret seed"
  echo; echo "sr25519"; echo
  get_account_id '--scheme sr25519' "$MNEMONIC//${account_names[$index]}"
  echo; echo "ed25519"; echo
  get_account_id '--scheme ed25519' "$MNEMONIC//${account_names[$index]}"
  echo
  npx ts-node tools/keys/index.ts "${MNEMONIC}" "${index}"
  echo; echo "------------"; echo
done
