use crate::hand::*;
use assets::constants::*;

const NUM_HAND_CATEGORIES: usize = HandCategory::StraightFlush as usize + 1;

/// Computes heads-up win rate.
pub fn heads_up_win_rate(
    hand1: &Hand,
    hand2: &Hand,
    board: &Hand,
    dead_cards: &Hand,
) -> (f64, f64) {
    assert_eq!(hand1.len(), 2);
    assert_eq!(hand2.len(), 2);
    assert!(board.len() == 0 || board.len() == 3 || board.len() == 4 || board.len() == 5);
    assert_eq!(
        (*hand1 + *hand2 + *board + *dead_cards).len(),
        hand1.len() + hand2.len() + board.len() + dead_cards.len()
    );
    let mut alive_cards = Vec::new();
    for i in 0..NUMBER_OF_CARDS {
        if (CARDS[i].1 & (hand1.mask | hand2.mask | board.mask | dead_cards.mask)) == 0 {
            alive_cards.push(i);
        }
    }
    assert!(alive_cards.len() >= 5 - board.len());
    let hand1 = *hand1 + *board;
    let hand2 = *hand2 + *board;
    match board.len() {
        0 => heads_up_win_rate_0(&hand1, &hand2, &alive_cards),
        3 => heads_up_win_rate_3(&hand1, &hand2, &alive_cards),
        4 => heads_up_win_rate_4(&hand1, &hand2, &alive_cards),
        5 => heads_up_win_rate_5(&hand1, &hand2, &alive_cards),
        _ => unreachable!(),
    }
}

/// Enumerates possible hand categories from `hand`.
pub fn enumerate_hand_category(hand: &Hand, dead_cards: &Hand) -> [u32; NUM_HAND_CATEGORIES] {
    assert!(2 <= hand.len() && hand.len() <= 7);
    assert!((hand.mask & dead_cards.mask) == 0);
    let mut alive_cards = Vec::new();
    for i in 0..NUMBER_OF_CARDS {
        if (CARDS[i].1 & (hand.mask | dead_cards.mask)) == 0 {
            alive_cards.push(i);
        }
    }
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

fn heads_up_win_rate_0(hand1: &Hand, hand2: &Hand, alive_cards: &[usize]) -> (f64, f64) {
    let len = alive_cards.len();
    let mut count = (0, 0, 0);
    for i in 0..(len - 4) {
        let hand1 = hand1.add_card(alive_cards[i]);
        let hand2 = hand2.add_card(alive_cards[i]);
        for j in (i + 1)..(len - 3) {
            let hand1 = hand1.add_card(alive_cards[j]);
            let hand2 = hand2.add_card(alive_cards[j]);
            for k in (j + 1)..(len - 2) {
                let hand1 = hand1.add_card(alive_cards[k]);
                let hand2 = hand2.add_card(alive_cards[k]);
                for m in (k + 1)..(len - 1) {
                    let hand1 = hand1.add_card(alive_cards[m]);
                    let hand2 = hand2.add_card(alive_cards[m]);
                    for n in (m + 1)..len {
                        let hand1 = hand1.add_card(alive_cards[n]);
                        let hand2 = hand2.add_card(alive_cards[n]);
                        let rank1 = hand1.evaluate();
                        let rank2 = hand2.evaluate();
                        if rank1 > rank2 {
                            count.0 += 1;
                        } else if rank1 < rank2 {
                            count.1 += 1;
                        } else {
                            count.2 += 1;
                        }
                    }
                }
            }
        }
    }
    let sum = (count.0 + count.1 + count.2) as f64;
    (count.0 as f64 / sum, count.1 as f64 / sum)
}

fn heads_up_win_rate_3(hand1: &Hand, hand2: &Hand, alive_cards: &[usize]) -> (f64, f64) {
    let len = alive_cards.len();
    let mut count = (0, 0, 0);
    for i in 0..(len - 1) {
        let hand1 = hand1.add_card(alive_cards[i]);
        let hand2 = hand2.add_card(alive_cards[i]);
        for j in (i + 1)..len {
            let hand1 = hand1.add_card(alive_cards[j]);
            let hand2 = hand2.add_card(alive_cards[j]);
            let rank1 = hand1.evaluate();
            let rank2 = hand2.evaluate();
            if rank1 > rank2 {
                count.0 += 1;
            } else if rank1 < rank2 {
                count.1 += 1;
            } else {
                count.2 += 1;
            }
        }
    }
    let sum = (count.0 + count.1 + count.2) as f64;
    (count.0 as f64 / sum, count.1 as f64 / sum)
}

fn heads_up_win_rate_4(hand1: &Hand, hand2: &Hand, alive_cards: &[usize]) -> (f64, f64) {
    let len = alive_cards.len();
    let mut count = (0, 0, 0);
    for i in 0..len {
        let hand1 = hand1.add_card(alive_cards[i]);
        let hand2 = hand2.add_card(alive_cards[i]);
        let rank1 = hand1.evaluate();
        let rank2 = hand2.evaluate();
        if rank1 > rank2 {
            count.0 += 1;
        } else if rank1 < rank2 {
            count.1 += 1;
        } else {
            count.2 += 1;
        }
    }
    let sum = (count.0 + count.1 + count.2) as f64;
    (count.0 as f64 / sum, count.1 as f64 / sum)
}

fn heads_up_win_rate_5(hand1: &Hand, hand2: &Hand, _: &[usize]) -> (f64, f64) {
    let rank1 = hand1.evaluate();
    let rank2 = hand2.evaluate();
    if rank1 > rank2 {
        (1.0, 0.0)
    } else if rank1 < rank2 {
        (0.0, 1.0)
    } else {
        (0.0, 0.0)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!(
            "{:?}",
            heads_up_win_rate(
                &"7h7s".parse().unwrap(),
                &"AdKd".parse().unwrap(),
                &"6sQd9d".parse().unwrap(),
                &Hand::new()
            )
        )
    }
}
