/// leet 2462

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    /// Two min-heaps approach. O(k log candidates) time, O(candidates) space.
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let n = costs.len();
        let k = k as usize;
        let candidates = candidates as usize;

        // If candidates * 2 >= n, all workers are always available — just pick k smallest.
        if candidates * 2 >= n {
            let mut sorted = costs.clone();
            sorted.sort_unstable();
            return sorted.iter().take(k).map(|&x| x as i64).sum();
        }

        // front heap: (cost, index), back heap: (cost, index)
        let mut front: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        let mut back: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        let mut lo = 0;
        let mut hi = n - 1;

        for _ in 0..candidates {
            front.push(Reverse((costs[lo], lo)));
            lo += 1;
        }
        for _ in 0..candidates {
            back.push(Reverse((costs[hi], hi)));
            hi -= 1;
        }
        // lo is next index to add to front, hi is next index to add to back
        // invariant: lo <= hi + 1

        let mut total: i64 = 0;
        for _ in 0..k {
            let pick_front = match (front.peek(), back.peek()) {
                (Some(&Reverse(f)), Some(&Reverse(b))) => f <= b,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => unreachable!(),
            };

            if pick_front {
                let Reverse((cost, _)) = front.pop().unwrap();
                total += cost as i64;
                if lo <= hi {
                    front.push(Reverse((costs[lo], lo)));
                    lo += 1;
                }
            } else {
                let Reverse((cost, _)) = back.pop().unwrap();
                total += cost as i64;
                if lo <= hi {
                    back.push(Reverse((costs[hi], hi)));
                    hi -= 1;
                }
            }
        }
        total
    }

    /// Single min-heap with side tracking. O(k log candidates) time.
    pub fn total_cost_v2(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let n = costs.len();
        let k = k as usize;
        let candidates = candidates as usize;

        if candidates * 2 >= n {
            let mut sorted = costs.clone();
            sorted.sort_unstable();
            return sorted.iter().take(k).map(|&x| x as i64).sum();
        }

        // Heap entries: (cost, index, is_front)
        // Use index to break ties: front (smaller index) wins.
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        let mut lo = 0usize;
        let mut hi = n - 1;

        for _ in 0..candidates {
            heap.push(Reverse((costs[lo], lo)));
            lo += 1;
        }
        for _ in 0..candidates {
            heap.push(Reverse((costs[hi], hi)));
            hi -= 1;
        }

        let mut total: i64 = 0;
        for _ in 0..k {
            let Reverse((cost, idx)) = heap.pop().unwrap();
            total += cost as i64;
            if lo <= hi {
                // Determine which side this came from to refill correctly
                if idx < lo {
                    // came from front side
                    heap.push(Reverse((costs[lo], lo)));
                    lo += 1;
                } else {
                    // came from back side
                    heap.push(Reverse((costs[hi], hi)));
                    hi -= 1;
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct Case {
        costs: Vec<i32>,
        k: i32,
        candidates: i32,
        expected: i64,
    }

    fn cases() -> Vec<Case> {
        vec![
            Case { costs: vec![17, 12, 10, 2, 7, 2, 11, 20, 8], k: 3, candidates: 4, expected: 11 },
            Case { costs: vec![1, 2, 4, 1], k: 3, candidates: 3, expected: 4 },
            Case { costs: vec![5], k: 1, candidates: 1, expected: 5 },
            Case { costs: vec![3, 3, 3, 3], k: 2, candidates: 2, expected: 6 },
            Case { costs: vec![10, 1, 10, 1], k: 2, candidates: 3, expected: 2 },
            Case { costs: vec![2, 1, 3], k: 3, candidates: 1, expected: 6 },
            Case { costs: vec![5, 4, 3, 2, 1], k: 3, candidates: 10, expected: 6 },
        ]
    }

    #[test]
    fn test_total_cost() {
        for (i, c) in cases().into_iter().enumerate() {
            assert_eq!(
                Solution::total_cost(c.costs, c.k, c.candidates),
                c.expected,
                "case {} failed",
                i
            );
        }
    }

    #[test]
    fn test_total_cost_v2() {
        for (i, c) in cases().into_iter().enumerate() {
            assert_eq!(
                Solution::total_cost_v2(c.costs, c.k, c.candidates),
                c.expected,
                "case {} (v2) failed",
                i
            );
        }
    }
}
