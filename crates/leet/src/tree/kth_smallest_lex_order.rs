// lc 440

use std::cmp::min;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let cnt_steps = |cur: i32| -> i32 {
            let mut cur = cur as u64;
            let mut next = cur.checked_add(1).expect("overflow");
            let (n, mut res) = (n as u64, 0);
            while cur <= n {
                res += min(n + 1, next) - cur;
                cur *= 10;
                next *= 10;
            }
            res as i32
        };
        let (mut k, mut cur) = (k - 1, 1);
        while k != 0 {
            let steps = cnt_steps(cur);
            if steps <= k {
                k -= steps;
                cur += 1;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }
}

struct Solution;