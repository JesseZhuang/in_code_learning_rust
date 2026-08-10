pub struct Solution;

impl Solution {
    /// DP approach: O(N*sqrt(N)) time, O(N) space
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[n]
    }

    /// Math approach using Lagrange's four-square theorem and Legendre's three-square theorem.
    /// O(sqrt(N)) time, O(1) space.
    ///
    /// Key facts:
    /// - Every natural number can be represented as the sum of at most 4 perfect squares.
    /// - A number can be represented as the sum of 3 squares iff it is NOT of the form 4^a*(8b+7).
    /// - Check if n is a perfect square (answer 1).
    /// - Check if n is a sum of two squares (answer 2).
    /// - Check Legendre's condition for answer 4.
    /// - Otherwise answer is 3.
    pub fn num_squares_math(n: i32) -> i32 {
        let is_square = |x: i32| -> bool {
            let s = (x as f64).sqrt() as i32;
            s * s == x || (s + 1) * (s + 1) == x
        };

        // Check if answer is 1
        if is_square(n) {
            return 1;
        }

        // Check if answer is 4: n = 4^a * (8b + 7)
        let mut tmp = n;
        while tmp % 4 == 0 {
            tmp /= 4;
        }
        if tmp % 8 == 7 {
            return 4;
        }

        // Check if answer is 2: n = a^2 + b^2
        let mut i = 1;
        while i * i <= n {
            if is_square(n - i * i) {
                return 2;
            }
            i += 1;
        }

        // Otherwise answer is 3
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares() {
        let cases = vec![
            (12, 3),
            (13, 2),
            (1, 1),
            (4, 1),
            (7, 4),
            (15, 4),
            (100, 1),
            (2, 2),
            (3, 3),
            (10000, 1),
            (9999, 4),
        ];
        for (n, expected) in &cases {
            assert_eq!(Solution::num_squares(*n), *expected, "dp failed for n={}", n);
            assert_eq!(
                Solution::num_squares_math(*n),
                *expected,
                "math failed for n={}",
                n
            );
        }
    }
}
