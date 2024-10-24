#!/bin/bash

if [ -d "test-ledger" ]; then
    echo "Removing test-ledger directory..."
    rm -rf test-ledger
else
    echo "test-ledger directory does not exist."
fi

function is_validator_running() {
    pgrep -f "solana-test-validator" > /dev/null 2>&1
    return $?
}

if is_validator_running; then
    echo "Stopping running Solana test validator..."
    pkill -f "solana-test-validator"
    sleep 2
else
    echo "No Solana test validator is currently running."
fi

echo "Starting Solana test validator..."
solana-test-validator &

sleep 5

echo "Airdropping 1000 SOL to the address 8k8fH1rXUFFAG7JdKKT3awJP3Y58bHMimiABvsCGWQmu..."
solana airdrop 1000 8k8fH1rXUFFAG7JdKKT3awJP3Y58bHMimiABvsCGWQmu

echo "Airdrop complete."
