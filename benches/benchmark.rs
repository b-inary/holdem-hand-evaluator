use assets::constants::*;
use criterion::{criterion_group, criterion_main, Criterion};
use holdem_hand_evaluator::Hand;

fn bench() -> u16 {
    let mut sum: u16 = 0;
    for i in 0..(NUMBER_OF_CARDS - 6) {
        let hand = Hand::new().add_card(i);
        for j in (i + 1)..(NUMBER_OF_CARDS - 5) {
            let hand = hand.add_card(j);
            for k in (j + 1)..(NUMBER_OF_CARDS - 4) {
                let hand = hand.add_card(k);
                for m in (k + 1)..(NUMBER_OF_CARDS - 3) {
                    let hand = hand.add_card(m);
                    for n in (m + 1)..(NUMBER_OF_CARDS - 2) {
                        let hand = hand.add_card(n);
                        for p in (n + 1)..(NUMBER_OF_CARDS - 1) {
                            let hand = hand.add_card(p);
                            for q in (p + 1)..NUMBER_OF_CARDS {
                                let hand = hand.add_card(q);
                                let rank = hand.evaluate();
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
    c.bench_function("evaluate_hand (133,784,560 hands)", |b| b.iter(|| bench()));
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
