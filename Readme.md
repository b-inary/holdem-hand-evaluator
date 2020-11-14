# holdem-hand-evaluator

Super fast hand rank evaluator for Texas hold'em for Rust (~800M eval/s sequential @Ryzen 7 3700X single-threaded)

## Usage

`Cargo.toml`
```toml
[dependencies]
holdem-hand-evaluator = { git = "https://github.com/b-inary/holdem-hand-evaluator", branch = "main" }
```

`example.rs`

```rust
use holdem_hand_evaluator::{get_hand_category, Hand};

fn main() {
    // card ID: 0-3 => 2c2d2h2s, 4-7 => 3c3d3h3s, ..., 48-51 => AcAdAhAs

    // construct hand one by one
    // argument of add_card() must be in range [0, 51] and must not be duplicated
    // (there are no error checks)
    let mut hand1 = Hand::new();
    hand1 = hand1.add_card(2); // 2h
    hand1 = hand1.add_card(3); // 2s
    hand1 = hand1.add_card(5); // 3d
    hand1 = hand1.add_card(7); // 3s
    hand1 = hand1.add_card(11); // 4s
    hand1 = hand1.add_card(13); // 5d
    hand1 = hand1.add_card(17); // 6d

    // construct hand from Vec (also there are no error checks)
    let hand2 = Hand::from_vec(&vec![19, 23, 29, 31, 37, 41, 43]); // 6s7s9d9sJdQdQs

    // construct hand from String
    let hand3 = "AhKhQhJhTh8c6d".parse::<Hand>().unwrap();

    // evaluate() function computes the hand rank (stronger hand yields higher value)
    // only supports 7-card hand (again there are no error checks)
    assert_eq!(hand1.len(), 7);
    let rank1 = hand1.evaluate();
    assert_eq!(hand2.len(), 7);
    let rank2 = hand2.evaluate();
    assert_eq!(hand3.len(), 7);
    let rank3 = hand3.evaluate();

    println!("rank1: {}", rank1); // 16385
    println!("category1: {:?}", get_hand_category(rank1)); // Straight

    println!("rank2: {}", rank2); // 8772
    println!("category2: {:?}", get_hand_category(rank2)); // TwoPair

    println!("rank3: {}", rank3); // 32777
    println!("category3: {:?}", get_hand_category(rank3)); // StraightFlush
}
```

## Generate Assets (optional)

Sources in [scripts](scripts) directory generate constants used in [assets](assets) directory.

See [scripts/Readme.md](scripts/Readme.md) for defails.

```sh
$ cargo run -p holdem-hand-evaluator-scripts --bin 01-rank_bases --release
$ cargo run -p holdem-hand-evaluator-scripts --bin 02-flush_table --release
$ cargo run -p holdem-hand-evaluator-scripts --bin 03-offset_table --release
$ cargo run -p holdem-hand-evaluator-scripts --bin 04-lookup_tables --release
```

## Run Tests

```sh
$ cargo test --release
$ cargo test -p holdem-hand-evaluator-scripts --release
```

## Run Benchmark

```sh
$ cargo bench
```
