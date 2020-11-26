// find rank values greedily so that every 5-7 cards combination has a unique sum.
// expected result: [1, 4, 16, 67, 295, 1334, 5734, 23800, 60883, 208450, 509982, 1304151, 2967844]

use assets::constants::NUMBER_OF_RANKS;
use std::cmp;
use std::collections::HashSet;

fn main() {
    let mut ans = vec![1];
    while ans.len() < NUMBER_OF_RANKS {
        ans.push(ans[ans.len() - 1] + 1);
        let r = ans.len();
        loop {
            let mut ok = true;
            let mut h = HashSet::new();
            'outer: for i in 0..(r - 1) {
                for j in i..r {
                    for k in j..r {
                        for m in k..r {
                            for n in cmp::max(m, i + 1)..r {
                                let x = ans[i] + ans[j] + ans[k] + ans[m] + ans[n];
                                if h.contains(&x) {
                                    ok = false;
                                    break 'outer;
                                }
                                h.insert(x);

                                for p in cmp::max(n, j + 1)..r {
                                    let x = x + ans[p];
                                    if h.contains(&x) {
                                        ok = false;
                                        break 'outer;
                                    }
                                    h.insert(x);

                                    for q in cmp::max(p, k + 1)..r {
                                        let x = x + ans[q];
                                        if h.contains(&x) {
                                            ok = false;
                                            break 'outer;
                                        }
                                        h.insert(x);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if ok {
                println!("index {}: {}", r - 1, ans[r - 1]);
                break;
            }
            ans[r - 1] += 1;
        }
    }
    println!("result: {:?}", ans);
}
