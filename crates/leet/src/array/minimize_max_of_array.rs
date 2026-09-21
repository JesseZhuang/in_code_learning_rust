/// leet 2439

pub struct Solution;

impl Solution {
    /// Prefix sum greedy: answer = max(ceil(prefix_sum / (i+1))) for i in [0, n).
    /// O(n) time, O(1) space.
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut prefix_sum: i64 = 0;
        let mut ans: i64 = 0;
        for (i, &num) in nums.iter().enumerate() {
            prefix_sum += num as i64;
            let ceil_avg = (prefix_sum + i as i64) / (i as i64 + 1);
            ans = ans.max(ceil_avg);
        }
        ans as i32
    }

    /// Binary search on answer in [0, max(nums)].
    /// Feasibility check: scan left-to-right tracking excess.
    /// O(n log(max)) time, O(1) space.
    pub fn minimize_array_value_bs(nums: Vec<i32>) -> i32 {
        let mut lo: i64 = 0;
        let mut hi: i64 = *nums.iter().max().unwrap() as i64;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::feasible(&nums, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }

    fn feasible(nums: &[i32], cap: i64) -> bool {
        let mut excess: i64 = 0;
        for &num in nums {
            excess += num as i64 - cap;
            if excess > 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const CASES: &[(&[i32], i32)] = &[
        (&[3, 7, 1, 6], 5),
        (&[10, 1], 10),
        (&[5], 5),
        (&[4, 4, 4, 4], 4),
        (&[10, 5, 1], 10),
        (&[1, 5, 10], 6),
        (&[0, 0, 0], 0),
        (&[0, 0, 0, 0, 100], 20),
        (&[1, 9], 5),
        (&[1, 10], 6),
    ];

    #[test]
    fn test_minimize_array_value() {
        for &(nums, expected) in CASES {
            assert_eq!(
                Solution::minimize_array_value(nums.to_vec()),
                expected,
                "failed for {:?}",
                nums
            );
        }
    }

    #[test]
    fn test_minimize_array_value_bs() {
        for &(nums, expected) in CASES {
            assert_eq!(
                Solution::minimize_array_value_bs(nums.to_vec()),
                expected,
                "failed for {:?}",
                nums
            );
        }
    }
}
