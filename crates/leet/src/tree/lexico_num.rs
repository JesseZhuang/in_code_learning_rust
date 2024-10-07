/// lc 386, medium

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let (mut res, mut cur) = (Vec::new(), 1);
        for _ in 0..n {
            res.push(cur);
            if cur * 10 <= n { cur *= 10 } else {
                while cur >= n || cur % 10 == 9 { cur /= 10 }
                cur += 1;
            }
        }
        res
    }
}

struct Solution;