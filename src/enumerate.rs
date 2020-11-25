use crate::hand::*;
use assets::constants::*;

const NUM_HAND_CATEGORIES: usize = HandCategory::StraightFlush as usize + 1;

/// Enumerates possible hand categories from `hand`.
pub fn enumerate_hand_category(hand: &Hand, dead_cards: &Hand) -> [u32; NUM_HAND_CATEGORIES] {
    assert!(2 <= hand.len() && hand.len() <= 7);
    assert!((hand.mask & dead_cards.mask) == 0);
    let alive_cards = compute_alive_cards(hand.mask | dead_cards.mask);
    assert!(alive_cards.len() >= 7 - hand.len());
    match hand.len() {
        2 => enumerate_hand_category_2(hand, &alive_cards),
        3 => enumerate_hand_category_3(hand, &alive_cards),
        4 => enumerate_hand_category_4(hand, &alive_cards),
        5 => enumerate_hand_category_5(hand, &alive_cards),
        6 => enumerate_hand_category_6(hand, &alive_cards),
        7 => enumerate_hand_category_7(hand, &alive_cards),
        _ => unreachable!(),
    }
}

fn compute_alive_cards(mask: u64) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..NUMBER_OF_CARDS {
        if (CARDS[i].1 & mask) == 0 {
            result.push(i);
        }
    }
    result
}

fn enumerate_hand_category_2(hand: &Hand, alive_cards: &[usize]) -> [u32; NUM_HAND_CATEGORIES] {
    let len = alive_cards.len();
    let mut result = [0; NUM_HAND_CATEGORIES];
    for i in 0..(len - 4) {
        let hand = hand.add_card(alive_cards[i]);
        for j in (i + 1)..(len - 3) {
            let hand = hand.add_card(alive_cards[j]);
            for k in (j + 1)..(len - 2) {
                let hand = hand.add_card(alive_cards[k]);
                for m in (k + 1)..(len - 1) {
                    let hand = hand.add_card(alive_cards[m]);
                    for n in (m + 1)..len {
                        let hand = hand.add_card(alive_cards[n]);
                        result[get_hand_category(hand.evaluate()) as usize] += 1;
                    }
                }
            }
        }
    }
    result
}

fn enumerate_hand_category_3(hand: &Hand, alive_cards: &[usize]) -> [u32; NUM_HAND_CATEGORIES] {
    let len = alive_cards.len();
    let mut result = [0; NUM_HAND_CATEGORIES];
    for i in 0..(len - 3) {
        let hand = hand.add_card(alive_cards[i]);
        for j in (i + 1)..(len - 2) {
            let hand = hand.add_card(alive_cards[j]);
            for k in (j + 1)..(len - 1) {
                let hand = hand.add_card(alive_cards[k]);
                for m in (k + 1)..len {
                    let hand = hand.add_card(alive_cards[m]);
                    result[get_hand_category(hand.evaluate()) as usize] += 1;
                }
            }
        }
    }
    result
}

fn enumerate_hand_category_4(hand: &Hand, alive_cards: &[usize]) -> [u32; NUM_HAND_CATEGORIES] {
    let len = alive_cards.len();
    let mut result = [0; NUM_HAND_CATEGORIES];
    for i in 0..(len - 2) {
        let hand = hand.add_card(alive_cards[i]);
        for j in (i + 1)..(len - 1) {
            let hand = hand.add_card(alive_cards[j]);
            for k in (j + 1)..len {
                let hand = hand.add_card(alive_cards[k]);
                result[get_hand_category(hand.evaluate()) as usize] += 1;
            }
        }
    }
    result
}

fn enumerate_hand_category_5(hand: &Hand, alive_cards: &[usize]) -> [u32; NUM_HAND_CATEGORIES] {
    let len = alive_cards.len();
    let mut result = [0; NUM_HAND_CATEGORIES];
    for i in 0..(len - 1) {
        let hand = hand.add_card(alive_cards[i]);
        for j in (i + 1)..len {
            let hand = hand.add_card(alive_cards[j]);
            result[get_hand_category(hand.evaluate()) as usize] += 1;
        }
    }
    result
}

fn enumerate_hand_category_6(hand: &Hand, alive_cards: &[usize]) -> [u32; NUM_HAND_CATEGORIES] {
    let len = alive_cards.len();
    let mut result = [0; NUM_HAND_CATEGORIES];
    for i in 0..len {
        let hand = hand.add_card(alive_cards[i]);
        result[get_hand_category(hand.evaluate()) as usize] += 1;
    }
    result
}

fn enumerate_hand_category_7(hand: &Hand, _: &[usize]) -> [u32; NUM_HAND_CATEGORIES] {
    let mut result = [0; NUM_HAND_CATEGORIES];
    result[get_hand_category(hand.evaluate()) as usize] += 1;
    result
}
