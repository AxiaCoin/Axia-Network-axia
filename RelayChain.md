### Start Relay Chain

```bash
# Start alice chain
./target/release/axia \
--chain ./betanet_local.json \
-d /tmp/relay/alice \
--validator \
--alice \
--port 50555
```

```bash
# Start Relay `Bob` node
./target/release/axia \
--chain ./betanet_local.json \
-d /tmp/relay/bob \
--validator \
--bob \
--port 50556
```