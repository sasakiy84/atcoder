// -*- coding:utf-8-unix -*-

use std::convert::TryInto;

use proconio::input;
use proconio::marker::Chars;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Chars; n],  // Vec<(i32, i32, i32)>
    }

    let n: usize = n;
    let m: usize = m;
    let ss: Vec<Vec<char>> = ss;

    let mut ans: Vec<(i128, i128)> = vec![];

    for i in 0..n - 8 {
        for j in 0..m - 8 {
            let mut ok = true;
            for k in i..i + 3 {
                if !(ss[k][j] == '#'
                    && ss[k][j + 1] == '#'
                    && ss[k][j + 2] == '#'
                    && ss[k][j + 3] == '.')
                {
                    ok = false;
                }
            }
            for k in i + 6..i + 9 {
                if !(ss[k][j + 5] == '.'
                    && ss[k][j + 6] == '#'
                    && ss[k][j + 7] == '#'
                    && ss[k][j + 8] == '#')
                {
                    ok = false;
                }
            }

            if !(ss[i + 3][j] == '.'
                && ss[i + 3][j + 1] == '.'
                && ss[i + 3][j + 2] == '.'
                && ss[i + 3][j + 3] == '.')
            {
                ok = false;
            }
            if !(ss[i + 5][j + 5] == '.'
                && ss[i + 5][j + 6] == '.'
                && ss[i + 5][j + 7] == '.'
                && ss[i + 5][j + 8] == '.')
            {
                ok = false;
            }

            if ok {
                ans.push((i.try_into().unwrap(), j.try_into().unwrap()))
            }
        }
    }

    ans.sort_by(|a, b| {
        if (a.0 - b.0) * 10 > 0 {
            std::cmp::Ordering::Greater
        } else {
            if (a.0 - b.0) * 10 < 0 {
                std::cmp::Ordering::Less
            } else {
                if a.1 - b.1 > 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
        }
    });

    if ans.len() == 0 {
        println!("")
    }

    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1)
    }
}
