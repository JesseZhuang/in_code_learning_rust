pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1i32; n]; // O(n) space
        let mut stack: Vec<usize> = Vec::new(); // monotonic decreasing stack of indices, O(n) space
        for i in 0..2 * n { // O(n), each index pushed/popped at most once
            while let Some(&j) = stack.last() { // O(n) total pops
                if nums[j] < nums[i % n] {
                    stack.pop();
                    res[j] = nums[i % n];
                } else {
                    break;
                }
            }
            if i < n {
                stack.push(i);
            }
        }
        res // Time O(n), Space O(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::next_greater_elements(vec![5]), vec![-1]);
        assert_eq!(
            Solution::next_greater_elements(vec![3, 3, 3]),
            vec![-1, -1, -1]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![4, 3, 5]),
            vec![5, 5, -1]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4]),
            vec![2, 3, 4, -1]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![3, 1, 5, 2, 4]),
            vec![5, 5, -1, 4, 5]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2]),
            vec![2, -1]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![2, 2]),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![-1, -3, 0]),
            vec![0, 0, -1]
        );
    }
}
