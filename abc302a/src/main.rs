// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        a: usize,
        b: usize,  // Vec<(i32, i32, i32)>
    }
    let c = a % b;
    let ans = if c == 0 { a / b } else { a / b + 1 };
    println!("{}", ans);
}
