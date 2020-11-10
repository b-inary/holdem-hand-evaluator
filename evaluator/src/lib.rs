pub mod card;

use assets::constants::*;
use assets::flush_table::FLUSH_TABLE;
use assets::lookup::{LOOKUP, LOOKUP_FLUSH};
use assets::offsets::{MIX_MULTIPLIER, OFFSETS};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandCategory {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn get_hand_category(rank: u16) -> HandCategory {
    match rank >> 12 {
        0 => HandCategory::HighCard,
        1 => HandCategory::OnePair,
        2 => HandCategory::TwoPair,
        3 => HandCategory::ThreeOfAKind,
        4 => HandCategory::Straight,
        5 => HandCategory::Flush,
        6 => HandCategory::FullHouse,
        7 => HandCategory::FourOfAKind,
        8 => HandCategory::StraightFlush,
        _ => panic!(),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hand {
    key: u64,
    mask: u64,
}

impl Hand {
    #[inline]
    pub fn new() -> Hand {
        Hand { key: 0, mask: 0 }
    }

    #[inline]
    pub fn add_card(&self, card: usize) -> Hand {
        let (k, m) = unsafe { *CARDS.get_unchecked(card) };
        Hand {
            key: self.key.wrapping_add(k),
            mask: self.mask.wrapping_add(m),
        }
    }

    #[inline]
    pub fn remove_card(&self, card: usize) -> Hand {
        let (k, m) = unsafe { *CARDS.get_unchecked(card) };
        Hand {
            key: self.key.wrapping_sub(k),
            mask: self.mask.wrapping_sub(m),
        }
    }
}

/// Returns hand strength in 16-bit integer.
/// - `hand`: unique value that represents 7-card combination
/// - `mask`: bit mask with exactly 7 bits set to 1 (suits are in 16-bit groups)
#[inline]
pub fn evaluate_hand(hand: Hand) -> u16 {
    let suit_key = (hand.key >> RANK_KEY_BITS) as usize;
    let is_flush = unsafe { *FLUSH_TABLE.get_unchecked(suit_key) };
    if is_flush >= 0 {
        let flush_key = (hand.mask >> (16 * is_flush as usize)) & ((1 << NUMBER_OF_RANKS) - 1);
        unsafe { *LOOKUP_FLUSH.get_unchecked(flush_key as usize) }
    } else {
        let mixed_key = (hand.key.wrapping_mul(MIX_MULTIPLIER) & RANK_KEY_MASK) as usize;
        let offset = unsafe { *OFFSETS.get_unchecked(mixed_key >> OFFSET_SHIFT) } as usize;
        let hash_key = mixed_key.wrapping_add(offset);
        unsafe { *LOOKUP.get_unchecked(hash_key) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use card::*;
    use std::collections::{HashMap, HashSet};

    fn evaluate_hand_str(
        c1: &str,
        c2: &str,
        c3: &str,
        c4: &str,
        c5: &str,
        c6: &str,
        c7: &str,
    ) -> u16 {
        let cards = [
            c1.parse::<Card>().unwrap().id(),
            c2.parse::<Card>().unwrap().id(),
            c3.parse::<Card>().unwrap().id(),
            c4.parse::<Card>().unwrap().id(),
            c5.parse::<Card>().unwrap().id(),
            c6.parse::<Card>().unwrap().id(),
            c7.parse::<Card>().unwrap().id(),
        ];
        let mut hand = Hand::new();
        for c in &cards {
            hand = hand.add_card(*c);
        }
        evaluate_hand(hand)
    }

    #[test]
    fn test_lookup_table() {
        let mut rankset = HashSet::new();
        for rank in &LOOKUP {
            rankset.insert(rank);
        }
        for rank in &LOOKUP_FLUSH {
            rankset.insert(rank);
        }
        assert_eq!(rankset.len(), 4825); // 4824 + 1 (zero)
    }

    #[test]
    fn test_all_7card_combinations() {
        let mut rankset = HashSet::new();
        let mut counter = HashMap::new();

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
                                    let rank = evaluate_hand(hand);
                                    let category = get_hand_category(rank);
                                    rankset.insert(rank);
                                    let c = counter.entry(category).or_insert(0);
                                    *c += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(rankset.len(), 4824);
        assert_eq!(counter.get(&HandCategory::StraightFlush), Some(&41584));
        assert_eq!(counter.get(&HandCategory::FourOfAKind), Some(&224848));
        assert_eq!(counter.get(&HandCategory::FullHouse), Some(&3473184));
        assert_eq!(counter.get(&HandCategory::Flush), Some(&4047644));
        assert_eq!(counter.get(&HandCategory::Straight), Some(&6180020));
        assert_eq!(counter.get(&HandCategory::ThreeOfAKind), Some(&6461620));
        assert_eq!(counter.get(&HandCategory::TwoPair), Some(&31433400));
        assert_eq!(counter.get(&HandCategory::OnePair), Some(&58627800));
        assert_eq!(counter.get(&HandCategory::HighCard), Some(&23294460));
    }

    #[test]
    fn test_edge_cases() {
        // straight flushes
        assert_eq!(
            evaluate_hand_str("As", "Ks", "Qs", "Js", "Ts", "7d", "5s"),
            (8 << 12) + 9
        );
        assert_eq!(
            evaluate_hand_str("Ac", "7c", "6c", "5c", "4c", "3c", "2c"),
            (8 << 12) + 2
        );
        assert_eq!(
            evaluate_hand_str("Ad", "Qs", "Jc", "5d", "4d", "3d", "2d"),
            (8 << 12) + 0
        );

        // four of a kinds
        assert_eq!(
            evaluate_hand_str("As", "Ac", "Ah", "Ad", "Ks", "Qc", "Th"),
            (7 << 12) + 155
        );
        assert_eq!(
            evaluate_hand_str("3d", "3h", "3s", "2c", "2d", "2h", "2s"),
            (7 << 12) + 0
        );

        // full houses
        assert_eq!(
            evaluate_hand_str("As", "Ad", "Ah", "Kc", "Kd", "Kh", "2d"),
            (6 << 12) + 155
        );
        assert_eq!(
            evaluate_hand_str("4h", "4c", "3s", "3c", "2d", "2c", "2h"),
            (6 << 12) + 1
        );
        assert_eq!(
            evaluate_hand_str("5h", "4c", "3s", "3c", "2d", "2c", "2h"),
            (6 << 12) + 0
        );

        // flushes
        assert_eq!(
            evaluate_hand_str("Ah", "Kh", "Qh", "Jh", "9h", "9c", "9s"),
            (5 << 12) + 1276
        );
        assert_eq!(
            evaluate_hand_str("Js", "7c", "6d", "5c", "4c", "3c", "2c"),
            (5 << 12) + 0
        );

        // straights
        assert_eq!(
            evaluate_hand_str("Ah", "Kc", "Kd", "Kh", "Qc", "Jd", "Ts"),
            (4 << 12) + 9
        );
        assert_eq!(
            evaluate_hand_str("Ac", "8c", "7c", "5d", "4d", "3d", "2d"),
            (4 << 12) + 0
        );

        // three of a kinds
        assert_eq!(
            evaluate_hand_str("As", "Ac", "Ah", "Kh", "Qd", "5c", "3s"),
            (3 << 12) + 857
        );
        assert_eq!(
            evaluate_hand_str("7d", "5c", "4c", "3c", "2d", "2s", "2h"),
            (3 << 12) + 8
        );

        // two pairs
        assert_eq!(
            evaluate_hand_str("As", "Ah", "Ks", "Kh", "Qs", "Qh", "Js"),
            (2 << 12) + 857
        );
        assert_eq!(
            evaluate_hand_str("7c", "6d", "5h", "3s", "3c", "2d", "2h"),
            (2 << 12) + 3
        );

        // one pairs
        assert_eq!(
            evaluate_hand_str("Ad", "As", "Kh", "Qd", "Js", "3s", "2c"),
            (1 << 12) + 2859
        );
        assert_eq!(
            evaluate_hand_str("8s", "7s", "5h", "4c", "3c", "2d", "2c"),
            (1 << 12) + 18
        );

        // high cards
        assert_eq!(
            evaluate_hand_str("Ad", "Kd", "Qd", "Jd", "9s", "3h", "2c"),
            1276
        );
        assert_eq!(
            evaluate_hand_str("9h", "8s", "7d", "5d", "4d", "3c", "2d"),
            48
        );
    }
}
