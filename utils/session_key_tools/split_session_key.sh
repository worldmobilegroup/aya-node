#!/usr/bin/env bash
set -e

if [[ $# -ne 1 ]]; then
    echo "Please provide a session key as parameter to the script!"
    exit 1
else
    SESSION_KEY=$1
    if [[ ! ${#SESSION_KEY} -eq 194 ]]; then
        echo "Please provide a valid session key!"
        exit 1
    fi
fi

echo "------------------------------------"
echo "Your session keys:"
echo AURA_SESSION_KEY=${SESSION_KEY:0:66}
echo GRANDPA_SESSION_KEY=0x${SESSION_KEY:66:64}
echo IM_ONLINE_SESSION_KEY=0x${SESSION_KEY:130:64}
echo "------------------------------------"