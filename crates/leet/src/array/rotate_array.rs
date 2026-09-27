/// leet 189

impl Solution {
    /// Triple reverse. O(n) time, O(1) space.
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 { return; }
        let k = k as usize % n;
        if k == 0 { return; }
        nums.reverse();           // reverse all
        nums[..k].reverse();      // reverse first k
        nums[k..].reverse();      // reverse last n-k
    }

    /// Extra array copy. O(n) time, O(n) space.
    pub fn rotate_extra(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 { return; }
        let k = k as usize % n;
        let copy = nums.clone(); // O(n) space
        for i in 0..n {          // O(n) time
            nums[(i + k) % n] = copy[i];
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_both(nums: &[i32], k: i32, expected: &[i32]) {
        let mut v1 = nums.to_vec();
        Solution::rotate(&mut v1, k);
        assert_eq!(v1, expected, "rotate failed for k={k}");

        let mut v2 = nums.to_vec();
        Solution::rotate_extra(&mut v2, k);
        assert_eq!(v2, expected, "rotate_extra failed for k={k}");
    }

    #[test]
    fn test_basic() {
        run_both(&[1,2,3,4,5,6,7], 3, &[5,6,7,1,2,3,4]);
        run_both(&[-1,-100,3,99], 2, &[3,99,-1,-100]);
    }

    #[test]
    fn test_single_element() {
        run_both(&[1], 0, &[1]);
        run_both(&[1], 1, &[1]);
    }

    #[test]
    fn test_two_elements() {
        run_both(&[1,2], 1, &[2,1]);
        run_both(&[1,2], 2, &[1,2]);
        run_both(&[1,2], 3, &[2,1]);
    }

    #[test]
    fn test_k_zero_and_full_rotation() {
        run_both(&[1,2,3,4,5,6,7], 0, &[1,2,3,4,5,6,7]);
        run_both(&[1,2,3,4,5,6,7], 7, &[1,2,3,4,5,6,7]);
    }

    #[test]
    fn test_k_greater_than_n() {
        run_both(&[1,2,3,4,5,6,7], 10, &[5,6,7,1,2,3,4]);
    }
}
