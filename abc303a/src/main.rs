// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let n: usize = n;

    let mut ok = true;
    for i in 0..n {
        let x = s[i];
        let y = t[i];

        if x == y {
            continue;
        } else if (x == '1' && y == 'l') || (x == 'l' || y == '1') {
            continue;
        } else if (x == '0' && y == 'o') || (x == 'o' || y == '0') {
            continue;
        }

        ok = false;
        break;
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
