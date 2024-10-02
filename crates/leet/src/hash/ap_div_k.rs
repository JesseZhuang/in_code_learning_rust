use std::collections::HashMap;

/// lc 1497

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut rem_cnt = HashMap::new();
        for a in arr.iter() {
            let rem = (a % k + k) % k;
            *rem_cnt.entry(rem).or_insert(0) += 1;
        }
        for a in arr.iter() {
            let rem = (a % k + k) % k;
            if rem == 0 {
                if rem_cnt.get(&rem).expect("not possible") % 2 == 1 { return false; }
            } else if rem_cnt.get(&rem) != rem_cnt.get(&(k - rem)) { return false }
        }
        true
    }
}

struct Solution;
