/// LeetCode 456 - 132 Pattern
/// Time: O(n), Space: O(n)

pub struct Solution;

impl Solution {
    /// Monotonic stack scanning right to left, tracking largest popped value as '2' candidate.
    /// Time: O(n), Space: O(n)
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut stack: Vec<i32> = Vec::new();
        // `two` is the candidate for the '2' in the 1-3-2 pattern (the middle value)
        let mut two = i32::MIN;

        for i in (0..n).rev() {
            // If nums[i] < two, we found a '1' that is less than '2',
            // and '2' was popped from a larger element ('3'), so pattern exists.
            if nums[i] < two {
                return true;
            }
            // Maintain decreasing stack; popped elements become '2' candidates
            while let Some(&top) = stack.last() {
                if top < nums[i] {
                    two = two.max(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(nums[i]);
        }
        false
    }

    /// Prefix min + monotonic stack approach.
    /// Time: O(n), Space: O(n)
    pub fn find132pattern_prefix_min(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }

        // prefix_min[i] = min(nums[0..=i])
        let mut prefix_min = vec![0i32; n];
        prefix_min[0] = nums[0];
        for i in 1..n {
            prefix_min[i] = prefix_min[i - 1].min(nums[i]);
        }

        // Stack stores indices; we scan from right to left
        let mut stack: Vec<usize> = Vec::new();

        for j in (0..n).rev() {
            // We need nums[j] > prefix_min[j] for a valid '3' candidate
            if nums[j] > prefix_min[j] {
                // Pop elements that are <= prefix_min[j] (they can't be '2')
                while let Some(&top) = stack.last() {
                    if nums[top] <= prefix_min[j] {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                // If stack top is less than nums[j], we found the pattern:
                // prefix_min[j] < nums[stack.top] < nums[j]  i.e. 1 < 2 < 3
                if let Some(&top) = stack.last() {
                    if nums[top] < nums[j] {
                        return true;
                    }
                }
                stack.push(j);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find132pattern() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
        assert!(!Solution::find132pattern(vec![1, 2]));
        assert!(!Solution::find132pattern(vec![5, 4, 3, 2, 1]));
        assert!(!Solution::find132pattern(vec![3, 3, 3, 3]));
        assert!(Solution::find132pattern(vec![-2, 1, -1]));
        assert!(Solution::find132pattern(vec![1, 3, 2]));
        assert!(Solution::find132pattern(vec![3, 5, 0, 3, 4]));
    }

    #[test]
    fn test_find132pattern_prefix_min() {
        assert!(!Solution::find132pattern_prefix_min(vec![1, 2, 3, 4]));
        assert!(Solution::find132pattern_prefix_min(vec![3, 1, 4, 2]));
        assert!(Solution::find132pattern_prefix_min(vec![-1, 3, 2, 0]));
        assert!(!Solution::find132pattern_prefix_min(vec![1, 2]));
        assert!(!Solution::find132pattern_prefix_min(vec![5, 4, 3, 2, 1]));
        assert!(!Solution::find132pattern_prefix_min(vec![3, 3, 3, 3]));
        assert!(Solution::find132pattern_prefix_min(vec![-2, 1, -1]));
        assert!(Solution::find132pattern_prefix_min(vec![1, 3, 2]));
        assert!(Solution::find132pattern_prefix_min(vec![3, 5, 0, 3, 4]));
    }
}
