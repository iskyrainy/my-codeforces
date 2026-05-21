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
    let round = read!(usize);
    for _ in 0..round {
        let (n, x, y, k) = (read!(usize), read!(usize), read!(usize), read!(usize));
        if n <= 3 {
            println!("1");
            continue;
        }
        let dis = x.abs_diff(y).min(n - x.abs_diff(y));
        println!("{}", dis + k);
    }
}
fn main() {
    solve();
}
