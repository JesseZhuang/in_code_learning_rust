use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    /// BFS approach
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<String> = deadends.into_iter().collect();
        let start = "0000".to_string();
        if dead.contains(&start) {
            return -1;
        }
        if target == start {
            return 0;
        }

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.clone());
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((curr, steps)) = queue.pop_front() {
            for next in Self::neighbors(&curr) {
                if next == target {
                    return steps + 1;
                }
                if !dead.contains(&next) && !visited.contains(&next) {
                    visited.insert(next.clone());
                    queue.push_back((next, steps + 1));
                }
            }
        }
        -1
    }

    fn neighbors(state: &str) -> Vec<String> {
        let digits: Vec<u8> = state.bytes().map(|b| b - b'0').collect();
        let mut result = Vec::with_capacity(8);
        for i in 0..4 {
            let mut up = digits.clone();
            up[i] = (up[i] + 1) % 10;
            result.push(up.iter().map(|d| (d + b'0') as char).collect());

            let mut down = digits.clone();
            down[i] = (down[i] + 9) % 10;
            result.push(down.iter().map(|d| (d + b'0') as char).collect());
        }
        result
    }

    /// Bidirectional BFS approach
    pub fn open_lock_bidirectional(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<String> = deadends.into_iter().collect();
        let start = "0000".to_string();
        if dead.contains(&start) {
            return -1;
        }
        if target == start {
            return 0;
        }
        if dead.contains(&target) {
            return -1;
        }

        let mut front: HashSet<String> = HashSet::new();
        let mut back: HashSet<String> = HashSet::new();
        let mut visited: HashSet<String> = HashSet::new();

        front.insert(start.clone());
        back.insert(target.clone());
        visited.insert(start);
        visited.insert(target);

        let mut steps = 0;

        while !front.is_empty() && !back.is_empty() {
            // Always expand the smaller frontier
            if front.len() > back.len() {
                std::mem::swap(&mut front, &mut back);
            }

            let mut next_front: HashSet<String> = HashSet::new();
            for curr in &front {
                for next in Self::neighbors(curr) {
                    if back.contains(&next) {
                        return steps + 1;
                    }
                    if !dead.contains(&next) && !visited.contains(&next) {
                        visited.insert(next.clone());
                        next_front.insert(next);
                    }
                }
            }
            front = next_front;
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|&x| x.to_string()).collect()
    }

    #[test]
    fn example1() {
        assert_eq!(
            Solution::open_lock(s(&["0201", "0101", "0102", "1212", "2002"]), "0202".to_string()),
            6
        );
        assert_eq!(
            Solution::open_lock_bidirectional(s(&["0201", "0101", "0102", "1212", "2002"]), "0202".to_string()),
            6
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::open_lock(s(&["8888"]), "0009".to_string()), 1);
        assert_eq!(Solution::open_lock_bidirectional(s(&["8888"]), "0009".to_string()), 1);
    }

    #[test]
    fn unreachable() {
        // All neighbors of "0000" are dead except we need to reach 8888
        let deadends = s(&[
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ]);
        assert_eq!(Solution::open_lock(deadends.clone(), "8888".to_string()), -1);
        assert_eq!(Solution::open_lock_bidirectional(deadends, "8888".to_string()), -1);
    }

    #[test]
    fn start_is_deadend() {
        assert_eq!(Solution::open_lock(s(&["0000"]), "8888".to_string()), -1);
        assert_eq!(Solution::open_lock_bidirectional(s(&["0000"]), "8888".to_string()), -1);
    }

    #[test]
    fn target_is_start() {
        assert_eq!(Solution::open_lock(s(&["1234"]), "0000".to_string()), 0);
        assert_eq!(Solution::open_lock_bidirectional(s(&["1234"]), "0000".to_string()), 0);
    }

    #[test]
    fn single_turn() {
        assert_eq!(Solution::open_lock(vec![], "1000".to_string()), 1);
        assert_eq!(Solution::open_lock_bidirectional(vec![], "1000".to_string()), 1);
    }

    #[test]
    fn wraparound() {
        assert_eq!(Solution::open_lock(vec![], "9999".to_string()), 4);
        assert_eq!(Solution::open_lock_bidirectional(vec![], "9999".to_string()), 4);
    }
}
