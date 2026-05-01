/// LeetCode 2064

pub struct Solution;

impl Solution {
    /// Binary search. O(n*log(k)) time, O(1) space.
    pub fn minimized_maximum(n: i32, quantities: &[i32]) -> i32 {
        let (mut left, mut right) = (1, 100000);
        while left < right {
            let mid = (left + right) / 2;
            let stores: i32 = quantities.iter().map(|&q| (q + mid - 1) / mid).sum();
            if stores > n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::minimized_maximum(6, &[11, 6]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::minimized_maximum(7, &[15, 10, 10]), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::minimized_maximum(1, &[100000]), 100000);
    }
}
