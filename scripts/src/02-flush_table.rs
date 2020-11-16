// generate a flush check table.

use assets::constants::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut result: Vec<i8> = vec![-2; 7 * SUIT_BASES[3] as usize + 1];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for m in 0..4 {
                    for n in 0..4 {
                        for p in 0..4 {
                            for q in 0..4 {
                                let a = SUIT_BASES[i] + SUIT_BASES[j] + SUIT_BASES[k];
                                let b = SUIT_BASES[m] + SUIT_BASES[n] + SUIT_BASES[p];
                                let x = a + b + SUIT_BASES[q];
                                let mut counter = [0; 4];
                                counter[i] += 1;
                                counter[j] += 1;
                                counter[k] += 1;
                                counter[m] += 1;
                                counter[n] += 1;
                                counter[p] += 1;
                                counter[q] += 1;
                                let val = match counter {
                                    [5..=7, _, _, _] => 0,
                                    [_, 5..=7, _, _] => 1,
                                    [_, _, 5..=7, _] => 2,
                                    [_, _, _, 5..=7] => 3,
                                    _ => -1,
                                };
                                match result[x as usize] {
                                    -2 => result[x as usize] = val,
                                    oldval => assert_eq!(oldval, val),
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    let mut file = File::create("assets/src/flush_table.rs").unwrap();
    writeln!(file, "/// flush checker:").unwrap();
    writeln!(
        file,
        "/// 0 => club, 1 => diamond, 2 => heart, 3 => spade, -1 => not a flush, -2 => error"
    )
    .unwrap();
    writeln!(
        file,
        "pub const FLUSH_TABLE: [i8; {}] = {:?};",
        result.len(),
        result
    )
    .unwrap();

    println!("wrote result to 'assets/src/flush_table.rs'");
}
