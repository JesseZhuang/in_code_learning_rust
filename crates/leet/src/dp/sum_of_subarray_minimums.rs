/// LeetCode 907, medium, tags: array, dynamic programming, stack, monotonic stack.

const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    /// Monotonic stack with sentinels. O(n) time, O(n) space.
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut result: i64 = 0;
        let mut stack: Vec<usize> = Vec::new(); // indices
        // Sentinel: prepend and append 0
        let arr: Vec<i32> = std::iter::once(0)
            .chain(arr.into_iter())
            .chain(std::iter::once(0))
            .collect();

        for i in 0..arr.len() { // O(n), each index pushed/popped once
            while !stack.is_empty() && arr[*stack.last().unwrap()] > arr[i] {
                let mid = stack.pop().unwrap();
                let left = (mid - *stack.last().unwrap()) as i64;
                let right = (i - mid) as i64;
                result = (result + arr[mid] as i64 * left * right) % MOD;
            }
            stack.push(i);
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(17, Solution::sum_subarray_mins(vec![3, 1, 2, 4]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(444, Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]));
    }

    #[test]
    fn test_single() {
        assert_eq!(5, Solution::sum_subarray_mins(vec![5]));
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(4, Solution::sum_subarray_mins(vec![1, 2]));
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(10, Solution::sum_subarray_mins(vec![3, 2, 1]));
    }

    #[test]
    fn test_increasing() {
        assert_eq!(10, Solution::sum_subarray_mins(vec![1, 2, 3]));
    }

    #[test]
    fn test_all_same() {
        assert_eq!(12, Solution::sum_subarray_mins(vec![2, 2, 2]));
    }
}
