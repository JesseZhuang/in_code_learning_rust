/// LeetCode 658

pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let (mut l, mut r) = (0usize, arr.len() - k);
        while l < r {
            let mid = l + (r - l) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        arr[l..l + k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::array::find_k_closest_elements::Solution;

    #[test]
    fn test_find_closest_elements() {
        assert_eq!(vec![1, 2, 3, 4], Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3));
        assert_eq!(vec![1, 1, 2, 3], Solution::find_closest_elements(vec![1, 1, 2, 3, 4, 5], 4, -1));
    }
}
