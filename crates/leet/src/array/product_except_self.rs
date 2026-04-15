// leet 238

struct Solution;

impl Solution {
    // O(n) time, O(1) extra space
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];

        // Forward pass: answer[i] = product of nums[0..i]
        let mut prefix = 1;
        for i in 0..n {
            answer[i] = prefix;
            prefix *= nums[i];
        }

        // Reverse pass: multiply by product of nums[i+1..n]
        let mut suffix = 1;
        for i in (0..n).rev() {
            answer[i] *= suffix;
            suffix *= nums[i];
        }

        answer
    }

    // O(n) time, O(n) space
    pub fn product_except_self_v2(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![1; n];
        let mut suffix = vec![1; n];

        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }

        (0..n).map(|i| prefix[i] * suffix[i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
            (vec![0, 0], vec![0, 0]),
            (vec![1, 0], vec![0, 1]),
            (vec![2, 3], vec![3, 2]),
            (vec![5], vec![1]),
            (vec![1, 2, 3, 4, 5], vec![120, 60, 40, 30, 24]),
        ];
        for (nums, exp) in cases {
            assert_eq!(Solution::product_except_self(nums.clone()), exp);
            assert_eq!(Solution::product_except_self_v2(nums), exp);
        }
    }
}
