#[cfg(test)]
fn eval_5cards_reference(c1: usize, c2: usize, c3: usize, c4: usize, c5: usize) -> u32 {
    let cards = [c1, c2, c3, c4, c5];

    let mut suit_bitset: u32 = 0;
    let mut rank_bitset: u32 = 0;
    let mut rank_count: [usize; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut count_to_ranks: [u32; 5] = [0, 0, 0, 0, 0];

    for card in &cards {
        let suit = *card % 4;
        let rank = *card / 4;
        suit_bitset |= 1 << suit;
        rank_bitset |= 1 << rank;
        rank_count[rank] += 1;
    }

    for rank in 0..13 {
        count_to_ranks[rank_count[rank]] |= 1 << rank;
    }

    let is_flush = suit_bitset.count_ones() == 1;
    let is_straight = if rank_bitset == 0b1_0000_0000_1111 {
        1 << 3
    } else {
        rank_bitset
            & (rank_bitset << 1)
            & (rank_bitset << 2)
            & (rank_bitset << 3)
            & (rank_bitset << 4)
    };

    if is_flush && is_straight > 0 {
        (8 << 26) | is_straight
    } else if count_to_ranks[4] > 0 {
        (7 << 26) | (count_to_ranks[4] << 13) | count_to_ranks[1]
    } else if count_to_ranks[3] > 0 && count_to_ranks[2] > 0 {
        (6 << 26) | (count_to_ranks[3] << 13) | count_to_ranks[2]
    } else if is_flush {
        (5 << 26) | rank_bitset
    } else if is_straight > 0 {
        (4 << 26) | is_straight
    } else if count_to_ranks[3] > 0 {
        (3 << 26) | (count_to_ranks[3] << 13) | count_to_ranks[1]
    } else if count_to_ranks[2] > 0 {
        let num_pairs = count_to_ranks[2].count_ones();
        (num_pairs << 26) | (count_to_ranks[2] << 13) | count_to_ranks[1]
    } else {
        (0 << 26) | rank_bitset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kev::evaluator::eval_5cards;

    #[test]
    fn test() {
        let mut table = vec![0; 7463];
        for i in 0..48 {
            for j in (i + 1)..49 {
                for k in (j + 1)..50 {
                    for m in (k + 1)..51 {
                        for n in (m + 1)..52 {
                            let rank_kev = eval_5cards(i, j, k, m, n);
                            let rank_ref = eval_5cards_reference(i, j, k, m, n);
                            if table[rank_kev as usize] == 0 {
                                table[rank_kev as usize] = rank_ref;
                            } else {
                                assert_eq!(table[rank_kev as usize], rank_ref);
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
