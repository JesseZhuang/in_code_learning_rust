pub struct Solution;

impl Solution {
    /// LeetCode 70: rolling DP in O(n) time and O(1) extra space.
    pub fn climb_stairs(n: i32) -> i32 {
        // One way to reach step 0 (stay put) and one way to reach step 1.
        let (mut prev, mut curr) = (1, 1);
        for _ in 2..=n {
            // The final move comes from either one or two steps below.
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn one_stair() {
        assert_eq!(1, Solution::climb_stairs(1));
    }

    #[test]
    fn two_stairs() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn three_stairs() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn maximum_stairs() {
        assert_eq!(1_836_311_903, Solution::climb_stairs(45));
    }
}
