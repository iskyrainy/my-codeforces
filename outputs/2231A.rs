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
        let n = read!(usize);
        for i in 0..n {
            if i > 0 {
                print!(" ");
            }
            print!("{}", n + i);
        }
        println!();
    }
}
fn main() {
    solve();
}
