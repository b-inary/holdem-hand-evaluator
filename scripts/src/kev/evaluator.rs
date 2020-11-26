// Cactus Kev's hand evaluator with Senzee's modification.
// http://suffe.cool/poker/evaluator.html
// http://senzee.blogspot.com/search/label/Poker%20Hand%20Evaluation

use crate::kev::arrays::*;
use std::cmp;

const CARDS: [u32; 52] = [
    0x18002, 0x14002, 0x12002, 0x11002, 0x28103, 0x24103, 0x22103, 0x21103, 0x48205, 0x44205,
    0x42205, 0x41205, 0x88307, 0x84307, 0x82307, 0x81307, 0x10840b, 0x10440b, 0x10240b, 0x10140b,
    0x20850d, 0x20450d, 0x20250d, 0x20150d, 0x408611, 0x404611, 0x402611, 0x401611, 0x808713,
    0x804713, 0x802713, 0x801713, 0x1008817, 0x1004817, 0x1002817, 0x1001817, 0x200891d, 0x200491d,
    0x200291d, 0x200191d, 0x4008a1f, 0x4004a1f, 0x4002a1f, 0x4001a1f, 0x8008b25, 0x8004b25,
    0x8002b25, 0x8001b25, 0x10008c29, 0x10004c29, 0x10002c29, 0x10001c29,
];

#[inline]
fn find_fast(u: u32) -> usize {
    let u = u.wrapping_add(0xe91aaa35);
    let u = u ^ (u >> 16);
    let u = u.wrapping_add(u << 8);
    let u = u ^ (u >> 4);
    let b = (u >> 8) & 0x1ff;
    let a = u.wrapping_add(u << 2) >> 19;
    a as usize ^ HASH_ADJUST[b as usize] as usize
}

#[inline]
fn eval_5cards_fast(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u16 {
    let q = ((c1 | c2 | c3 | c4 | c5) >> 16) as usize;
    if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        return FLUSHES[q];
    }
    let s = UNIQUE5[q];
    if s != 0 {
        return s;
    }
    HASH_VALUES[find_fast((c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff))]
}

#[inline]
fn eval_6cards_fast(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32, c6: u32) -> u16 {
    let hand = [c1, c2, c3, c4, c5, c6];
    let mut best = 9999;
    for perm in &PERM6 {
        let mut subhand = Vec::new();
        for i in 0..5 {
            subhand.push(hand[perm[i]]);
        }
        let q = eval_5cards_fast(subhand[0], subhand[1], subhand[2], subhand[3], subhand[4]);
        best = cmp::min(best, q);
    }
    best
}

#[inline]
fn eval_7cards_fast(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32, c6: u32, c7: u32) -> u16 {
    let hand = [c1, c2, c3, c4, c5, c6, c7];
    let mut best = 9999;
    for perm in &PERM7 {
        let mut subhand = Vec::new();
        for i in 0..5 {
            subhand.push(hand[perm[i]]);
        }
        let q = eval_5cards_fast(subhand[0], subhand[1], subhand[2], subhand[3], subhand[4]);
        best = cmp::min(best, q);
    }
    best
}

#[inline]
pub fn eval_5cards(c1: usize, c2: usize, c3: usize, c4: usize, c5: usize) -> u16 {
    eval_5cards_fast(CARDS[c1], CARDS[c2], CARDS[c3], CARDS[c4], CARDS[c5])
}

#[inline]
pub fn eval_6cards(c1: usize, c2: usize, c3: usize, c4: usize, c5: usize, c6: usize) -> u16 {
    eval_6cards_fast(
        CARDS[c1], CARDS[c2], CARDS[c3], CARDS[c4], CARDS[c5], CARDS[c6],
    )
}

#[inline]
pub fn eval_7cards(
    c1: usize,
    c2: usize,
    c3: usize,
    c4: usize,
    c5: usize,
    c6: usize,
    c7: usize,
) -> u16 {
    eval_7cards_fast(
        CARDS[c1], CARDS[c2], CARDS[c3], CARDS[c4], CARDS[c5], CARDS[c6], CARDS[c7],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum HandCategory {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        Straight,
        Flush,
        FullHouse,
        FourOfAKind,
        StraightFlush,
    }

    fn get_hand_category(rank: u16) -> HandCategory {
        match rank {
            1..=10 => HandCategory::StraightFlush,
            11..=166 => HandCategory::FourOfAKind,
            167..=322 => HandCategory::FullHouse,
            323..=1599 => HandCategory::Flush,
            1600..=1609 => HandCategory::Straight,
            1610..=2467 => HandCategory::ThreeOfAKind,
            2468..=3325 => HandCategory::TwoPair,
            3326..=6185 => HandCategory::OnePair,
            6186..=7462 => HandCategory::HighCard,
            _ => panic!(),
        }
    }

    #[test]
    fn test_all_5card_combinations() {
        let mut rankset = HashSet::new();
        let mut counter = HashMap::new();

        for i in 0..48 {
            for j in (i + 1)..49 {
                for k in (j + 1)..50 {
                    for m in (k + 1)..51 {
                        for n in (m + 1)..52 {
                            let rank = eval_5cards(i, j, k, m, n);
                            let category = get_hand_category(rank);
                            rankset.insert(rank);
                            let c = counter.entry(category).or_insert(0);
                            *c += 1;
                        }
                    }
                }
            }
        }

        // reference: http://suffe.cool/poker/evaluator.html
        assert_eq!(rankset.len(), 7462);
        assert_eq!(counter.get(&HandCategory::StraightFlush), Some(&40));
        assert_eq!(counter.get(&HandCategory::FourOfAKind), Some(&624));
        assert_eq!(counter.get(&HandCategory::FullHouse), Some(&3744));
        assert_eq!(counter.get(&HandCategory::Flush), Some(&5108));
        assert_eq!(counter.get(&HandCategory::Straight), Some(&10200));
        assert_eq!(counter.get(&HandCategory::ThreeOfAKind), Some(&54912));
        assert_eq!(counter.get(&HandCategory::TwoPair), Some(&123552));
        assert_eq!(counter.get(&HandCategory::OnePair), Some(&1098240));
        assert_eq!(counter.get(&HandCategory::HighCard), Some(&1302540));
    }
}
