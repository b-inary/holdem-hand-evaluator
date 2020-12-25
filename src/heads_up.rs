use crate::hand::*;
use assets::constants::*;
use assets::heads_up::HEADS_UP_WIN_FREQUENCY;

/// Computes heads-up win frequency.
/// Return value: (# of `hand1` wins, # of `hand2` wins, # of tie)
pub fn heads_up_win_frequency(
    hand1: &Hand,
    hand2: &Hand,
    board: &Hand,
    dead_cards: &Hand,
) -> (u32, u32, u32) {
    assert_eq!(hand1.len(), 2);
    assert!(hand2.len() <= 2);
    assert!(board.len() == 0 || board.len() == 3 || board.len() == 4 || board.len() == 5);
    assert_eq!(
        (*hand1 + *hand2 + *board + *dead_cards).len(),
        hand1.len() + hand2.len() + board.len() + dead_cards.len()
    );
    let alive_cards = compute_alive_cards(
        hand1.get_mask() | hand2.get_mask() | board.get_mask() | dead_cards.get_mask(),
    );
    assert!(alive_cards.len() >= 5 - board.len());
    let hand1 = *hand1 + *board;
    let hand2 = *hand2 + *board;
    match (hand2.len() - board.len(), board.len()) {
        (0, 0) => match dead_cards.len() {
            0 => heads_up_win_freq_0_0(&hand1),
            _ => heads_up_win_freq_0(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_0),
        },
        (0, 3) => heads_up_win_freq_0(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_3),
        (0, 4) => heads_up_win_freq_0(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_4),
        (0, 5) => heads_up_win_freq_0(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_5),
        (1, 0) => heads_up_win_freq_1(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_0),
        (1, 3) => heads_up_win_freq_1(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_3),
        (1, 4) => heads_up_win_freq_1(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_4),
        (1, 5) => heads_up_win_freq_1(&hand1, &hand2, &alive_cards, heads_up_win_freq_2_5),
        (2, 0) => heads_up_win_freq_2_0(&hand1, &hand2, &alive_cards),
        (2, 3) => heads_up_win_freq_2_3(&hand1, &hand2, &alive_cards),
        (2, 4) => heads_up_win_freq_2_4(&hand1, &hand2, &alive_cards),
        (2, 5) => heads_up_win_freq_2_5(&hand1, &hand2, &alive_cards),
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

fn heads_up_win_freq_0_0(hand: &Hand) -> (u32, u32, u32) {
    let mut cards = Vec::new();
    for i in 0..NUMBER_OF_CARDS {
        if (CARDS[i].1 & hand.get_mask()) != 0 {
            cards.push(i);
        }
    }
    let rank1 = cards[0] / 4;
    let suit1 = cards[0] % 4;
    let rank2 = cards[1] / 4;
    let suit2 = cards[1] % 4;
    if suit1 == suit2 {
        HEADS_UP_WIN_FREQUENCY[rank1 * 13 + rank2]
    } else {
        HEADS_UP_WIN_FREQUENCY[rank2 * 13 + rank1]
    }
}

fn heads_up_win_freq_0(
    hand1: &Hand,
    hand2: &Hand,
    alive_cards: &[usize],
    func: fn(&Hand, &Hand, &[usize]) -> (u32, u32, u32),
) -> (u32, u32, u32) {
    let len = alive_cards.len();
    let mut result = (0, 0, 0);
    for i in 0..(len - 1) {
        let hand2 = hand2.add_card(alive_cards[i]);
        for j in (i + 1)..len {
            let hand2 = hand2.add_card(alive_cards[j]);
            let alive_cards = alive_cards
                .iter()
                .enumerate()
                .filter_map(|(idx, x)| match idx {
                    _ if idx == i || idx == j => None,
                    _ => Some(*x),
                })
                .collect::<Vec<usize>>();
            let tmp = func(hand1, &hand2, &alive_cards);
            result.0 += tmp.0;
            result.1 += tmp.1;
            result.2 += tmp.2;
        }
    }
    result
}

fn heads_up_win_freq_1(
    hand1: &Hand,
    hand2: &Hand,
    alive_cards: &[usize],
    func: fn(&Hand, &Hand, &[usize]) -> (u32, u32, u32),
) -> (u32, u32, u32) {
    let len = alive_cards.len();
    let mut result = (0, 0, 0);
    for i in 0..len {
        let hand2 = hand2.add_card(alive_cards[i]);
        let alive_cards = alive_cards
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| match idx {
                _ if idx == i => None,
                _ => Some(*x),
            })
            .collect::<Vec<usize>>();
        let tmp = func(hand1, &hand2, &alive_cards);
        result.0 += tmp.0;
        result.1 += tmp.1;
        result.2 += tmp.2;
    }
    result
}

fn heads_up_win_freq_2_0(hand1: &Hand, hand2: &Hand, alive_cards: &[usize]) -> (u32, u32, u32) {
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
    count
}

fn heads_up_win_freq_2_3(hand1: &Hand, hand2: &Hand, alive_cards: &[usize]) -> (u32, u32, u32) {
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
    count
}

fn heads_up_win_freq_2_4(hand1: &Hand, hand2: &Hand, alive_cards: &[usize]) -> (u32, u32, u32) {
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
    count
}

fn heads_up_win_freq_2_5(hand1: &Hand, hand2: &Hand, _: &[usize]) -> (u32, u32, u32) {
    let rank1 = hand1.evaluate();
    let rank2 = hand2.evaluate();
    if rank1 > rank2 {
        (1, 0, 0)
    } else if rank1 < rank2 {
        (0, 1, 0)
    } else {
        (0, 0, 1)
    }
}
