use assets::constants::*;
use criterion::{criterion_group, criterion_main, Criterion};
use evaluator::{add_card, evaluate_hand};

fn bench() -> u16 {
    let mut sum: u16 = 0;
    for i in 0..(NUMBER_OF_CARDS - 6) {
        let (hand, mask) = add_card(0, 0, i);
        for j in (i + 1)..(NUMBER_OF_CARDS - 5) {
            let (hand, mask) = add_card(hand, mask, j);
            for k in (j + 1)..(NUMBER_OF_CARDS - 4) {
                let (hand, mask) = add_card(hand, mask, k);
                for m in (k + 1)..(NUMBER_OF_CARDS - 3) {
                    let (hand, mask) = add_card(hand, mask, m);
                    for n in (m + 1)..(NUMBER_OF_CARDS - 2) {
                        let (hand, mask) = add_card(hand, mask, n);
                        for p in (n + 1)..(NUMBER_OF_CARDS - 1) {
                            let (hand, mask) = add_card(hand, mask, p);
                            for q in (p + 1)..NUMBER_OF_CARDS {
                                let (hand, mask) = add_card(hand, mask, q);
                                let rank = evaluate_hand(hand, mask);
                                sum = sum.wrapping_add(rank);
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
