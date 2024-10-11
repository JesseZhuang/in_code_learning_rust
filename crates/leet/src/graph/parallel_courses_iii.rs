// lc 2050

use std::cmp::max;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let (mut cache, mut adj) = (vec![-1; n + 1], vec![vec![]; n + 1]);
        for e in relations {
            adj[e[0] as usize].push(e[1] as usize);
        }
        let env = Env {
            adj,
            time,
        };
        fn dfs(env: &Env, v: usize, cache: &mut Vec<i32>) {
            if cache[v] != -1 { return; }
            cache[v] = 0;
            for w in &env.adj[v] {
                dfs(env, *w, cache);
                cache[v] = max(cache[v], cache[*w]);
            }
            cache[v] += env.time[v - 1];
        }
        for v in 1..=n { dfs(&env, v, &mut cache); }
        *cache.iter().max().unwrap()
    }
}

struct Env {
    adj: Vec<Vec<usize>>,
    time: Vec<i32>,
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::graph::parallel_courses_iii::Solution;

    #[test]
    fn test() {
        let relations = vec![vec![1, 3], vec![2, 3]];
        let time = vec![3, 2, 5];
        println!("{}", Solution::minimum_time(3, relations, time));
    }
}