// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        _n: usize,
        s: Chars
    };

    let s: Vec<char> = s;

    let mut ans = vec![];
    for c in s.iter() {
        ans.push(c.to_string());
        ans.push(c.to_string())
    }

    let ans = ans.join("");
    println!("{}", ans);
}
