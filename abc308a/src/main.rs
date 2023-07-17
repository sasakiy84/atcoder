// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        ss: [usize; 8],
    }

    let ss: Vec<usize> = ss;
    let condition1 = ss.windows(2).all(|s| s[0] <= s[1]);
    let condition2 = ss.iter().all(|s| s >= &100 && s <= &675);
    let condition3 = ss.iter().all(|s| s % 25 == 0);

    println!(
        "{}",
        if condition1 && condition2 && condition3 {
            "Yes"
        } else {
            "No"
        }
    );
}
