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
    let d = read!(i32);
    let total = read!(i32);
    let mut mins = vec![];
    let mut maxs = vec![];
    for _ in 0..d {
        mins.push(read!(i32));
        maxs.push(read!(i32));
    }
    let min_sum = mins.iter().sum::<i32>();
    let max_sum = maxs.iter().sum::<i32>();
    if total < min_sum || total > max_sum {
        println!("NO");
        return;
    }
    let mut res = mins.clone();
    let mut remain = total - min_sum;
    for i in 0..(d as usize) {
        let can_add = maxs[i] - mins[i];
        let add = remain.min(can_add);
        res[i] += add;
        remain -= add;
        if remain == 0 {
            break;
        }
    }
    println!("YES");
    for (i, out) in res.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", out);
    }
    println!();
}
fn main() {
    solve();
}
