// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        dishes: [usize; n],  // Vec<(i32, i32, i32)>
    };

    let mut min = 1 << 32 - 1;
    for d in dishes {
        if d < min {
            min = d
        }
    }

    let q_plus_d = q + min;

    println!("{}", if q_plus_d < p { q_plus_d } else { p });
}
