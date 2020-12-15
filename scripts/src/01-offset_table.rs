// generate an offset table for perfect hash function with the following form:
//   hash_key = input_value + OFFSETS[input_value >> OFFSET_SHIFT];

// reference: Z. J. Czech, G. Havas, and B. S. Majewski. "Perfect hashing".
//            Theoretical Computer Science, 182(1-2), 1-143. 1997. (Section 5.2)

use assets::constants::*;
use std::cmp::max;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug)]
struct Row {
    cols: Vec<u64>,
    idx: usize,
}

fn main() {
    let offset_table_len = ((MAX_RANK_KEY >> OFFSET_SHIFT) + 1) as usize;
    println!("size of offset table: {}", offset_table_len);

    let mut keys = Vec::new();
    for i in 0..(NUMBER_OF_RANKS - 1) {
        for j in i..NUMBER_OF_RANKS {
            for k in j..NUMBER_OF_RANKS {
                for m in k..NUMBER_OF_RANKS {
                    for n in max(m, i + 1)..NUMBER_OF_RANKS {
                        let x = RANK_BASES[i] + RANK_BASES[j] + RANK_BASES[k];
                        let x = x + RANK_BASES[m] + RANK_BASES[n];
                        keys.push(x);
                        for p in max(n, j + 1)..NUMBER_OF_RANKS {
                            let x = x + RANK_BASES[p];
                            keys.push(x);
                            for q in max(p, k + 1)..NUMBER_OF_RANKS {
                                let x = x + RANK_BASES[q];
                                keys.push(x);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("number of elements: {}", keys.len());

    let mut rows: Vec<Row> = vec![
        Row {
            cols: Vec::new(),
            idx: 0
        };
        offset_table_len
    ];

    for (i, row) in rows.iter_mut().enumerate() {
        row.idx = i;
    }

    for key in &keys {
        let row = key >> OFFSET_SHIFT;
        let col = key & ((1 << OFFSET_SHIFT) - 1);
        rows[row as usize].cols.push(col);
    }

    for row in &mut rows {
        row.cols.sort_unstable();
    }

    // apply first-fit-decreasing method
    rows.sort_unstable_by_key(|row| row.cols.len());
    rows.reverse();

    let mut least_empty = 0;
    let mut filled = vec![false; offset_table_len << OFFSET_SHIFT];
    let mut offsets = vec![i32::MIN; rows.len()];

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

    for (i, offset) in offsets.iter_mut().enumerate() {
        *offset = match *offset {
            i32::MIN => 0,
            _ => *offset - (i << OFFSET_SHIFT) as i32,
        }
    }

    let image_size = filled.iter().rposition(|&b| b).unwrap() + 1;
    println!("image size: {}", image_size);

    let mut file = File::create("assets/src/offsets.rs").unwrap();
    writeln!(
        file,
        "pub const OFFSETS: [i32; {}] = {:?};",
        offsets.len(),
        offsets
    )
    .unwrap();

    println!("wrote result to 'assets/src/offsets.rs'");
}
