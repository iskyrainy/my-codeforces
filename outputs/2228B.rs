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
        let len = read!(usize);
        let mut w = vec![];
        for _ in 0..len {
            w.push(read!(usize));
        }
        let (mut n0, mut n1, mut n2) = (0, 0, 0);
        for num in w {
            match num {
                0 => n0 += 1,
                1 => n1 += 1,
                _ => n2 += 1,
            }
        }
        if n1 > n2 {
            println!("{}", n0 + n2 + (n1 - n2) / 3);
        } else {
            println!("{}", n0 + n1 + (n2 - n1) / 3);
        }
    }
}
fn main() {
    solve();
}
