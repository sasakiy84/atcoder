// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        s: String  // Vec<(i32, i32, i32)>
    }
    let ans = s == "ACE"
        || "BDF" == s
        || "CEG" == s
        || "DFA" == s
        || "EGB" == s
        || "FAC" == s
        || "GBD" == s;
    println!("{}", if ans { "Yes" } else { "No" });
}
