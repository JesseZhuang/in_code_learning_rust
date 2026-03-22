// lc 1298
use std::collections::VecDeque;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();
        let mut has_box = vec![false; n];
        let mut opened = vec![false; n];
        let mut has_key: Vec<bool> = status.iter().map(|&s| s == 1).collect();
        let mut res = 0;
        let mut q = VecDeque::new();

        for &b in &initial_boxes {
            let b = b as usize;
            has_box[b] = true;
            if has_key[b] {
                q.push_back(b);
                opened[b] = true;
            }
        }

        while let Some(box_id) = q.pop_front() {
            res += candies[box_id];
            for &k in &keys[box_id] {
                let k = k as usize;
                if !has_key[k] {
                    has_key[k] = true;
                    if has_box[k] && !opened[k] {
                        opened[k] = true;
                        q.push_back(k);
                    }
                }
            }
            for &b in &contained_boxes[box_id] {
                let b = b as usize;
                has_box[b] = true;
                if !opened[b] && has_key[b] {
                    opened[b] = true;
                    q.push_back(b);
                }
            }
        }

        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::graph::max_candies_boxes::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec![vec![], vec![], vec![1], vec![]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
                vec![0],
            ),
            16
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
                vec![0],
            ),
            6
        );
    }

    #[test]
    fn no_initial() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 1],
                vec![10, 20],
                vec![vec![], vec![]],
                vec![vec![], vec![]],
                vec![],
            ),
            0
        );
    }

    #[test]
    fn single_open() {
        assert_eq!(
            Solution::max_candies(
                vec![1],
                vec![42],
                vec![vec![]],
                vec![vec![]],
                vec![0],
            ),
            42
        );
    }

    #[test]
    fn single_closed() {
        assert_eq!(
            Solution::max_candies(
                vec![0],
                vec![42],
                vec![vec![]],
                vec![vec![]],
                vec![0],
            ),
            0
        );
    }

    #[test]
    fn chain() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0],
                vec![1, 2, 3],
                vec![vec![1], vec![2], vec![]],
                vec![vec![1], vec![2], vec![]],
                vec![0],
            ),
            6
        );
    }

    #[test]
    fn key_before_box() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1],
                vec![10, 100, 1],
                vec![vec![1], vec![], vec![]],
                vec![vec![], vec![], vec![1]],
                vec![0, 2],
            ),
            111
        );
    }

    #[test]
    fn unreachable() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0],
                vec![5, 10, 100],
                vec![vec![], vec![], vec![]],
                vec![vec![1], vec![], vec![]],
                vec![0],
            ),
            5
        );
    }

    #[test]
    fn duplicate_keys() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 1, 0],
                vec![1, 2, 3],
                vec![vec![2], vec![2], vec![]],
                vec![vec![1], vec![2], vec![]],
                vec![0],
            ),
            6
        );
    }

    #[test]
    fn circular_closed() {
        assert_eq!(
            Solution::max_candies(
                vec![0, 0],
                vec![10, 20],
                vec![vec![1], vec![0]],
                vec![vec![], vec![]],
                vec![0, 1],
            ),
            0
        );
    }
}
