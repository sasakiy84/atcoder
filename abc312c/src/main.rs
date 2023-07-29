// -*- coding:utf-8-unix -*-

// 解けてない

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        m: usize,
        mut seller: [i64; n],  // Vec<(i32, i32, i32)>
        mut buyer: [i64; m],  // Vec<(i32, i32, i32)>
    }
    let mut seller: Vec<i64> = seller;
    let mut buyer: Vec<i64> = buyer;

    seller.sort();
    buyer.sort();

    let mut solved = false;

    for (i, sell_price) in seller.iter().enumerate() {
        if m - i < 2 {
            break;
        }
        let ok = &buyer[(m - i - 2)] >= sell_price;
        if ok {
            println!("{}", sell_price);
            solved = true;
            break;
        }
    }

    if solved == false {
        println!("{}", buyer.last().unwrap() + 1)
    }
}
