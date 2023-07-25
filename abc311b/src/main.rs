// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        d: usize,
        ss: [Chars; n]
    }

    let ss: Vec<Vec<char>> = ss;
    let d: usize = d;

    let mut combined: Vec<bool> = vec![];

    for i in 0..d {
        let mut ok = true;
        for s in ss.clone() {
            if s[i] == 'x' {
                ok = false;
                break;
            }
        }
        combined.push(ok);
    }

    let mut canditate = vec![0];
    let mut day = 0;
    for r in combined {
        if r {
            day += 1;
        } else {
            canditate.push(day);
            day = 0;
        }
    }
    canditate.push(day);
    let ans = canditate.into_iter().max().unwrap();
    println!("{}", ans);
}
