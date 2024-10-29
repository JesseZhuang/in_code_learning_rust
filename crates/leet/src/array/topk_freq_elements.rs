use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut count, N) = (HashMap::new(), nums.len());
        for &n in nums.iter() { *count.entry(n).or_insert(0) += 1; }
        let mut buk = vec![Vec::new(); N + 1];
        for (&n, &cnt) in count.iter() { buk[cnt].push(n); }
        buk.into_iter().flatten().skip(count.len() - k as usize).collect()
    }
}

struct Solution;
