#!/bin/bash

# Run a development instance of the Betanet Axlib bridge node.
# To override the default port just export BETANET_PORT=9955
#
# Note: This script will not work out of the box with the bridges
# repo since it relies on a Axia binary.

BETANET_PORT="${BETANET_PORT:-9955}"

RUST_LOG=runtime=trace,runtime::bridge=trace \
./target/debug/axia --chain=betanet-dev --alice --tmp \
    --rpc-cors=all --unsafe-rpc-external --unsafe-ws-external \
    --port 33044 --rpc-port 9934 --ws-port $BETANET_PORT \
