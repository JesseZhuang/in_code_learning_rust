use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Bellman-Ford variant: relax all edges up to k+1 times.
    /// Time O((k+1) * E), Space O(n).
    pub fn find_cheapest_price_bf(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut prices = vec![i32::MAX; n];
        prices[src as usize] = 0;

        for _ in 0..=k { // O(k+1) rounds
            let mut tmp = prices.clone();
            for f in &flights { // O(E) edges per round
                let (u, v, w) = (f[0] as usize, f[1] as usize, f[2]);
                if prices[u] != i32::MAX && prices[u] + w < tmp[v] {
                    tmp[v] = prices[u] + w;
                }
            }
            prices = tmp;
        }
        if prices[dst as usize] == i32::MAX { -1 } else { prices[dst as usize] }
    }

    /// BFS with pruning: level-order BFS where level = number of stops.
    /// Time O((k+1) * E), Space O(n + E).
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for f in &flights {
            graph[f[0] as usize].push((f[1] as usize, f[2]));
        }

        let mut dist = vec![i32::MAX; n];
        dist[src as usize] = 0;
        let mut q = VecDeque::new();
        q.push_back((src as usize, 0i32));
        let mut stops = 0;

        while !q.is_empty() && stops <= k {
            let size = q.len();
            for _ in 0..size {
                let (node, cost) = q.pop_front().unwrap();
                for &(nei, w) in &graph[node] {
                    let new_cost = cost + w;
                    if new_cost < dist[nei] {
                        dist[nei] = new_cost;
                        q.push_back((nei, new_cost));
                    }
                }
            }
            stops += 1;
        }
        if dist[dst as usize] == i32::MAX { -1 } else { dist[dst as usize] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let flights = vec![vec![0,1,100], vec![1,2,100], vec![2,0,100], vec![1,3,600], vec![2,3,200]];
        assert_eq!(Solution::find_cheapest_price_bf(4, flights.clone(), 0, 3, 1), 700);
        assert_eq!(Solution::find_cheapest_price(4, flights, 0, 3, 1), 700);
    }

    #[test]
    fn test_example2() {
        let flights = vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]];
        assert_eq!(Solution::find_cheapest_price_bf(3, flights.clone(), 0, 2, 1), 200);
        assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 1), 200);
    }

    #[test]
    fn test_zero_stops() {
        let flights = vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]];
        assert_eq!(Solution::find_cheapest_price_bf(3, flights.clone(), 0, 2, 0), 500);
        assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 0), 500);
    }

    #[test]
    fn test_no_path() {
        let flights = vec![vec![0,1,100]];
        assert_eq!(Solution::find_cheapest_price_bf(3, flights.clone(), 0, 2, 1), -1);
        assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 1), -1);
    }

    #[test]
    fn test_direct_flight() {
        let flights = vec![vec![0,1,50]];
        assert_eq!(Solution::find_cheapest_price_bf(2, flights.clone(), 0, 1, 0), 50);
        assert_eq!(Solution::find_cheapest_price(2, flights, 0, 1, 0), 50);
    }

    #[test]
    fn test_k_limits_cheaper_path() {
        let flights = vec![vec![0,1,100], vec![1,2,100], vec![2,3,100], vec![1,3,500]];
        assert_eq!(Solution::find_cheapest_price_bf(4, flights.clone(), 0, 3, 1), 600);
        assert_eq!(Solution::find_cheapest_price(4, flights.clone(), 0, 3, 1), 600);
        assert_eq!(Solution::find_cheapest_price_bf(4, flights.clone(), 0, 3, 2), 300);
        assert_eq!(Solution::find_cheapest_price(4, flights, 0, 3, 2), 300);
    }
}
