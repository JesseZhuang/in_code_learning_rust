// lc 84

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::new(); // monotonic increasing stack of indices
        let mut max_area = 0;
        for i in 0..=n { // O(n)
            let h = if i == n { 0 } else { heights[i] };
            while let Some(&top) = stack.last() { // each index pushed/popped once, O(n) total
                if h < heights[top] {
                    stack.pop();
                    let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                    max_area = max_area.max(heights[top] as i64 * width as i64);
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        max_area as i32 // Time O(n), Space O(n)
    }

    pub fn largest_rectangle_area_array(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left_wall = vec![-1i64; n];
        let mut right_wall = vec![n as i64; n];
        for i in 1..n { // O(n) amortized
            let mut p = i as i64 - 1;
            while p >= 0 && heights[p as usize] >= heights[i] {
                p = left_wall[p as usize];
            }
            left_wall[i] = p;
        }
        for i in (0..n.saturating_sub(1)).rev() { // O(n) amortized
            let mut p = i as i64 + 1;
            while p < n as i64 && heights[p as usize] >= heights[i] {
                p = right_wall[p as usize];
            }
            right_wall[i] = p;
        }
        let mut max_area: i64 = 0;
        for i in 0..n { // O(n)
            max_area = max_area.max(heights[i] as i64 * (right_wall[i] - left_wall[i] - 1));
        }
        max_area as i32 // Time O(n), Space O(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_example1() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }

    #[test]
    fn test_stack_example2() {
        assert_eq!(4, Solution::largest_rectangle_area(vec![2, 4]));
    }

    #[test]
    fn test_stack_single() {
        assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
    }

    #[test]
    fn test_stack_increasing() {
        assert_eq!(9, Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_stack_decreasing() {
        assert_eq!(9, Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_stack_all_same() {
        assert_eq!(12, Solution::largest_rectangle_area(vec![3, 3, 3, 3]));
    }

    #[test]
    fn test_array_example1() {
        assert_eq!(10, Solution::largest_rectangle_area_array(vec![2, 1, 5, 6, 2, 3]));
    }

    #[test]
    fn test_array_example2() {
        assert_eq!(4, Solution::largest_rectangle_area_array(vec![2, 4]));
    }

    #[test]
    fn test_array_single() {
        assert_eq!(5, Solution::largest_rectangle_area_array(vec![5]));
    }

    #[test]
    fn test_array_increasing() {
        assert_eq!(9, Solution::largest_rectangle_area_array(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_array_decreasing() {
        assert_eq!(9, Solution::largest_rectangle_area_array(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_array_all_same() {
        assert_eq!(12, Solution::largest_rectangle_area_array(vec![3, 3, 3, 3]));
    }
}
