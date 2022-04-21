#!/bin/bash
set -xeu

sleep 60
curl -v http://rialto-node-alice:9933/health
curl -v http://rialto-allychain-collator-alice:9933/health

/home/user/axlib-relay register-allychain rialto-allychain \
	--allychain-host rialto-allychain-collator-alice \
	--allychain-port 9944 \
	--relaychain-host rialto-node-alice \
	--relaychain-port 9944 \
	--relaychain-signer //Alice
