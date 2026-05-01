/// LeetCode 2466

pub struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let high = high as usize;
        let low = low as usize;
        let zero = zero as usize;
        let one = one as usize;
        let modv: i64 = 1_000_000_007;
        let mut dp = vec![0i64; high + 1];
        dp[0] = 1;
        for i in 1..=high {
            if i >= zero { dp[i] += dp[i - zero]; }
            if i >= one { dp[i] += dp[i - one]; }
            dp[i] %= modv;
        }
        let res: i64 = dp[low..=high].iter().sum::<i64>() % modv;
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::count_ways_good_strings::Solution;

    #[test]
    fn test_count_good_strings() {
        assert_eq!(8, Solution::count_good_strings(3, 3, 1, 1));
        assert_eq!(5, Solution::count_good_strings(2, 3, 1, 2));
    }
}
