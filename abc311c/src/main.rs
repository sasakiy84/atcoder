// -*- coding:utf-8-unix -*-

use std::{collections::HashSet, vec};

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        graph: [usize; n],  // Vec<(i32, i32, i32)>
    };

    let n: usize = n;
    let mut unsatisfied_nodes = HashSet::new();

    for start in 0..n {
        let mut nodes = vec![start + 1];
        let mut next = graph[start];

        let mut already_appeared = HashSet::new();

        let mut ok = false;
        for _ in 0..n {
            already_appeared.insert(next);
            if next == start + 1 {
                ok = true;
                break;
            }
            nodes.push(next);
            next = graph[next - 1];
            if already_appeared.contains(&next) {
                break;
            }
            if unsatisfied_nodes.contains(&next) {
                break;
            }
        }

        if ok {
            let ans: Vec<String> = nodes.iter().map(|v| v.to_string()).collect();

            println!("{}", nodes.len());
            println!("{}", ans.join(" "));
            break;
        }

        unsatisfied_nodes.insert(start + 1);
    }
}
