/// leet 46

pub struct Solution;

impl Solution {
    /// Backtracking with swap. O(n*n!) time, O(n) space (recursion depth).
    /// At each level, swap the current index with each candidate, recurse, then swap back.
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = Vec::new();
        Self::backtrack(&mut nums, 0, &mut res);
        res
    }

    fn backtrack(nums: &mut Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            res.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(start, i);
            Self::backtrack(nums, start + 1, res);
            nums.swap(start, i);
        }
    }

    /// Iterative insertion. O(n*n!) time, O(1) extra space (not counting result).
    /// Build permutations incrementally: for each new number, insert it at every
    /// possible position in each existing permutation.
    pub fn permute_insert(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut perms: Vec<Vec<i32>> = vec![vec![]];
        for &num in &nums {
            let mut next = Vec::with_capacity(perms.len() * (perms[0].len() + 1));
            for perm in &perms {
                for pos in 0..=perm.len() {
                    let mut new_perm = perm.clone();
                    new_perm.insert(pos, num);
                    next.push(new_perm);
                }
            }
            perms = next;
        }
        perms
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for c in &mut v {
            c.sort();
        }
        v.sort();
        v
    }

    #[test]
    fn test_permute_backtrack() {
        assert_eq!(sorted(Solution::permute(vec![1, 2, 3])).len(), 6);
        assert_eq!(sorted(Solution::permute(vec![0, 1])).len(), 2);
        assert_eq!(sorted(Solution::permute(vec![1])).len(), 1);
        assert_eq!(sorted(Solution::permute(vec![-1, 0])).len(), 2);
        assert_eq!(sorted(Solution::permute(vec![1, 2, 3, 4])).len(), 24);
        assert_eq!(sorted(Solution::permute(vec![1, 2, 3, 4, 5, 6])).len(), 720);

        // Verify actual content for small case
        assert_eq!(
            sorted(Solution::permute(vec![1, 2, 3])),
            sorted(vec![
                vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3],
                vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1],
            ])
        );
    }

    #[test]
    fn test_permute_insert() {
        assert_eq!(sorted(Solution::permute_insert(vec![1, 2, 3])).len(), 6);
        assert_eq!(sorted(Solution::permute_insert(vec![0, 1])).len(), 2);
        assert_eq!(sorted(Solution::permute_insert(vec![1])).len(), 1);
        assert_eq!(sorted(Solution::permute_insert(vec![-1, 0])).len(), 2);
        assert_eq!(sorted(Solution::permute_insert(vec![1, 2, 3, 4])).len(), 24);
        assert_eq!(sorted(Solution::permute_insert(vec![1, 2, 3, 4, 5, 6])).len(), 720);

        // Both methods produce same results
        assert_eq!(
            sorted(Solution::permute(vec![1, 2, 3])),
            sorted(Solution::permute_insert(vec![1, 2, 3]))
        );
    }
}
