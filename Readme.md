# holdem-hand-evaluator

Super fast hand rank evaluator for Texas hold'em written in Rust (~800M eval/s sequential @Ryzen 7 3700X single-threaded)

## Generate Assets (optional)

Sources in [scripts](scripts) directory generate constants used in [assets](assets) directory.

See [Readme.md](scripts/Readme.md) in scripts directory for defails.

```sh
$ cargo run -p holdem-hand-evaluator-scripts --bin 01-rank_bases --release
$ cargo run -p holdem-hand-evaluator-scripts --bin 02-flush_table --release
$ cargo run -p holdem-hand-evaluator-scripts --bin 03-offset_table --release
$ cargo run -p holdem-hand-evaluator-scripts --bin 04-lookup_tables --release
```

## Run Tests

```sh
$ cargo test --release
```

## Run Benchmark

```sh
$ cargo bench
```
