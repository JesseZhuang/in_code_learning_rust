pub struct Solution;

impl Solution {
    /// Binary search on the shorter array.
    /// Time: O(log(min(m, n))), Space: O(1)
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Ensure nums1 is the shorter array for O(log(min(m,n))) binary search
        let (a, b) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let (m, n) = (a.len(), b.len());
        let half = (m + n + 1) / 2; // left partition size

        let mut lo: usize = 0;
        let mut hi: usize = m;

        while lo <= hi {
            let i = (lo + hi) / 2; // partition index in a
            let j = half - i; // partition index in b (derived)

            // O(1) comparisons per iteration
            let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
            let a_right = if i == m { i32::MAX } else { a[i] };
            let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
            let b_right = if j == n { i32::MAX } else { b[j] };

            if a_left <= b_right && b_left <= a_right {
                // Found correct partition
                return if (m + n) % 2 == 1 {
                    a_left.max(b_left) as f64
                } else {
                    (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0
                };
            } else if a_left > b_right {
                hi = i - 1; // move partition left in a
            } else {
                lo = i + 1; // move partition right in a
            }
        }
        unreachable!()
    }

    /// Merge approach.
    /// Time: O(m + n), Space: O(m + n)
    pub fn find_median_sorted_arrays_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let mut merged = Vec::with_capacity(total); // O(m+n) space allocation

        let (mut i, mut j) = (0, 0);
        // O(m+n) merge pass
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        while i < nums1.len() {
            merged.push(nums1[i]);
            i += 1;
        }
        while j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        }

        let mid = total / 2;
        if total % 2 == 1 {
            merged[mid] as f64
        } else {
            (merged[mid - 1] as f64 + merged[mid] as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5]), 3.0);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-5, -3, -1], vec![0, 2, 4]),
            -0.5
        );
    }

    #[test]
    fn test_merge() {
        assert_eq!(Solution::find_median_sorted_arrays_merge(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays_merge(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays_merge(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays_merge(vec![2], vec![]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays_merge(vec![1], vec![2]), 1.5);
        assert_eq!(Solution::find_median_sorted_arrays_merge(vec![1, 2], vec![3, 4, 5]), 3.0);
        assert_eq!(
            Solution::find_median_sorted_arrays_merge(vec![-5, -3, -1], vec![0, 2, 4]),
            -0.5
        );
    }
}
