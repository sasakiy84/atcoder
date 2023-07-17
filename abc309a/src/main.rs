// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!(
        "{}",
        if a % 3 != 0 && a + 1 == b {
            "Yes"
        } else {
            "No"
        }
    );
}
