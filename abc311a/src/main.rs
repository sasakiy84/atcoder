// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        _: usize,
        s: Chars,  // Vec<(i32, i32, i32)>
    }

    let mut ans = 0;
    let mut mem = [false, false, false];

    for c in s {
        ans += 1;
        if c == 'A' {
            mem[0] = true;
        } else if c == 'B' {
            mem[1] = true;
        } else {
            mem[2] = true;
        };

        if mem.iter().all(|v| *v == true) {
            break;
        }
    }
    println!("{}", ans)
}
