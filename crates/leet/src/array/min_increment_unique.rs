pub struct Solution;

impl Solution {
    /// Sort + Greedy approach.
    /// Time: O(n log n), Space: O(1) (in-place sort).
    pub fn min_increment_for_unique(nums: &mut Vec<i32>) -> i32 {
        nums.sort(); // O(n log n)
        let mut moves = 0i32;
        for i in 1..nums.len() {
            // If current <= previous, bump it to previous + 1
            if nums[i] <= nums[i - 1] {
                let need = nums[i - 1] + 1;
                moves += need - nums[i]; // accumulate the increment cost
                nums[i] = need;
            }
        }
        moves
    }

    /// Counting Sort approach.
    /// Time: O(n + max_val), Space: O(max_val).
    pub fn min_increment_for_unique2(nums: &[i32]) -> i32 {
        let max_val = *nums.iter().max().unwrap_or(&0) as usize;
        // Worst case: all elements equal max_val, each pushed +1, so need max_val + n slots
        let size = max_val + nums.len() + 1; // O(max_val + n) space
        let mut count = vec![0i32; size];
        for &v in nums {
            count[v as usize] += 1; // frequency table — O(n)
        }
        let mut moves = 0i32;
        for i in 0..size - 1 {
            if count[i] > 1 {
                let extras = count[i] - 1; // duplicates to push forward
                count[i + 1] += extras; // carry extras to next slot — O(1) per slot
                moves += extras; // each extra costs 1 move (incremented by 1)
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    /// Run both solutions and assert they agree on the expected answer.
    fn run_both(input: &[i32], expected: i32) {
        let mut v = input.to_vec();
        assert_eq!(Solution::min_increment_for_unique(&mut v), expected);
        assert_eq!(Solution::min_increment_for_unique2(input), expected);
    }

    #[test]
    fn test_example1() {
        // [1,1,2] → [1,2,3], moves = 1+1 = 3? No: sort → [1,1,2] → bump 1→2, 2→3 = 1+1 = 3
        // LeetCode example: [1,2,2] → 1
        run_both(&[1, 2, 2], 1);
    }

    #[test]
    fn test_example2() {
        // [3,2,1,2,1,7] → sorted [1,1,2,2,3,7] → [1,2,3,4,5,7] moves = 1+1+2+2 = 6
        run_both(&[3, 2, 1, 2, 1, 7], 6);
    }

    #[test]
    fn test_single_element() {
        run_both(&[0], 0);
        run_both(&[100_000], 0);
    }

    #[test]
    fn test_already_unique() {
        run_both(&[1, 2, 3, 4, 5], 0);
    }

    #[test]
    fn test_all_same() {
        // [3,3,3] → [3,4,5], moves = 0+1+2 = 3
        run_both(&[3, 3, 3], 3);
    }

    #[test]
    fn test_all_zeros() {
        // [0,0,0,0] → [0,1,2,3], moves = 1+2+3 = 6
        run_both(&[0, 0, 0, 0], 6);
    }

    #[test]
    fn test_two_elements_same() {
        run_both(&[5, 5], 1);
    }

    #[test]
    fn test_descending() {
        // [5,4,3,2,1] → sorted [1,2,3,4,5] already unique → 0
        run_both(&[5, 4, 3, 2, 1], 0);
    }

    #[test]
    fn test_large_gap() {
        // [0,0,100000] → [0,1,100000], moves = 1
        run_both(&[0, 0, 100_000], 1);
    }
}
