// generate lookup tables.

mod kev;

use assets::constants::*;
use assets::offsets::{MIX_MULTIPLIER, OFFSETS};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn adjust_hand_rank(rank: u16) -> u16 {
    let reversed_rank = 7463 - rank; // now best hand = 7462
    match reversed_rank {
        1..=1277 => reversed_rank - 1,                   // 1277 high card
        1278..=4137 => (1 << 12) + reversed_rank - 1278, // 2860 one pair
        4138..=4995 => (2 << 12) + reversed_rank - 4138, //  858 two pair
        4996..=5853 => (3 << 12) + reversed_rank - 4996, //  858 three-kind
        5854..=5863 => (4 << 12) + reversed_rank - 5854, //   10 straights
        5864..=7140 => (5 << 12) + reversed_rank - 5864, // 1277 flushes
        7141..=7296 => (6 << 12) + reversed_rank - 7141, //  156 full house
        7297..=7452 => (7 << 12) + reversed_rank - 7297, //  156 four-kind
        7453..=7462 => (8 << 12) + reversed_rank - 7453, //   10 straight flushes
        _ => panic!(),
    }
}

#[inline]
fn add_card(key: u64, mask: u64, card: usize) -> (u64, u64) {
    let (k, m) = unsafe { *CARDS.get_unchecked(card) };
    (key.wrapping_add(k), mask.wrapping_add(m))
}

#[inline]
fn update(
    key: u64,
    mask: u64,
    val: u16,
    lookup: &mut HashMap<usize, u16>,
    lookup_flush: &mut HashMap<usize, u16>,
) {
    let is_flush = key & FLUSH_MASK;
    if is_flush > 0 {
        let flush_key = (mask >> (4 * is_flush.leading_zeros())) as u16;
        match lookup_flush.insert(flush_key as usize, val) {
            Some(v) => assert_eq!(val, v),
            None => (),
        };
    } else {
        let mixed_key = (key.wrapping_mul(MIX_MULTIPLIER) & RANK_KEY_MASK) as usize;
        let offset = OFFSETS[mixed_key >> OFFSET_SHIFT] as usize;
        let hash_key = mixed_key.wrapping_add(offset);
        match lookup.insert(hash_key, val) {
            Some(v) => assert_eq!(val, v),
            None => (),
        }
    }
}

fn main() {
    let mut lookup = HashMap::new();
    let mut lookup_flush = HashMap::new();

    // 5-cards
    for i in 0..(NUMBER_OF_CARDS - 4) {
        let (key, mask) = add_card(0x3333 << SUIT_SHIFT, 0, i);
        for j in (i + 1)..(NUMBER_OF_CARDS - 3) {
            let (key, mask) = add_card(key, mask, j);
            for k in (j + 1)..(NUMBER_OF_CARDS - 2) {
                let (key, mask) = add_card(key, mask, k);
                for m in (k + 1)..(NUMBER_OF_CARDS - 1) {
                    let (key, mask) = add_card(key, mask, m);
                    for n in (m + 1)..NUMBER_OF_CARDS {
                        let (key, mask) = add_card(key, mask, n);
                        update(
                            key,
                            mask,
                            kev::eval_5cards(i, j, k, m, n),
                            &mut lookup,
                            &mut lookup_flush,
                        );
                    }
                }
            }
        }
    }

    // 6-cards
    for i in 0..(NUMBER_OF_CARDS - 5) {
        let (key, mask) = add_card(0x3333 << SUIT_SHIFT, 0, i);
        for j in (i + 1)..(NUMBER_OF_CARDS - 4) {
            let (key, mask) = add_card(key, mask, j);
            for k in (j + 1)..(NUMBER_OF_CARDS - 3) {
                let (key, mask) = add_card(key, mask, k);
                for m in (k + 1)..(NUMBER_OF_CARDS - 2) {
                    let (key, mask) = add_card(key, mask, m);
                    for n in (m + 1)..(NUMBER_OF_CARDS - 1) {
                        let (key, mask) = add_card(key, mask, n);
                        for p in (n + 1)..NUMBER_OF_CARDS {
                            let (key, mask) = add_card(key, mask, p);
                            update(
                                key,
                                mask,
                                kev::eval_6cards(i, j, k, m, n, p),
                                &mut lookup,
                                &mut lookup_flush,
                            );
                        }
                    }
                }
            }
        }
    }

    // 7-cards
    for i in 0..(NUMBER_OF_CARDS - 6) {
        let (key, mask) = add_card(0x3333 << SUIT_SHIFT, 0, i);
        for j in (i + 1)..(NUMBER_OF_CARDS - 5) {
            let (key, mask) = add_card(key, mask, j);
            for k in (j + 1)..(NUMBER_OF_CARDS - 4) {
                let (key, mask) = add_card(key, mask, k);
                for m in (k + 1)..(NUMBER_OF_CARDS - 3) {
                    let (key, mask) = add_card(key, mask, m);
                    for n in (m + 1)..(NUMBER_OF_CARDS - 2) {
                        let (key, mask) = add_card(key, mask, n);
                        for p in (n + 1)..(NUMBER_OF_CARDS - 1) {
                            let (key, mask) = add_card(key, mask, p);
                            for q in (p + 1)..NUMBER_OF_CARDS {
                                let (key, mask) = add_card(key, mask, q);
                                update(
                                    key,
                                    mask,
                                    kev::eval_7cards(i, j, k, m, n, p, q),
                                    &mut lookup,
                                    &mut lookup_flush,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    let mut lookup_vec = vec![0; lookup.keys().max().unwrap() + 1];
    let mut lookup_flush_vec = vec![0; *lookup_flush.keys().max().unwrap() + 1];

    for (key, value) in &lookup {
        lookup_vec[*key] = adjust_hand_rank(*value);
    }

    for (key, value) in &lookup_flush {
        lookup_flush_vec[*key] = adjust_hand_rank(*value);
    }

    let mut file = File::create("assets/src/lookup.rs").unwrap();
    writeln!(
        file,
        "pub const LOOKUP: [u16; {}] = {:?};",
        lookup_vec.len(),
        lookup_vec
    )
    .unwrap();
    writeln!(file).unwrap();
    writeln!(
        file,
        "pub const LOOKUP_FLUSH: [u16; {}] = {:?};",
        lookup_flush_vec.len(),
        lookup_flush_vec
    )
    .unwrap();

    println!("wrote result to 'assets/src/lookup.rs'");
}
