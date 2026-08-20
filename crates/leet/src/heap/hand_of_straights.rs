// lc 846 - Hand of Straights

pub struct Solution;

impl Solution {
    /// Greedy with BTreeMap.
    /// Time O(n log n), Space O(n).
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        use std::collections::BTreeMap;
        let n = hand.len();
        let g = group_size as usize;
        if n % g != 0 {
            return false;
        }
        let mut counts: BTreeMap<i32, usize> = BTreeMap::new();
        for &card in &hand {
            *counts.entry(card).or_insert(0) += 1;
        }
        while let Some((&start, _)) = counts.iter().next() {
            for i in 0..group_size {
                let key = start + i;
                match counts.get_mut(&key) {
                    Some(cnt) if *cnt > 1 => *cnt -= 1,
                    Some(cnt) if *cnt == 1 => { counts.remove(&key); }
                    _ => return false,
                }
            }
        }
        true
    }

    /// Sort + HashMap.
    /// Time O(n log n), Space O(n).
    pub fn is_n_straight_hand_heap(hand: Vec<i32>, group_size: i32) -> bool {
        use std::collections::HashMap;
        let n = hand.len();
        let g = group_size as usize;
        if n % g != 0 {
            return false;
        }
        let mut sorted = hand;
        sorted.sort_unstable();
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for &card in &sorted {
            *counts.entry(card).or_insert(0) += 1;
        }
        for &card in &sorted {
            if *counts.get(&card).unwrap_or(&0) == 0 {
                continue;
            }
            for i in 0..group_size {
                let key = card + i;
                match counts.get_mut(&key) {
                    Some(cnt) if *cnt > 0 => *cnt -= 1,
                    _ => return false,
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(Solution::is_n_straight_hand(vec![1,2,3,6,2,3,4,7,8], 3));
        assert!(Solution::is_n_straight_hand_heap(vec![1,2,3,6,2,3,4,7,8], 3));
    }

    #[test]
    fn example2() {
        assert!(!Solution::is_n_straight_hand(vec![1,2,3,4,5], 4));
        assert!(!Solution::is_n_straight_hand_heap(vec![1,2,3,4,5], 4));
    }

    #[test]
    fn single_group() {
        assert!(Solution::is_n_straight_hand(vec![1,2,3], 3));
        assert!(Solution::is_n_straight_hand_heap(vec![1,2,3], 3));
    }

    #[test]
    fn single_card_groups() {
        assert!(Solution::is_n_straight_hand(vec![5,3,1,2], 1));
        assert!(Solution::is_n_straight_hand_heap(vec![5,3,1,2], 1));
    }

    #[test]
    fn duplicates() {
        assert!(Solution::is_n_straight_hand(vec![1,1,2,2,3,3], 3));
        assert!(Solution::is_n_straight_hand_heap(vec![1,1,2,2,3,3], 3));
    }

    #[test]
    fn gap() {
        assert!(!Solution::is_n_straight_hand(vec![1,3,5,7], 2));
        assert!(!Solution::is_n_straight_hand_heap(vec![1,3,5,7], 2));
    }

    #[test]
    fn indivisible_length() {
        assert!(!Solution::is_n_straight_hand(vec![1,2,3,4], 3));
        assert!(!Solution::is_n_straight_hand_heap(vec![1,2,3,4], 3));
    }
}
