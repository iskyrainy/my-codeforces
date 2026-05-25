use std::io::{self, BufRead};
fn solve() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().flat_map(|l| {
        l.unwrap()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>()
    });
    macro_rules! read {
        ($ t : ty) => {
            iter.next().unwrap().parse::<$t>().unwrap()
        };
    }
    let t: usize = read!(usize);
    for _ in 0..t {
        let n: usize = read!(usize);
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = read!(i64);
        }
        let mut need_add = vec![false; n];
        let mut max_so_far = a[0];
        let mut k = 0;
        for i in 1..n {
            if a[i] < max_so_far {
                need_add[i] = true;
                let required = max_so_far - a[i];
                k = k.max(required);
            } else {
                max_so_far = max_so_far.max(a[i]);
            }
        }
        let mut valid = true;
        let mut prev = a[0];
        for i in 1..n {
            let curr = if need_add[i] { a[i] + k } else { a[i] };
            if curr < prev {
                valid = false;
                break;
            }
            prev = curr;
        }
        println!("{}", if valid { "YES" } else { "NO" });
    }
}
fn main() {
    solve();
}
