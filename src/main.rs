// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![[0; 30]; 30];
    for i in 0..n {
        for j in 0..=i {
            if j == 0 || j == i {
                ans[i][j] = 1;
            }
            else {
                ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
            }
        }
    }
    for i in 0..n {
        for j in 0..=i {
            print!("{} ", ans[i][j]);
        }
        print!("\n");
    }
}
