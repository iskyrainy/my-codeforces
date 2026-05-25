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
        let num = read!(usize);
        let (mut max, mut min) = (1, 1000);
        for _ in 0..num {
            let n = read!(i32);
            if n > max {
                max = n;
            }
            if n < min {
                min = n;
            }
        }
        println!(
            "{}",
            if (max - min) % 2 == 0 {
                (max - min) / 2
            } else {
                (max - min) / 2 + 1
            }
        );
    }
}
fn main() {
    solve();
}
