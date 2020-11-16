#[cfg(test)]
mod tests {
    use crate::Hand;
    use assets::constants::NUMBER_OF_CARDS;

    fn msb(x: u32) -> u32 {
        1 << (x.leading_zeros() ^ 31)
    }

    fn keep_n_msb(x: u32, n: usize) -> u32 {
        let mut x = x;
        let mut result = 0;
        for _ in 0..n {
            let m = msb(x);
            x ^= m;
            result |= m;
        }
        result
    }

    fn find_straight(rankset: u32) -> Option<u32> {
        let wheel = 0b1_0000_0000_1111;
        match rankset & (rankset << 1) & (rankset << 2) & (rankset << 3) & (rankset << 4) {
            0 => {
                if (rankset & wheel) == wheel {
                    Some(1 << 3)
                } else {
                    None
                }
            }
            x => Some(keep_n_msb(x, 1)),
        }
    }

    fn evaluate_hand_naive(hand: &[usize]) -> u32 {
        let mut rankset: u32 = 0;
        let mut rankset_suit: [u32; 4] = [0; 4];
        let mut rankset_of_count: [u32; 5] = [0; 5];
        let mut count: [usize; 13] = [0; 13];

        for card in hand {
            let suit = *card % 4;
            let rank = *card / 4;
            rankset |= 1 << rank;
            rankset_suit[suit] |= 1 << rank;
            count[rank] += 1;
        }

        for rank in 0..13 {
            rankset_of_count[count[rank]] |= 1 << rank;
        }

        let mut is_flush = -1;
        for i in 0..4 {
            if rankset_suit[i as usize].count_ones() >= 5 {
                is_flush = i;
            }
        }

        if is_flush >= 0 {
            match find_straight(rankset_suit[is_flush as usize]) {
                // straight flush
                Some(x) => (8 << 26) | x,
                // flush
                None => (5 << 26) | keep_n_msb(rankset_suit[is_flush as usize], 5),
            }
        } else if rankset_of_count[4] > 0 {
            // four of a kind
            let remaining = keep_n_msb(rankset ^ rankset_of_count[4], 1);
            (7 << 26) | (rankset_of_count[4] << 13) | remaining
        } else if rankset_of_count[3].count_ones() == 2 {
            // full house
            let trips = keep_n_msb(rankset_of_count[3], 1);
            let pair = rankset_of_count[3] ^ trips;
            (6 << 26) | (trips << 13) | pair
        } else if rankset_of_count[3] > 0 && rankset_of_count[2] > 0 {
            // full house
            let pair = keep_n_msb(rankset_of_count[2], 1);
            (6 << 26) | (rankset_of_count[3] << 13) | pair
        } else if let Some(x) = find_straight(rankset) {
            // straight
            (4 << 26) | x
        } else if rankset_of_count[3] > 0 {
            // three of a kind
            let remaining = keep_n_msb(rankset_of_count[1], 2);
            (3 << 26) | (rankset_of_count[3] << 13) | remaining
        } else if rankset_of_count[2].count_ones() >= 2 {
            // two pair
            let pairs = keep_n_msb(rankset_of_count[2], 2);
            let remaining = keep_n_msb(rankset ^ pairs, 1);
            (2 << 26) | (pairs << 13) | remaining
        } else if rankset_of_count[2] > 0 {
            // one pair
            let remaining = keep_n_msb(rankset_of_count[1], 3);
            (1 << 26) | (rankset_of_count[2] << 13) | remaining
        } else {
            // high card
            (0 << 26) | keep_n_msb(rankset, 5)
        }
    }

    #[test]
    fn test_naive() {
        let mut table = vec![0; 32780];
        let mut hand_array = [0; 7];
        for i in 0..(NUMBER_OF_CARDS - 6) {
            hand_array[0] = i;
            let hand = Hand::new().add_card(i);
            for j in (i + 1)..(NUMBER_OF_CARDS - 5) {
                hand_array[1] = j;
                let hand = hand.add_card(j);
                for k in (j + 1)..(NUMBER_OF_CARDS - 4) {
                    hand_array[2] = k;
                    let hand = hand.add_card(k);
                    for m in (k + 1)..(NUMBER_OF_CARDS - 3) {
                        hand_array[3] = m;
                        let hand = hand.add_card(m);
                        for n in (m + 1)..(NUMBER_OF_CARDS - 2) {
                            hand_array[4] = n;
                            let hand = hand.add_card(n);
                            for p in (n + 1)..(NUMBER_OF_CARDS - 1) {
                                hand_array[5] = p;
                                let hand = hand.add_card(p);
                                for q in (p + 1)..NUMBER_OF_CARDS {
                                    hand_array[6] = q;
                                    let rank_naive = evaluate_hand_naive(&hand_array);
                                    let hand = hand.add_card(q);
                                    let rank = hand.evaluate();
                                    if table[rank as usize] == 0 {
                                        table[rank as usize] = rank_naive;
                                    } else {
                                        assert_eq!(table[rank as usize], rank_naive);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut prev_rank = 0;
        for rank_naive in &table {
            if *rank_naive > 0 {
                assert!(prev_rank < *rank_naive);
                prev_rank = *rank_naive;
            }
        }
    }
}
