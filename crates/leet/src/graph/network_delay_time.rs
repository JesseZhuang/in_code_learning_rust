use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n + 1];
        for edge in &times {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        let mut dist = vec![i32::MAX; n + 1];
        dist[k as usize] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, k as usize)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }

        let ans = dist[1..=n].iter().copied().max().unwrap();
        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_delay_time() {
        assert_eq!(
            Solution::network_delay_time(vec![vec![2,1,1], vec![2,3,1], vec![3,4,1]], 4, 2),
            2
        );
        assert_eq!(
            Solution::network_delay_time(vec![vec![1,2,1]], 2, 1),
            1
        );
        assert_eq!(
            Solution::network_delay_time(vec![vec![1,2,1]], 2, 2),
            -1
        );
        assert_eq!(
            Solution::network_delay_time(vec![vec![1,2,1], vec![1,3,4], vec![2,3,2]], 3, 1),
            3
        );
        assert_eq!(
            Solution::network_delay_time(vec![vec![1,2,1], vec![3,2,1]], 3, 1),
            -1
        );
    }
}
