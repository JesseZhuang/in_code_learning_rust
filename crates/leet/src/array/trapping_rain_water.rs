/// LeetCode 42 - Trapping Rain Water
/// Two-pointer approach
/// Time: O(n) — single pass from both ends
/// Space: O(1) — only two pointers and two max trackers
struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut left_max = height[left]; // tallest bar seen from the left
        let mut right_max = height[right]; // tallest bar seen from the right
        let mut water = 0i32;

        while left < right {
            if left_max <= right_max {
                // water at `left` is bounded by left_max (the shorter side)
                left += 1;
                left_max = left_max.max(height[left]);
                water += left_max - height[left]; // O(1) per position
            } else {
                // water at `right` is bounded by right_max
                right -= 1;
                right_max = right_max.max(height[right]);
                water += right_max - height[right]; // O(1) per position
            }
        }
        water
    }
}

/// Monotonic-stack approach
/// Time: O(n) — each bar is pushed and popped at most once
/// Space: O(n) — stack can hold up to n indices
struct Solution2;

impl Solution2 {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new(); // stores indices; heights are non-increasing
        let mut water = 0i32;

        for i in 0..height.len() {
            // pop shorter bars and accumulate horizontal water layers
            while let Some(&top) = stack.last() {
                if height[i] <= height[top] {
                    break;
                }
                stack.pop(); // pop the bottom of the "pool"
                if let Some(&left) = stack.last() {
                    let bounded_height = height[i].min(height[left]) - height[top]; // water level minus bottom
                    let width = (i - left - 1) as i32; // horizontal span
                    water += bounded_height * width; // area of this horizontal layer
                }
            }
            stack.push(i); // O(1) amortized — each index pushed once
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to run a test case against both solutions.
    fn check(height: Vec<i32>, expected: i32) {
        assert_eq!(Solution::trap(height.clone()), expected, "Solution failed");
        assert_eq!(Solution2::trap(height), expected, "Solution2 failed");
    }

    #[test]
    fn example_1() {
        check(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
    }

    #[test]
    fn example_2() {
        check(vec![4, 2, 0, 3, 2, 5], 9);
    }

    #[test]
    fn empty() {
        check(vec![], 0);
    }

    #[test]
    fn single() {
        check(vec![5], 0);
    }

    #[test]
    fn simple_valley() {
        check(vec![3, 0, 3], 3);
    }

    #[test]
    fn strictly_decreasing() {
        check(vec![5, 4, 3, 2, 1], 0);
    }

    #[test]
    fn strictly_increasing() {
        check(vec![1, 2, 3, 4, 5], 0);
    }

    #[test]
    fn flat() {
        check(vec![2, 2, 2, 2], 0);
    }

    #[test]
    fn wide_valley() {
        check(vec![5, 0, 0, 0, 5], 15);
    }

    #[test]
    fn two_valleys() {
        check(vec![3, 0, 2, 0, 4], 7);
    }
}
