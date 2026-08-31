/// leet 40

impl Solution {
    /// Backtracking with sort + skip duplicates. O(2^n) time, O(n) space.
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort(); // sort to enable duplicate skipping
        let mut res = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut vec![], &mut res);
        res
    }

    fn backtrack(c: &[i32], remaining: i32, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            res.push(path.clone());
            return;
        }
        for i in start..c.len() { // O(n) branches per level
            if c[i] > remaining { break; } // prune: sorted
            if i > start && c[i] == c[i - 1] { continue; } // skip duplicates at same level
            path.push(c[i]);
            Self::backtrack(c, remaining - c[i], i + 1, path, res); // i+1: each used at most once
            path.pop();
        }
    }

    /// Counter-based backtracking using HashMap. O(2^n) time, O(n) space.
    pub fn combination_sum2_counter(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut counter: HashMap<i32, usize> = HashMap::new();
        for c in &candidates { // O(n) count frequencies
            *counter.entry(*c).or_insert(0) += 1;
        }
        let mut uniq: Vec<(i32, usize)> = counter.into_iter().collect();
        uniq.sort(); // sort for deterministic order
        let mut res = Vec::new();
        Self::bt_counter(&uniq, target, 0, &mut vec![], &mut res);
        res
    }

    fn bt_counter(
        uniq: &[(i32, usize)],
        remaining: i32,
        start: usize,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            res.push(path.clone());
            return;
        }
        for i in start..uniq.len() { // O(k) unique candidates
            let (val, count) = uniq[i];
            if val > remaining { break; } // prune: sorted
            for times in 1..=count { // use val 1..count times
                if val * times as i32 > remaining { break; }
                path.push(val);
                Self::bt_counter(uniq, remaining - val * times as i32, i + 1, path, res);
            }
            for _ in 0..count.min((remaining / val) as usize) { // pop all pushed
                path.pop();
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for c in &mut v { c.sort(); }
        v.sort();
        v
    }

    #[test]
    fn test_backtrack() {
        assert_eq!(sorted(vec![vec![1,1,6],vec![1,2,5],vec![1,7],vec![2,6]]), sorted(Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8)));
        assert_eq!(sorted(vec![vec![1,2,2],vec![5]]), sorted(Solution::combination_sum2(vec![2,5,2,1,2], 5)));
        assert_eq!(sorted(vec![] as Vec<Vec<i32>>), sorted(Solution::combination_sum2(vec![2,4,6], 1)));
        assert_eq!(sorted(vec![vec![1]]), sorted(Solution::combination_sum2(vec![1], 1)));
        assert_eq!(sorted(vec![vec![1,1,1]]), sorted(Solution::combination_sum2(vec![1,1,1,1,1], 3)));
        assert_eq!(sorted(vec![vec![3,3,3]]), sorted(Solution::combination_sum2(vec![3,3,3], 9)));
        assert_eq!(sorted(vec![vec![1,1,2],vec![2,2]]), sorted(Solution::combination_sum2(vec![1,1,1,2,2], 4)));
    }

    #[test]
    fn test_counter() {
        assert_eq!(sorted(vec![vec![1,1,6],vec![1,2,5],vec![1,7],vec![2,6]]), sorted(Solution::combination_sum2_counter(vec![10,1,2,7,6,1,5], 8)));
        assert_eq!(sorted(vec![vec![1,2,2],vec![5]]), sorted(Solution::combination_sum2_counter(vec![2,5,2,1,2], 5)));
        assert_eq!(sorted(vec![] as Vec<Vec<i32>>), sorted(Solution::combination_sum2_counter(vec![2,4,6], 1)));
        assert_eq!(sorted(vec![vec![1]]), sorted(Solution::combination_sum2_counter(vec![1], 1)));
        assert_eq!(sorted(vec![vec![1,1,1]]), sorted(Solution::combination_sum2_counter(vec![1,1,1,1,1], 3)));
        assert_eq!(sorted(vec![vec![3,3,3]]), sorted(Solution::combination_sum2_counter(vec![3,3,3], 9)));
        assert_eq!(sorted(vec![vec![1,1,2],vec![2,2]]), sorted(Solution::combination_sum2_counter(vec![1,1,1,2,2], 4)));
    }
}
