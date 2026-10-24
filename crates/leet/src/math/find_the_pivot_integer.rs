pub struct Solution;

impl Solution {
    /// O(1) time and space: the pivot's prefix sum equals the total suffix sum.
    pub fn pivot_integer(n: i32) -> i32 {
        if n < 1 {
            return -1;
        }

        let n = i64::from(n);
        let total = n * (n + 1) / 2;
        let pivot = (total as f64).sqrt().round() as i64;
        if pivot * pivot == total {
            pivot as i32
        } else {
            -1
        }
    }

    /// O(n) time and O(1) space with running prefix and suffix sums.
    pub fn pivot_integer_prefix_sum(n: i32) -> i32 {
        let n = i64::from(n);
        let mut prefix = 0;
        let mut suffix = n * (n + 1) / 2;

        // O(n) iterations; each pivot advances both sums in constant time.
        for pivot in 1..=n {
            prefix += pivot;
            if prefix == suffix {
                return pivot as i32;
            }
            suffix -= pivot;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn known_cases() {
        for (n, expected) in [(1, 1), (8, 6), (4, -1), (49, 35)] {
            assert_eq!(Solution::pivot_integer(n), expected, "math: n={n}");
            assert_eq!(
                Solution::pivot_integer_prefix_sum(n),
                expected,
                "prefix/suffix: n={n}"
            );
        }
    }

    #[test]
    fn both_algorithms_agree_for_first_thousand_inputs() {
        for n in 1..=1000 {
            assert_eq!(
                Solution::pivot_integer(n),
                Solution::pivot_integer_prefix_sum(n),
                "n={n}"
            );
        }
    }
}
