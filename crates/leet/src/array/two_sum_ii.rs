/// LeetCode 167 - Two Sum II (Input Array Is Sorted)
/// Two-pointer approach
/// Time: O(n) — single pass with two converging pointers
/// Space: O(1) — only two index variables
pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lo = 0usize;
        let mut hi = numbers.len() - 1;
        while lo < hi {
            let sum = numbers[lo] + numbers[hi]; // O(1) per check
            if sum == target {
                return vec![lo as i32 + 1, hi as i32 + 1]; // 1-indexed
            } else if sum < target {
                lo += 1; // need a larger sum
            } else {
                hi -= 1; // need a smaller sum
            }
        }
        unreachable!("problem guarantees exactly one solution")
    }
}

/// Binary search approach
/// Time: O(n log n) — for each element, binary search the complement
/// Space: O(1) — no extra allocation
struct Solution2;

impl Solution2 {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        for i in 0..n - 1 {
            let complement = target - numbers[i];
            // binary search for complement in numbers[i+1..n]
            let (mut lo, mut hi) = (i + 1, n - 1);
            while lo <= hi {
                let mid = lo + (hi - lo) / 2; // O(log n) per outer iteration
                if numbers[mid] == complement {
                    return vec![i as i32 + 1, mid as i32 + 1]; // 1-indexed
                } else if numbers[mid] < complement {
                    lo = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    hi = mid - 1;
                }
            }
        }
        unreachable!("problem guarantees exactly one solution")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: run against both solutions and compare.
    fn check(numbers: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(
            Solution::two_sum(numbers.clone(), target),
            expected,
            "Solution (two-pointer) failed"
        );
        assert_eq!(
            Solution2::two_sum(numbers, target),
            expected,
            "Solution2 (binary search) failed"
        );
    }

    #[test]
    fn example1() {
        check(vec![2, 7, 11, 15], 9, vec![1, 2]);
    }

    #[test]
    fn example2() {
        check(vec![2, 3, 4], 6, vec![1, 3]);
    }

    #[test]
    fn example3() {
        check(vec![-1, 0], -1, vec![1, 2]);
    }

    #[test]
    fn negatives() {
        check(vec![-3, -1, 0, 2, 4], -4, vec![1, 2]);
    }

    #[test]
    fn boundary_values() {
        check(vec![-1000, 1000], 0, vec![1, 2]);
    }

    #[test]
    fn duplicates_in_array() {
        check(vec![1, 2, 2, 3], 4, vec![1, 4]);
    }

    #[test]
    fn large_target() {
        check(vec![1, 2, 3, 4, 5], 9, vec![4, 5]);
    }

    #[test]
    fn negative_target() {
        check(vec![-5, -3, -1, 0, 2], -8, vec![1, 2]);
    }
}
