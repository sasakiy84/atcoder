// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        _n: usize,
        s: Chars,  // Vec<(i32, i32, i32)>
    }

    let t: usize = s.clone().into_iter().filter(|c| c == &'T').count();
    let a: usize = s.clone().into_iter().filter(|c| c == &'A').count();
    let ans = if t == a {
        if s.last().unwrap() == &'T' {
            'A'
        } else {
            'T'
        }
    } else if t < a {
        'A'
    } else {
        'T'
    };

    println!("{}", ans);
}
