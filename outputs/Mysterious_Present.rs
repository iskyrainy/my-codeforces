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
    let (num, card_w, card_h) = (read!(usize), read!(usize), read!(usize));
    let mut envelopes = vec![];
    for i in 0..num {
        let w = read!(usize);
        let h = read!(usize);
        if w > card_w && h > card_h {
            envelopes.push((i + 1, w, h));
        }
    }
    if envelopes.is_empty() {
        println!("0");
        return;
    }
    envelopes.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| b.2.cmp(&a.2)));
    let m = envelopes.len();
    let mut lis = Vec::new();
    let mut lis_idx = Vec::new();
    let mut parent = vec![m; m];
    for i in 0..m {
        let h = envelopes[i].2;
        let pos = lis.binary_search(&h).unwrap_or_else(|x| x);
        if pos == lis.len() {
            lis.push(h);
            lis_idx.push(i);
        } else {
            lis[pos] = h;
            lis_idx[pos] = i;
        }
        if pos > 0 {
            parent[i] = lis_idx[pos - 1];
        }
    }
    let mut result = Vec::new();
    let mut cur = *lis_idx.last().unwrap();
    while cur < m {
        result.push(envelopes[cur].0);
        if parent[cur] == m {
            break;
        }
        cur = parent[cur];
    }
    result.reverse();
    println!("{}", lis.len());
    for (i, idx) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", idx);
    }
    println!();
}
fn main() {
    solve();
}
