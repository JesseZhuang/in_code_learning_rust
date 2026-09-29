/// LeetCode 1094 - Car Pooling
struct Solution;

impl Solution {
    /// Difference array approach.
    /// Time O(n + 1001), Space O(1001)
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut diff = vec![0i32; 1001];
        for trip in &trips { // O(n)
            let (num, from, to) = (trip[0], trip[1] as usize, trip[2] as usize);
            diff[from] += num;
            diff[to] -= num;
        }
        let mut current = 0;
        for d in &diff { // O(1001)
            current += d;
            if current > capacity {
                return false;
            }
        }
        true
    }

    /// Sorted events sweep approach.
    /// Time O(n log n), Space O(n)
    pub fn car_pooling_events(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut events: Vec<(i32, i32)> = Vec::with_capacity(trips.len() * 2);
        for trip in &trips { // O(n)
            events.push((trip[1], trip[0]));
            events.push((trip[2], -trip[0]));
        }
        events.sort(); // O(n log n)
        let mut current = 0;
        for (_, delta) in &events { // O(n)
            current += delta;
            if current > capacity {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_pooling() {
        let cases: Vec<(Vec<Vec<i32>>, i32, bool)> = vec![
            (vec![vec![2, 1, 5], vec![3, 3, 7]], 4, false),
            (vec![vec![2, 1, 5], vec![3, 3, 7]], 5, true),
            (vec![vec![2, 1, 5], vec![3, 5, 7]], 3, true),
            (vec![vec![3, 2, 7]], 3, true),
            (vec![vec![3, 2, 7]], 2, false),
            (vec![vec![5, 0, 3], vec![5, 3, 6]], 5, true),
            (vec![vec![2, 0, 5], vec![3, 0, 5]], 5, true),
            (vec![vec![2, 0, 5], vec![3, 0, 5]], 4, false),
            (vec![vec![3, 0, 2], vec![3, 2, 4]], 3, true),
            (vec![], 1, true),
        ];
        for (trips, cap, expected) in &cases {
            assert_eq!(
                Solution::car_pooling(trips.clone(), *cap),
                *expected,
                "diff array: trips={:?}, cap={}",
                trips,
                cap
            );
            assert_eq!(
                Solution::car_pooling_events(trips.clone(), *cap),
                *expected,
                "events: trips={:?}, cap={}",
                trips,
                cap
            );
        }
    }
}
