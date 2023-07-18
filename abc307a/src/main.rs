// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        plan: [[usize; 7]; n],  // Vec<(i32, i32, i32)>
    }

    let n: usize = n;
    let plan: Vec<Vec<usize>> = plan;

    let mut ans: Vec<_> = vec![];
    for i in 0..n {
        ans.push(plan[i].iter().sum::<usize>().to_string());
    }

    println!("{}", ans.join(" "))
}
