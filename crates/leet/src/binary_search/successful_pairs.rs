/// LeetCode 2300 - Successful Pairs of Spells and Potions
///
/// Given two arrays `spells` and `potions` and an integer `success`,
/// a pair (i, j) is successful if spells[i] * potions[j] >= success.
/// Return an array where answer[i] is the number of potions that form
/// a successful pair with spell i.

pub struct Solution;

impl Solution {
    /// Sort potions, then binary search for each spell.
    /// O((m+n) log n) time, O(n) extra space for sorting.
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        let n = potions.len() as i32;

        spells
            .iter()
            .map(|&spell| {
                // Find the smallest potion p such that spell * p >= success
                // i.e. p >= ceil(success / spell)
                let spell = spell as i64;
                // Minimum potion value needed (ceiling division)
                let min_potion = (success + spell - 1) / spell;

                // Binary search: find first index where potions[idx] >= min_potion
                let idx = potions.partition_point(|&p| (p as i64) < min_potion);
                n - idx as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }

    #[test]
    fn test_single_spell_success() {
        // spell=10, potions=[1], success=5 -> 10*1=10>=5 -> [1]
        assert_eq!(
            Solution::successful_pairs(vec![10], vec![1], 5),
            vec![1]
        );
    }

    #[test]
    fn test_single_spell_fail() {
        // spell=1, potions=[1], success=5 -> 1*1=1<5 -> [0]
        assert_eq!(
            Solution::successful_pairs(vec![1], vec![1], 5),
            vec![0]
        );
    }

    #[test]
    fn test_large_values() {
        // 100000 * 100000 = 10^10, success = 10^10 -> pair succeeds
        assert_eq!(
            Solution::successful_pairs(vec![100000], vec![100000], 10_000_000_000),
            vec![1]
        );
        // success just above -> fails
        assert_eq!(
            Solution::successful_pairs(vec![100000], vec![100000], 10_000_000_001),
            vec![0]
        );
    }

    #[test]
    fn test_spell_one() {
        // spell=1, potions=[1,2,3,4,5], success=3 -> need potion>=3 -> [3,4,5] -> 3
        assert_eq!(
            Solution::successful_pairs(vec![1], vec![1, 2, 3, 4, 5], 3),
            vec![3]
        );
    }
}
