/// LeetCode 2251 - Number of Flowers in Full Bloom
///
/// Given flowers[i] = [start_i, end_i] meaning flower i blooms from day start_i
/// to day end_i (inclusive), and people[i] = the day person i arrives, return an
/// array where answer[i] = number of flowers in full bloom when person i arrives.

pub struct Solution;

impl Solution {
    /// Binary Search approach: sort starts and ends separately.
    /// For each person at time t: count = (starts <= t) - (ends < t).
    /// O((n+q) log n) time, O(n) space where n = flowers.len(), q = people.len().
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let n = flowers.len();
        let mut starts = Vec::with_capacity(n);
        let mut ends = Vec::with_capacity(n);

        for f in &flowers {
            starts.push(f[0]);
            ends.push(f[1]);
        }

        // O(n log n) sorting
        starts.sort_unstable();
        ends.sort_unstable();

        // O(q log n) — binary search for each person
        people
            .iter()
            .map(|&t| {
                // Number of flowers that have started blooming by time t
                let started = starts.partition_point(|&s| s <= t) as i32;
                // Number of flowers that have finished blooming before time t
                // (ended strictly before t, i.e. end < t)
                let ended = ends.partition_point(|&e| e < t) as i32;
                started - ended
            })
            .collect()
    }

    /// Sweep Line approach: create +1/-1 events, sort queries with original index.
    /// O((n+q) log(n+q)) time, O(n+q) space.
    pub fn full_bloom_flowers_sweep(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        // Build events: (day, delta). Start adds a flower, end+1 removes it.
        // O(n) space for events
        let mut events: Vec<(i32, i32)> = Vec::with_capacity(flowers.len() * 2);
        for f in &flowers {
            events.push((f[0], 1));
            events.push((f[1] + 1, -1));
        }

        // O(n log n) sort events by day (ties broken by delta so -1 before +1 at same day)
        events.sort_unstable();

        // O(q) space — pair each person with their original index
        let mut queries: Vec<(i32, usize)> = people.iter().enumerate().map(|(i, &t)| (t, i)).collect();
        // O(q log q) sort queries by time
        queries.sort_unstable();

        let mut result = vec![0i32; people.len()];
        let mut bloom_count = 0i32;
        let mut ei = 0; // event index

        // O(n + q) sweep — each event and query visited at most once
        for (time, orig_idx) in queries {
            while ei < events.len() && events[ei].0 <= time {
                bloom_count += events[ei].1;
                ei += 1;
            }
            result[orig_idx] = bloom_count;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];
        assert_eq!(Solution::full_bloom_flowers(flowers.clone(), people.clone()), vec![1, 2, 2, 2]);
        assert_eq!(Solution::full_bloom_flowers_sweep(flowers, people), vec![1, 2, 2, 2]);
    }

    #[test]
    fn test_example2() {
        let flowers = vec![vec![1, 10], vec![3, 3]];
        let people = vec![3, 3, 2];
        assert_eq!(Solution::full_bloom_flowers(flowers.clone(), people.clone()), vec![2, 2, 1]);
        assert_eq!(Solution::full_bloom_flowers_sweep(flowers, people), vec![2, 2, 1]);
    }

    #[test]
    fn test_single_flower_single_person() {
        let flowers = vec![vec![5, 10]];
        let people = vec![4, 5, 7, 10, 11];
        assert_eq!(Solution::full_bloom_flowers(flowers.clone(), people.clone()), vec![0, 1, 1, 1, 0]);
        assert_eq!(Solution::full_bloom_flowers_sweep(flowers, people), vec![0, 1, 1, 1, 0]);
    }

    #[test]
    fn test_no_overlap() {
        let flowers = vec![vec![1, 2], vec![4, 5]];
        let people = vec![3];
        assert_eq!(Solution::full_bloom_flowers(flowers.clone(), people.clone()), vec![0]);
        assert_eq!(Solution::full_bloom_flowers_sweep(flowers, people), vec![0]);
    }

    #[test]
    fn test_all_overlap() {
        let flowers = vec![vec![1, 10], vec![1, 10], vec![1, 10]];
        let people = vec![1, 5, 10];
        assert_eq!(Solution::full_bloom_flowers(flowers.clone(), people.clone()), vec![3, 3, 3]);
        assert_eq!(Solution::full_bloom_flowers_sweep(flowers, people), vec![3, 3, 3]);
    }

    #[test]
    fn test_person_at_boundary() {
        let flowers = vec![vec![2, 5], vec![3, 7]];
        let people = vec![2, 5, 7, 8];
        assert_eq!(Solution::full_bloom_flowers(flowers.clone(), people.clone()), vec![1, 2, 1, 0]);
        assert_eq!(Solution::full_bloom_flowers_sweep(flowers, people), vec![1, 2, 1, 0]);
    }
}
