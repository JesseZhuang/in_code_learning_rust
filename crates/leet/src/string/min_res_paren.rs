// lc 2232

use std::str::FromStr;

impl Solution {
    pub fn minimize_result(e: String) -> String {
        let sp: Vec<&str> = e.split('+').collect();
        let (l, r) = (sp[0], sp[1]);
        let (mut res, mut left, mut right) = (i32::MAX, 0, 1);
        for i in 0..l.len() {
            let a = if i == 0 { 1 } else { i32::from_str(&l[0..i]).unwrap() };
            let b = i32::from_str(&l[i..]).unwrap();
            for j in 1..r.len() + 1 {
                let c = i32::from_str(&r[0..j]).unwrap();
                let d = if j == r.len() { 1 } else { i32::from_str(&r[j..]).unwrap() };
                let p = a * (b + c) * d;
                if p < res {
                    (res, left, right) = (p, i, j);
                }
            }
        }
        // &slice[start_idx..end_idx]
        let (a, b, c, d) = (&l[0..left], &l[left..], &r[0..right], &r[right..]);
        format!("{a}({b}+{c}){d}")
    }
}

struct Solution;