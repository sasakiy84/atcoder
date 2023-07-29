// -*- coding:utf-8-unix -*-

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
    let seller: Vec<i64> = seller;
    let buyer: Vec<i64> = buyer;

    let mut tail = 0;
    let mut head = 1 * 10_i64.pow(9) + 1;

    while tail + 1 < head {
        let x = (tail + head) / 2;

        let buyer_count = buyer.iter().filter(|price| &&x <= price).count();
        let seller_count = seller.iter().filter(|price| &&x >= price).count();

        if seller_count >= buyer_count {
            head = x;
        } else {
            tail = x;
        }
    }

    println!("{}", head)
}
