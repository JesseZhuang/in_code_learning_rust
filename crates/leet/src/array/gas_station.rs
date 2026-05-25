pub struct Solution;

impl Solution {
    /// One-pass greedy solution
    /// Time: O(n), Space: O(1)
    ///
    /// Key insight:
    /// - If total_gain >= 0, there exists a solution
    /// - If curr_gain < 0 at station i, no station from 0..=i can be the start
    /// - Reset start to i+1 and continue
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_gain = 0;
        let mut curr_gain = 0;
        let mut start = 0;

        // O(n) - single pass through stations
        for i in 0..n {
            let gain = gas[i] - cost[i];
            total_gain += gain; // O(1)
            curr_gain += gain; // O(1)

            // If curr_gain < 0, all stations from start..=i cannot be the answer
            // Reset to try starting from i+1
            if curr_gain < 0 {
                start = i + 1; // O(1)
                curr_gain = 0; // O(1)
            }
        }

        // If total_gain >= 0, there's enough gas to complete the circuit
        // The last start candidate is the answer
        if total_gain >= 0 {
            start as i32 // O(1)
        } else {
            -1 // O(1)
        }
    }

    /// Brute force solution
    /// Time: O(n^2), Space: O(1)
    ///
    /// Try each station as a starting point and simulate the circuit
    pub fn can_complete_circuit_brute(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();

        // O(n) - try each starting station
        for start in 0..n {
            let mut tank = 0;
            let mut can_complete = true;

            // O(n) - simulate circuit from this start
            for offset in 0..n {
                let i = (start + offset) % n; // O(1)
                tank += gas[i]; // O(1)

                // Check if we have enough gas to reach next station
                if tank < cost[i] {
                    can_complete = false; // O(1)
                    break;
                }

                tank -= cost[i]; // O(1)
            }

            if can_complete {
                return start as i32; // O(1)
            }
        }

        -1 // O(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_basic_cases() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn test_greedy_single_station() {
        assert_eq!(Solution::can_complete_circuit(vec![5], vec![3]), 0);
        assert_eq!(Solution::can_complete_circuit(vec![3], vec![5]), -1);
        assert_eq!(Solution::can_complete_circuit(vec![4], vec![4]), 0);
    }

    #[test]
    fn test_greedy_edge_cases() {
        assert_eq!(
            Solution::can_complete_circuit(vec![0, 0, 0], vec![0, 0, 0]),
            0
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![10000, 0, 0, 0], vec![0, 5000, 3000, 2000]),
            0
        );
    }

    #[test]
    fn test_brute_basic_cases() {
        assert_eq!(
            Solution::can_complete_circuit_brute(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit_brute(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn test_brute_single_station() {
        assert_eq!(Solution::can_complete_circuit_brute(vec![5], vec![3]), 0);
        assert_eq!(Solution::can_complete_circuit_brute(vec![3], vec![5]), -1);
        assert_eq!(Solution::can_complete_circuit_brute(vec![4], vec![4]), 0);
    }

    #[test]
    fn test_brute_edge_cases() {
        assert_eq!(
            Solution::can_complete_circuit_brute(vec![0, 0, 0], vec![0, 0, 0]),
            0
        );
        assert_eq!(
            Solution::can_complete_circuit_brute(vec![10000, 0, 0, 0], vec![0, 5000, 3000, 2000]),
            0
        );
    }
}
