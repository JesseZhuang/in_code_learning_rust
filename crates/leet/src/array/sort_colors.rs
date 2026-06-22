pub struct Solution;

impl Solution {
    /// Dutch National Flag algorithm — three pointers.
    /// Time: O(n), Space: O(1). Single pass.
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }
        let mut lo: usize = 0; // boundary for 0s (next position to place a 0)
        let mut mid: usize = 0; // current element under examination
        let mut hi: usize = n - 1; // boundary for 2s (next position to place a 2)

        // Invariant: [0..lo) are 0s, [lo..mid) are 1s, (hi..n) are 2s
        while mid <= hi {
            match nums[mid] {
                0 => {
                    nums.swap(lo, mid);
                    lo += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                2 => {
                    nums.swap(mid, hi);
                    // Don't advance mid — swapped element needs inspection
                    if hi == 0 {
                        break;
                    }
                    hi -= 1;
                }
                _ => unreachable!(),
            }
        }
    }

    /// Counting sort variant.
    /// Time: O(n), Space: O(1) — only 3 counters.
    pub fn sort_colors_counting(nums: &mut Vec<i32>) {
        let mut counts = [0usize; 3];
        for &v in nums.iter() {
            counts[v as usize] += 1;
        }
        let mut idx = 0;
        for (color, &cnt) in counts.iter().enumerate() {
            for _ in 0..cnt {
                nums[idx] = color as i32;
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_both(input: &[i32], expected: &[i32]) {
        let mut v1 = input.to_vec();
        Solution::sort_colors(&mut v1);
        assert_eq!(v1, expected);

        let mut v2 = input.to_vec();
        Solution::sort_colors_counting(&mut v2);
        assert_eq!(v2, expected);
    }

    #[test]
    fn test_example1() {
        run_both(&[2, 0, 2, 1, 1, 0], &[0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_example2() {
        run_both(&[2, 0, 1], &[0, 1, 2]);
    }

    #[test]
    fn test_single_element() {
        run_both(&[0], &[0]);
        run_both(&[1], &[1]);
        run_both(&[2], &[2]);
    }

    #[test]
    fn test_all_same() {
        run_both(&[0, 0, 0], &[0, 0, 0]);
        run_both(&[1, 1, 1], &[1, 1, 1]);
        run_both(&[2, 2, 2], &[2, 2, 2]);
    }

    #[test]
    fn test_already_sorted() {
        run_both(&[0, 0, 1, 1, 2, 2], &[0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_reverse_sorted() {
        run_both(&[2, 2, 1, 1, 0, 0], &[0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_two_elements() {
        run_both(&[1, 0], &[0, 1]);
        run_both(&[2, 0], &[0, 2]);
        run_both(&[2, 1], &[1, 2]);
    }

    #[test]
    fn test_missing_colors() {
        run_both(&[0, 1, 0, 1], &[0, 0, 1, 1]);
        run_both(&[2, 0, 2, 0], &[0, 0, 2, 2]);
        run_both(&[1, 2, 1, 2], &[1, 1, 2, 2]);
    }
}
