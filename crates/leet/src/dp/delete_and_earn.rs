pub struct Solution;

impl Solution {
    /// DP (House Robber on frequency array).
    /// O(n + max_val) time, O(max_val) space.
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap() as usize;
        // earn[v] = v * count(v)
        let mut earn = vec![0i64; max_val + 1]; // O(max_val) space
        for &v in &nums {
            earn[v as usize] += v as i64;
        }
        // House robber DP over earn[0..=max_val]. O(max_val) time.
        let mut prev = 0i64;
        let mut curr = 0i64;
        for i in 0..=max_val {
            let tmp = curr;
            curr = curr.max(prev + earn[i]);
            prev = tmp;
        }
        curr as i32
    }

    /// Sort + Group DP: only apply "skip" penalty when values are consecutive.
    /// O(n + k log k) time, O(k) space where k = number of unique values.
    pub fn delete_and_earn_sort(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut freq: HashMap<i32, i64> = HashMap::new();
        for &v in &nums {
            *freq.entry(v).or_default() += v as i64;
        }
        let mut keys: Vec<i32> = freq.keys().copied().collect();
        keys.sort_unstable(); // O(k log k)

        let mut prev = 0i64;
        let mut curr = 0i64;
        for i in 0..keys.len() {
            let earn = freq[&keys[i]];
            // O(k) iterations total
            if i > 0 && keys[i] == keys[i - 1] + 1 {
                // Adjacent: house-robber choice
                let tmp = curr;
                curr = curr.max(prev + earn);
                prev = tmp;
            } else {
                // Gap: safe to take unconditionally
                prev = curr;
                curr += earn;
            }
        }
        curr as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn cases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![3, 4, 2], 6),
            (vec![2, 2, 3, 3, 3, 4], 9),
            (vec![1], 1),
            (vec![3, 3, 3], 9),
            (vec![1, 1, 1, 5, 5, 5], 18),
            (vec![1, 2], 2),
            (vec![1, 2, 3, 4], 6),
            (vec![3, 3, 3, 4], 9),
            (vec![10000], 10000),
            (vec![1, 1, 1, 1, 2, 3, 3, 3, 3], 16),
        ]
    }

    #[test]
    fn test_dp() {
        for (nums, expected) in cases() {
            assert_eq!(Solution::delete_and_earn(nums), expected);
        }
    }

    #[test]
    fn test_sort() {
        for (nums, expected) in cases() {
            assert_eq!(Solution::delete_and_earn_sort(nums), expected);
        }
    }
}
