// lc 1143

use std::cmp::max;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        if m < n { return Solution::longest_common_subsequence(text2, text1); }
        let vec1: Vec<char> = text1.chars().collect();
        let vec2: Vec<char> = text2.chars().collect();
        let mut dp = vec![0; n + 1];
        for i in 0..m {
            let mut pr = 0; // prev row, prev row prev col
            let mut prpc;
            for j in 0..n {
                prpc = pr;
                pr = dp[j + 1];
                dp[j + 1] = if vec1[i] == vec2[j] { prpc + 1 } else { max(dp[j], pr) }
            }
        }
        dp[n]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::dp::lcss::Solution;

    #[test]
    fn test() {
        assert_eq!(1, Solution::longest_common_subsequence(
            String::from("bsbininm"), String::from("jmjkbkjkv")));
        // bug, init pr, prpc at wrong place
    }
}