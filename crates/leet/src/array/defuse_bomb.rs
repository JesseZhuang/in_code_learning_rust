/// LeetCode 1652

pub struct Solution;

impl Solution {
    /// Rolling sum. O(n) time, O(1) space (excluding result).
    pub fn decrypt(code: &[i32], k: i32) -> Vec<i32> {
        let n = code.len();
        let mut res = vec![0i32; n];
        if k == 0 {
            return res;
        }
        let (mut start, mut end): (i32, i32) = if k > 0 {
            (1, k)
        } else {
            (n as i32 + k, n as i32 - 1)
        };
        let mut sum: i32 = (start..=end).map(|i| code[i as usize % n]).sum();
        for i in 0..n {
            res[i] = sum;
            sum -= code[start as usize % n];
            start += 1;
            end += 1;
            sum += code[end as usize % n];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::decrypt(&[5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::decrypt(&[1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::decrypt(&[2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
