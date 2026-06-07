use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        // O(n) space: map prefix_sum -> earliest index
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1); // base case: prefix sum 0 at virtual index -1

        let mut prefix_sum = 0; // O(1) running sum
        let mut max_len = 0;

        for (i, &num) in nums.iter().enumerate() {
            // Treat 0 as -1 so equal 0s and 1s yield sum = 0
            prefix_sum += if num == 1 { 1 } else { -1 }; // O(1) per element

            if let Some(&prev_idx) = map.get(&prefix_sum) {
                // O(1) lookup: same prefix_sum means subarray sums to 0
                max_len = max_len.max(i as i32 - prev_idx);
            } else {
                // O(1) insert: store earliest occurrence
                map.insert(prefix_sum, i as i32);
            }
        }

        max_len // O(n) time total, O(n) space for HashMap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_length() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 0, 0]), 0);
        assert_eq!(Solution::find_max_length(vec![0, 1, 1, 0]), 4);
        assert_eq!(Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]), 6);
    }
}
