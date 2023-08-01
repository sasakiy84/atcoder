// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        canditates: [usize; n],  // Vec<(i32, i32, i32)>
    }

    let canditates: Vec<usize> = canditates;
    let ans = canditates.iter().position(|v| a + b == *v).unwrap();

    println!("{}", ans + 1);
}
