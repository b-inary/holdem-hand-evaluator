// generate an offset table for perfect hash function with the following form:
//   mixed_key = (input_value * HASH_MULTIPLIER) & KEY_MASK;
//   hash_key = mixed_key + OFFSETS[mixed_key >> OFFSET_SHIFT];

// reference: Z. J. Czech, G. Havas, and B. S. Majewski. "Perfect hashing".
//            Theoretical Computer Science, 182(1-2), 1-143. 1997. (Section 5.2)

use assets::constants::*;
use std::cmp;
use std::fs::File;
use std::io::{stdout, Write};

#[derive(Clone, Debug)]
struct Row {
    cols: Vec<u64>,
    idx: usize,
}

fn main() {
    println!(
        "size of offset table: {}",
        1 << (RANK_KEY_BITS - OFFSET_SHIFT)
    );

    let mut keys = Vec::new();
    for i in 0..(NUMBER_OF_RANKS - 1) {
        for j in i..(NUMBER_OF_RANKS - 1) {
            for k in j..(NUMBER_OF_RANKS - 1) {
                for m in k..NUMBER_OF_RANKS {
                    for n in cmp::max(m, i + 1)..NUMBER_OF_RANKS {
                        for p in cmp::max(n, j + 1)..NUMBER_OF_RANKS {
                            for q in cmp::max(p, k + 1)..NUMBER_OF_RANKS {
                                let a = RANK_BASES[i] + RANK_BASES[j] + RANK_BASES[k];
                                let b = RANK_BASES[m] + RANK_BASES[n] + RANK_BASES[p];
                                keys.push(a + b + RANK_BASES[q]);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut best_size = usize::MAX;
    let mut best_mult = 0;
    let mut best_offsets = Vec::new();

    for mult in (1..1000).step_by(2) {
        let mut rows: Vec<Row> = vec![
            Row {
                cols: Vec::new(),
                idx: 0
            };
            1 << (RANK_KEY_BITS - OFFSET_SHIFT)
        ];

        for (i, row) in rows.iter_mut().enumerate() {
            row.idx = i;
        }

        for key in &keys {
            let x = key.wrapping_mul(mult) & RANK_KEY_MASK;
            let row = x >> OFFSET_SHIFT;
            let col = x & (1 << OFFSET_SHIFT) - 1;
            rows[row as usize].cols.push(col);
        }

        for row in &mut rows {
            row.cols.sort();
        }

        // apply first-fit-decreasing method
        rows.sort_by_key(|row| row.cols.len());
        rows.reverse();

        let mut least_empty = 0;
        let mut filled = vec![false; 1 << RANK_KEY_BITS];
        let mut offsets = vec![0; rows.len()];

        for Row { cols, idx } in &rows {
            if cols.is_empty() {
                break;
            }
            let mut offset = least_empty as i32 - cols[0] as i32;
            'search: for i in offset.. {
                for col in cols {
                    if filled[col.wrapping_add(i as u64) as usize] {
                        continue 'search;
                    }
                }
                offset = i;
                break;
            }
            offsets[*idx] = offset;
            for col in cols {
                filled[col.wrapping_add(offset as u64) as usize] = true;
            }
            while filled[least_empty] {
                least_empty += 1;
            }
        }

        let table_size = filled.iter().rposition(|&b| b).unwrap() + 1;

        if table_size < best_size {
            best_size = table_size;
            best_mult = mult;
            best_offsets = offsets;
            print!(
                "\rcurrent size of hash table: {} (mult = {})    \x08\x08\x08\x08",
                table_size, mult
            );
            stdout().flush().unwrap();
        }

        // best compression acheived
        if best_size == keys.len() {
            break;
        }
    }

    for (i, offset) in best_offsets.iter_mut().enumerate() {
        *offset -= (i << OFFSET_SHIFT) as i32;
    }

    let mut file = File::create("assets/src/offsets.rs").unwrap();
    writeln!(file, "pub const MIX_MULTIPLIER: u64 = {};", best_mult).unwrap();
    writeln!(
        file,
        "pub const OFFSETS: [i32; {}] = {:?};",
        best_offsets.len(),
        best_offsets
    )
    .unwrap();

    println!();
    println!("wrote result to 'assets/src/offsets.rs'");
}
