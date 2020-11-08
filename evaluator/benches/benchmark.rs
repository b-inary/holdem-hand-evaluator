use assets::constants::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench() -> u16 {
    let mut sum: u16 = 0;
    for i in 0..(NUMBER_OF_CARDS - 6) {
        let hand = CARDS[i];
        let mask = CARDS_BIT[i];
        for j in (i + 1)..(NUMBER_OF_CARDS - 5) {
            let hand = hand + CARDS[j];
            let mask = mask | CARDS_BIT[j];
            for k in (j + 1)..(NUMBER_OF_CARDS - 4) {
                let hand = hand + CARDS[k];
                let mask = mask | CARDS_BIT[k];
                for m in (k + 1)..(NUMBER_OF_CARDS - 3) {
                    let hand = hand + CARDS[m];
                    let mask = mask | CARDS_BIT[m];
                    for n in (m + 1)..(NUMBER_OF_CARDS - 2) {
                        let hand = hand + CARDS[n];
                        let mask = mask | CARDS_BIT[n];
                        for p in (n + 1)..(NUMBER_OF_CARDS - 1) {
                            let hand = hand + CARDS[p];
                            let mask = mask | CARDS_BIT[p];
                            for q in (p + 1)..NUMBER_OF_CARDS {
                                let hand = hand + CARDS[q];
                                let mask = mask | CARDS_BIT[q];
                                sum = sum.wrapping_add(evaluator::evaluate_hand(hand, mask));
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("evaluate_hand (133,784,560 times)", |b| b.iter(|| bench()));
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
