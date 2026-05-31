pub struct Solution;

impl Solution {
    /// O(n) time, O(k) space. Monotonic deque storing indices.
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = Vec::with_capacity(nums.len() - k + 1);
        let mut dq = std::collections::VecDeque::new();
        for i in 0..nums.len() {
            if let Some(&front) = dq.front() {
                if front + k <= i {
                    dq.pop_front();
                }
            }
            while let Some(&back) = dq.back() {
                if nums[back] <= nums[i] {
                    dq.pop_back();
                } else {
                    break;
                }
            }
            dq.push_back(i);
            if i >= k - 1 {
                res.push(nums[*dq.front().unwrap()]);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
    }

    #[test]
    fn k_equals_length() {
        assert_eq!(vec![7], Solution::max_sliding_window(vec![4, 2, 7, 1], 4));
    }

    #[test]
    fn decreasing() {
        assert_eq!(
            vec![9, 8, 7],
            Solution::max_sliding_window(vec![9, 8, 7, 6, 5], 3)
        );
    }

    #[test]
    fn underflow_case() {
        assert_eq!(
            vec![10, 10, 9, 2],
            Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5)
        );
    }
}
