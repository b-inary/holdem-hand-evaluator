// generate a flush check table.

use assets::constants::*;
use std::fs::File;
use std::io::Write;

fn update(x: u64, counter: &[i32; 4], result: &mut Vec<i8>) {
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

fn main() {
    let mut counter = [0; 4];
    let mut result: Vec<i8> = vec![-2; 7 * SUIT_BASES[3] as usize + 1];
    for i in 0..4 {
        counter[i] += 1;
        for j in 0..4 {
            counter[j] += 1;
            for k in 0..4 {
                counter[k] += 1;
                for m in 0..4 {
                    counter[m] += 1;
                    for n in 0..4 {
                        counter[n] += 1;
                        let x = SUIT_BASES[i] + SUIT_BASES[j] + SUIT_BASES[k];
                        let x = x + SUIT_BASES[m] + SUIT_BASES[n];
                        update(x, &counter, &mut result);
                        for p in 0..4 {
                            counter[p] += 1;
                            let x = x + SUIT_BASES[p];
                            update(x, &counter, &mut result);
                            for q in 0..4 {
                                counter[q] += 1;
                                let x = x + SUIT_BASES[q];
                                update(x, &counter, &mut result);
                                counter[q] -= 1;
                            }
                            counter[p] -= 1;
                        }
                        counter[n] -= 1;
                    }
                    counter[m] -= 1;
                }
                counter[k] -= 1;
            }
            counter[j] -= 1;
        }
        counter[i] -= 1;
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
