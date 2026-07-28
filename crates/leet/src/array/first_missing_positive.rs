pub struct Solution;

impl Solution {
    /// Cyclic sort approach: place each value v in [1, n] at index v-1.
    /// O(n) time, O(1) space.
    pub fn first_missing_positive(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        // O(n) — each element is swapped at most once into its correct position
        let mut i = 0;
        while i < n {
            let v = nums[i];
            if v > 0 && (v as usize) <= n && nums[(v - 1) as usize] != v {
                let target = (v - 1) as usize;
                nums.swap(i, target);
            } else {
                i += 1;
            }
        }
        // O(n) — linear scan for first missing
        for i in 0..n {
            if nums[i] != (i as i32) + 1 {
                return (i as i32) + 1;
            }
        }
        (n as i32) + 1
    }

    /// Index marking approach: use sign of nums[i] to mark presence of i+1.
    /// O(n) time, O(1) space.
    pub fn first_missing_positive_marking(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        // Step 1 O(n): replace non-positive and out-of-range values with n+1
        for i in 0..n {
            if nums[i] <= 0 || nums[i] > n as i32 {
                nums[i] = (n as i32) + 1;
            }
        }
        // Step 2 O(n): for each value v in [1,n], mark index v-1 as negative
        for i in 0..n {
            let v = nums[i].unsigned_abs() as usize;
            if v >= 1 && v <= n {
                nums[v - 1] = -nums[v - 1].abs();
            }
        }
        // Step 3 O(n): first positive index + 1 is the answer
        for i in 0..n {
            if nums[i] > 0 {
                return (i as i32) + 1;
            }
        }
        (n as i32) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&mut Vec<i32>) -> i32) {
        assert_eq!(f(&mut vec![1, 2, 0]), 3);
        assert_eq!(f(&mut vec![3, 4, -1, 1]), 2);
        assert_eq!(f(&mut vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(f(&mut vec![1]), 2);
        assert_eq!(f(&mut vec![2]), 1);
        assert_eq!(f(&mut vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(f(&mut vec![1, 1, 1, 1]), 2);
        assert_eq!(f(&mut vec![-1, -2, -3]), 1);
        assert_eq!(f(&mut vec![i32::MAX, i32::MIN, 1, 2]), 3);
    }

    #[test]
    fn test_cyclic_sort() {
        run_tests(Solution::first_missing_positive);
    }

    #[test]
    fn test_marking() {
        run_tests(Solution::first_missing_positive_marking);
    }
}
