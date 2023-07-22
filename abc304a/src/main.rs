// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        people: [(String, usize); n],  // Vec<(i32, i32, i32)>
    }

    let people: Vec<(String, usize)> = people;
    let mut min_index: usize = 1 << 32 - 1;
    let mut min_year: usize = 1 << 32 - 1;

    for (index, (_, year)) in people.clone().into_iter().enumerate() {
        if year < min_year {
            min_index = index;
            min_year = year;
        }
    }

    let (second, first) = people.split_at(min_index);

    for (name, _) in first {
        println!("{}", name);
    }
    for (name, _) in second {
        println!("{}", name);
    }

    // 模範解答
    // let si = people.iter().position_min_by_key(|x| x.age).unwrap();
    // for offset in 0..n {
    //     println!("{}", people[(si + offset) % n].name)
    // }
}
