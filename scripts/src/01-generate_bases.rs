// find rank values greedily so that every 7-cards combination has a unique sum.
// expected result: [0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181]

use assets::constants::NUMBER_OF_RANKS;
use std::cmp;
use std::collections::HashSet;

fn main() {
    let mut ans = vec![0];
    while ans.len() < NUMBER_OF_RANKS {
        ans.push(ans[ans.len() - 1]);
        let r = ans.len();
        loop {
            let mut ok = true;
            let mut h = HashSet::new();
            'outer: for i in 0..(r - 1) {
                for j in i..(r - 1) {
                    for k in j..(r - 1) {
                        for m in k..r {
                            for n in cmp::max(m, i + 1)..r {
                                for p in cmp::max(n, j + 1)..r {
                                    for q in cmp::max(p, k + 1)..r {
                                        let a = ans[i] + ans[j] + ans[k] + ans[m];
                                        let b = ans[n] + ans[p] + ans[q];
                                        let x = a + b;
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
