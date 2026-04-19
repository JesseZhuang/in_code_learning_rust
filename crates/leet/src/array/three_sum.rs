/// LeetCode 15 - 3Sum
/// Sort + two-pointer approach
/// Time: O(n²) — one outer loop × linear two-pointer scan
/// Space: O(1) — excluding the output vector (sorting is in-place)
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        if n < 3 {
            return result;
        }
        nums.sort_unstable(); // O(n log n) — enables two-pointer technique

        for i in 0..n - 2 {
            if nums[i] > 0 {
                break; // smallest element positive ⇒ no triplet sums to zero
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate first element
            }
            let mut lo = i + 1; // left pointer
            let mut hi = n - 1; // right pointer
            while lo < hi {
                let sum = nums[i] + nums[lo] + nums[hi]; // O(1) per check
                if sum < 0 {
                    lo += 1; // need a larger sum
                } else if sum > 0 {
                    hi -= 1; // need a smaller sum
                } else {
                    result.push(vec![nums[i], nums[lo], nums[hi]]);
                    // skip duplicates on both pointers
                    while lo < hi && nums[lo] == nums[lo + 1] {
                        lo += 1;
                    }
                    while lo < hi && nums[hi] == nums[hi - 1] {
                        hi -= 1;
                    }
                    lo += 1;
                    hi -= 1;
                }
            }
        }
        result
    }
}

/// Hash-set approach
/// Time: O(n²) — for each pair, hash lookup is O(1) amortized
/// Space: O(n) — hash set for the inner loop
struct Solution2;

impl Solution2 {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut result: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        if n < 3 {
            return result;
        }
        nums.sort_unstable(); // sort so we can skip duplicates easily

        for i in 0..n - 2 {
            if nums[i] > 0 {
                break; // early termination — same reasoning as Solution
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate first element
            }
            let mut seen: HashSet<i32> = HashSet::new(); // O(n) space per outer iteration
            let mut j = i + 1;
            while j < n {
                let complement = -nums[i] - nums[j]; // target for the third element
                if seen.contains(&complement) {
                    result.push(vec![nums[i], complement, nums[j]]);
                    // skip duplicates on j
                    while j + 1 < n && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                }
                seen.insert(nums[j]); // O(1) amortized insertion
                j += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: run against both solutions, sorting results for comparison.
    fn check(nums: Vec<i32>, mut expected: Vec<Vec<i32>>) {
        expected.sort();
        let mut r1 = Solution::three_sum(nums.clone());
        r1.sort();
        assert_eq!(r1, expected, "Solution failed");
        let mut r2 = Solution2::three_sum(nums);
        r2.sort();
        assert_eq!(r2, expected, "Solution2 failed");
    }

    #[test]
    fn example1() {
        check(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        );
    }

    #[test]
    fn example2() {
        check(vec![0, 1, 1], vec![]);
    }

    #[test]
    fn example3() {
        check(vec![0, 0, 0], vec![vec![0, 0, 0]]);
    }

    #[test]
    fn empty() {
        check(vec![], vec![]);
    }

    #[test]
    fn two_elements() {
        check(vec![1, -1], vec![]);
    }

    #[test]
    fn all_zeros() {
        check(vec![0, 0, 0, 0], vec![vec![0, 0, 0]]);
    }

    #[test]
    fn no_triplet() {
        check(vec![1, 2, 3, 4, 5], vec![]);
    }

    #[test]
    fn all_negative() {
        check(vec![-5, -4, -3, -2, -1], vec![]);
    }

    #[test]
    fn multiple_triplets() {
        check(
            vec![-2, 0, 1, 1, 2],
            vec![vec![-2, 0, 2], vec![-2, 1, 1]],
        );
    }

    #[test]
    fn large_duplicates() {
        check(
            vec![-1, -1, -1, 0, 1, 1, 1],
            vec![vec![-1, 0, 1]],
        );
    }
}
