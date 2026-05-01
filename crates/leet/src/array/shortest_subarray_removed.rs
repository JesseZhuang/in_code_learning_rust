/// LeetCode 1574

pub struct Solution;

impl Solution {
    /// Two pointers. O(n) time, O(1) space.
    pub fn find_length_of_shortest_subarray(arr: &[i32]) -> i32 {
        let n = arr.len();
        let mut r = n - 1;
        while r > 0 && arr[r] >= arr[r - 1] {
            r -= 1;
        }
        let mut res = r as i32;
        let mut l = 0usize;
        while l < r && (l == 0 || arr[l - 1] <= arr[l]) {
            while r < n && arr[l] > arr[r] {
                r += 1;
            }
            res = res.min((r - l - 1) as i32);
            l += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_length_of_shortest_subarray(&[1, 2, 3, 10, 4, 2, 3, 5]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_length_of_shortest_subarray(&[5, 4, 3, 2, 1]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_length_of_shortest_subarray(&[1, 2, 3]), 0);
    }

    #[test]
    fn single() {
        assert_eq!(Solution::find_length_of_shortest_subarray(&[1]), 0);
    }

    #[test]
    fn decreasing_halves() {
        assert_eq!(Solution::find_length_of_shortest_subarray(&[2, 2, 2, 1, 1, 1]), 3);
    }
}
