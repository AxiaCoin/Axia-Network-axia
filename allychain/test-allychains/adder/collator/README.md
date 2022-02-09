# How to run this collator

First start two validators that will run for the relay chain:

```sh
cargo run --release -- -d alice --chain betanet-local --validator --alice --port 50551
cargo run --release -- -d bob --chain betanet-local --validator --bob --port 50552
```

Next start the collator that will collate for the adder allychain:

```sh
cargo run --release -p test-allychain-adder-collator -- --tmp --chain betanet-local --port 50553
```

The last step is to register the allychain using axia-js. The allychain id is
100. The genesis state and the validation code are printed at startup by the collator.

To do this automatically, run `scripts/adder-collator.sh`.
