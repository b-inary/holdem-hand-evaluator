#[cfg(test)]
mod tests {
    use crate::kev::eval_5cards;

    fn eval_5cards_naive(c1: usize, c2: usize, c3: usize, c4: usize, c5: usize) -> u32 {
        let cards = [c1, c2, c3, c4, c5];

        let mut suitset: u32 = 0;
        let mut rankset: u32 = 0;
        let mut rankset_of_count: [u32; 5] = [0; 5];
        let mut rankcount: [usize; 13] = [0; 13];

        for card in &cards {
            let suit = *card % 4;
            let rank = *card / 4;
            suitset |= 1 << suit;
            rankset |= 1 << rank;
            rankcount[rank] += 1;
        }

        for rank in 0..13 {
            rankset_of_count[rankcount[rank]] |= 1 << rank;
        }

        let is_flush = suitset.count_ones() == 1;
        let is_straight = match rankset {
            0b1_0000_0000_1111 => 1 << 3,
            _ => rankset & (rankset << 1) & (rankset << 2) & (rankset << 3) & (rankset << 4),
        };

        if is_flush && is_straight > 0 {
            // straight flush
            (8 << 26) | is_straight
        } else if rankset_of_count[4] > 0 {
            // four of a kind
            (7 << 26) | (rankset_of_count[4] << 13) | rankset_of_count[1]
        } else if rankset_of_count[3] > 0 && rankset_of_count[2] > 0 {
            // full house
            (6 << 26) | (rankset_of_count[3] << 13) | rankset_of_count[2]
        } else if is_flush {
            // flush
            (5 << 26) | rankset
        } else if is_straight > 0 {
            // straight
            (4 << 26) | is_straight
        } else if rankset_of_count[3] > 0 {
            // three of a kind
            (3 << 26) | (rankset_of_count[3] << 13) | rankset_of_count[1]
        } else if rankset_of_count[2] > 0 {
            // two pair or one pair
            let num_pairs = rankset_of_count[2].count_ones();
            (num_pairs << 26) | (rankset_of_count[2] << 13) | rankset_of_count[1]
        } else {
            // high card
            (0 << 26) | rankset
        }
    }

    #[test]
    fn test_naive() {
        let mut table = vec![0; 7463];
        for i in 0..48 {
            for j in (i + 1)..49 {
                for k in (j + 1)..50 {
                    for m in (k + 1)..51 {
                        for n in (m + 1)..52 {
                            let rank_kev = eval_5cards(i, j, k, m, n);
                            let rank_naive = eval_5cards_naive(i, j, k, m, n);
                            if table[rank_kev as usize] == 0 {
                                table[rank_kev as usize] = rank_naive;
                            } else {
                                assert_eq!(table[rank_kev as usize], rank_naive);
                            }
                        }
                    }
                }
            }
        }
        for i in 1..7462 {
            assert!(table[i] > table[i + 1]);
        }
    }
}
