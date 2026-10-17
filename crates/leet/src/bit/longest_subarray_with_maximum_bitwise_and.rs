pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut maximum = i32::MIN;
        let mut run = 0;
        let mut best = 0;

        // O(n) time, O(1) extra space.
        for num in nums {
            if num > maximum {
                maximum = num;
                run = 1;
                best = 1;
            } else if num == maximum {
                run += 1;
                best = best.max(run);
            } else {
                run = 0;
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn singleton() {
        assert_eq!(Solution::longest_subarray(vec![42]), 1);
    }

    #[test]
    fn all_equal() {
        assert_eq!(Solution::longest_subarray(vec![7, 7, 7, 7]), 4);
    }

    #[test]
    fn maximum_late() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 9, 9, 9]), 3);
        assert_eq!(Solution::longest_subarray(vec![5, 5, 5, 1, 8]), 1);
    }

    #[test]
    fn split_runs() {
        assert_eq!(
            Solution::longest_subarray(vec![7, 7, 1, 7, 7, 7, 2, 7, 7]),
            3
        );
    }

    #[test]
    fn extreme_values() {
        assert_eq!(
            Solution::longest_subarray(vec![1, i32::MAX, i32::MAX, 1, i32::MAX]),
            2
        );
    }

    #[test]
    fn hundred_thousand_elements() {
        assert_eq!(Solution::longest_subarray(vec![i32::MAX; 100_000]), 100_000);
    }
}
