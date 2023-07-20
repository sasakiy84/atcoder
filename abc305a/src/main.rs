// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
    }

    let n: usize = n;
    let distance = n % 5;
    let ans = if 3 <= distance {
        n + (5 - distance)
    } else {
        n - distance
    };

    println!("{}", ans);
}
