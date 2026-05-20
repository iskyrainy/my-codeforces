use std::{
    collections::HashMap,
    io::{self, BufRead},
};
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
    let count = read!(usize);
    let mut sets = HashMap::with_capacity(count);
    for _ in 0..count {
        let name = read!(String);
        if !sets.contains_key(&name) {
            sets.insert(name, 0);
            println!("OK");
        } else {
            let suffix = sets.get(&name).unwrap();
            println!("{}{}", &name, suffix + 1);
            sets.insert(name, suffix + 1);
        }
    }
}
fn main() {
    solve();
}
