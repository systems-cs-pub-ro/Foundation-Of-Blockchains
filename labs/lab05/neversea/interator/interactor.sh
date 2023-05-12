#!/bin/bash

mxpy contract deploy --bytecode output/neversea.wasm --pem ~/multiversX/keys/shard0.pem  --recall-nonce --gas-limit 60000000 --proxy https://testnet-gateway.multiversx.com --chain T --argument 100 200 --send
