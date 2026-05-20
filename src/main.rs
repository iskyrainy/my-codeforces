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
        ($t:ty) => {
            iter.next().unwrap().parse::<$t>().unwrap()
        };
    }

    todo!()
}

fn main() {
    solve();
}
