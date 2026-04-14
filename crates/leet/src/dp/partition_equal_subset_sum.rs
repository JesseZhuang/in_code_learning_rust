/// leet 416

impl Solution {
    /// DP boolean vec. O(n*target) time, O(target) space.
    /// Iterate nums; for each num iterate backward from target to num.
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }
        let target = total as usize / 2;
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for num in nums {
            let num = num as usize;
            for j in (num..=target).rev() {
                dp[j] = dp[j] || dp[j - num];
            }
        }
        dp[target]
    }

    /// Bitwise bitset. O(n*target/64) time per num, O(target/64) space.
    /// Use Vec<u64> as a bitset. Set bit 0. For each num, OR the bitset
    /// with itself shifted left by num. Check if bit `target` is set.
    pub fn can_partition_bitset(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }
        let target = total as usize / 2;
        let words = target / 64 + 1;
        let mut bits = vec![0u64; words];
        bits[0] = 1; // bit 0 set

        for num in nums {
            let num = num as usize;
            let word_shift = num / 64;
            let bit_shift = num % 64;
            // OR bits with (bits << num), iterating from high to low
            for i in (0..words).rev() {
                let mut val = 0u64;
                if i >= word_shift {
                    let src = i - word_shift;
                    val = bits[src] << bit_shift;
                    if bit_shift > 0 && src > 0 {
                        val |= bits[src - 1] >> (64 - bit_shift);
                    }
                }
                bits[i] |= val;
            }
        }
        bits[target / 64] & (1u64 << (target % 64)) != 0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dp() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
        assert_eq!(Solution::can_partition(vec![1, 1]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 5]), false);
        assert_eq!(Solution::can_partition(vec![2, 2, 1, 1]), true);
        assert_eq!(Solution::can_partition(vec![1]), false);
        assert_eq!(Solution::can_partition(vec![100]), false);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(Solution::can_partition(vec![14, 9, 8, 4, 3, 2]), true);
    }

    #[test]
    fn test_bitset() {
        assert_eq!(Solution::can_partition_bitset(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition_bitset(vec![1, 2, 3, 5]), false);
        assert_eq!(Solution::can_partition_bitset(vec![1, 1]), true);
        assert_eq!(Solution::can_partition_bitset(vec![1, 2, 5]), false);
        assert_eq!(Solution::can_partition_bitset(vec![2, 2, 1, 1]), true);
        assert_eq!(Solution::can_partition_bitset(vec![1]), false);
        assert_eq!(Solution::can_partition_bitset(vec![100]), false);
        assert_eq!(Solution::can_partition_bitset(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(Solution::can_partition_bitset(vec![14, 9, 8, 4, 3, 2]), true);
    }
}
