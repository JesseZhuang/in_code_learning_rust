// lc 721 — O(nk·α(nk) + nk·log(nk)) union-find; O(nk·log(nk)) DFS
use std::collections::{HashMap, HashSet};

pub struct Solution;

pub struct Solution2;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut rank: HashMap<String, i32> = HashMap::new();
        let mut email_to_name: HashMap<String, String> = HashMap::new();

        for acct in &accounts {
            let name = &acct[0];
            let emails: Vec<&str> = acct[1..].iter().map(|s| s.as_str()).collect();
            for &e in &emails {
                parent.entry(e.to_string()).or_insert_with(|| e.to_string());
                rank.entry(e.to_string()).or_insert(0);
                email_to_name.entry(e.to_string()).or_insert_with(|| name.clone());
                Self::union(&mut parent, &mut rank, emails[0], e);
            }
        }

        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for email in parent.keys().cloned().collect::<Vec<_>>() {
            let r = Self::find(&mut parent, &email);
            groups.entry(r).or_default().push(email);
        }

        groups
            .into_iter()
            .map(|(root, mut emails)| {
                emails.sort();
                emails.dedup();
                let mut row = vec![email_to_name[&root].clone()];
                row.extend(emails);
                row
            })
            .collect()
    }

    fn find(parent: &mut HashMap<String, String>, x: &str) -> String {
        let mut cur = x.to_string();
        loop {
            let p = parent[&cur].clone();
            if p == cur {
                return cur;
            }
            let gp = parent[&p].clone();
            *parent.get_mut(&cur).unwrap() = gp.clone();
            cur = gp;
        }
    }

    fn union(parent: &mut HashMap<String, String>, rank: &mut HashMap<String, i32>, a: &str, b: &str) {
        let mut ra = Self::find(parent, a);
        let mut rb = Self::find(parent, b);
        if ra == rb {
            return;
        }
        let rank_a = *rank.get(&ra).unwrap_or(&0);
        let rank_b = *rank.get(&rb).unwrap_or(&0);
        if rank_a < rank_b {
            std::mem::swap(&mut ra, &mut rb);
        }
        parent.insert(rb.clone(), ra.clone());
        if rank_a == rank_b {
            *rank.entry(ra).or_insert(0) += 1;
        }
    }
}

impl Solution2 {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        let mut email_to_name: HashMap<String, String> = HashMap::new();

        for acct in &accounts {
            let name = &acct[0];
            let first = &acct[1];
            for email in &acct[1..] {
                email_to_name.entry(email.clone()).or_insert_with(|| name.clone());
                graph.entry(first.clone()).or_default().insert(email.clone());
                graph.entry(email.clone()).or_default().insert(first.clone());
            }
        }

        fn dfs(
            email: &str,
            graph: &HashMap<String, HashSet<String>>,
            visited: &mut HashSet<String>,
            comp: &mut Vec<String>,
        ) {
            visited.insert(email.to_string());
            comp.push(email.to_string());
            if let Some(nbrs) = graph.get(email) {
                for nbr in nbrs {
                    if !visited.contains(nbr.as_str()) {
                        dfs(nbr, graph, visited, comp);
                    }
                }
            }
        }

        let mut visited: HashSet<String> = HashSet::new();
        let mut res = Vec::new();

        for email in graph.keys().cloned().collect::<Vec<_>>() {
            if visited.contains(&email) {
                continue;
            }
            let mut comp = Vec::new();
            dfs(&email, &graph, &mut visited, &mut comp);
            comp.sort();
            let mut row = vec![email_to_name[&email].clone()];
            row.extend(comp);
            res.push(row);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::accounts_merge::{Solution, Solution2};

    fn normalize(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut out: Vec<Vec<String>> = accounts
            .into_iter()
            .map(|mut row| {
                let name = row.remove(0);
                row.sort();
                row.dedup();
                std::iter::once(name).chain(row.into_iter()).collect()
            })
            .collect();
        out.sort();
        out
    }

    fn assert_both(input: Vec<Vec<String>>, expected: Vec<Vec<String>>) {
        let want = normalize(expected);
        let got1 = normalize(Solution::accounts_merge(input.clone()));
        let got2 = normalize(Solution2::accounts_merge(input));
        assert_eq!(want, got1);
        assert_eq!(want, got2);
    }

    #[test]
    fn example1() {
        assert_both(
            vec![
                vec![
                    "John".into(),
                    "johnsmith@mail.com".into(),
                    "john_newyork@mail.com".into(),
                ],
                vec![
                    "John".into(),
                    "johnsmith@mail.com".into(),
                    "john00@mail.com".into(),
                ],
                vec!["Mary".into(), "mary@mail.com".into()],
                vec!["John".into(), "johnnybravo@mail.com".into()],
            ],
            vec![
                vec![
                    "John".into(),
                    "john00@mail.com".into(),
                    "john_newyork@mail.com".into(),
                    "johnsmith@mail.com".into(),
                ],
                vec!["Mary".into(), "mary@mail.com".into()],
                vec!["John".into(), "johnnybravo@mail.com".into()],
            ],
        );
    }

    #[test]
    fn example2() {
        assert_both(
            vec![
                vec!["Gabe".into(), "gabe0@mail.com".into()],
                vec!["Kevin".into(), "kevin1@mail.com".into()],
                vec!["Ethan".into(), "ethan2@mail.com".into()],
                vec!["Hanzo".into(), "hanzo3@mail.com".into()],
                vec!["Fern".into(), "fern4@mail.com".into()],
            ],
            vec![
                vec!["Ethan".into(), "ethan2@mail.com".into()],
                vec!["Fern".into(), "fern4@mail.com".into()],
                vec!["Gabe".into(), "gabe0@mail.com".into()],
                vec!["Hanzo".into(), "hanzo3@mail.com".into()],
                vec!["Kevin".into(), "kevin1@mail.com".into()],
            ],
        );
    }

    #[test]
    fn single_account() {
        assert_both(
            vec![vec!["Alice".into(), "a@m.co".into()]],
            vec![vec!["Alice".into(), "a@m.co".into()]],
        );
    }

    #[test]
    fn all_merge() {
        assert_both(
            vec![
                vec!["Bob".into(), "e1@x.com".into()],
                vec!["Bob".into(), "e1@x.com".into(), "e2@x.com".into()],
                vec!["Bob".into(), "e2@x.com".into(), "e3@x.com".into()],
            ],
            vec![vec![
                "Bob".into(),
                "e1@x.com".into(),
                "e2@x.com".into(),
                "e3@x.com".into(),
            ]],
        );
    }

    #[test]
    fn same_name_no_merge() {
        assert_both(
            vec![
                vec!["Tom".into(), "tom1@mail.com".into()],
                vec!["Tom".into(), "tom2@mail.com".into()],
            ],
            vec![
                vec!["Tom".into(), "tom1@mail.com".into()],
                vec!["Tom".into(), "tom2@mail.com".into()],
            ],
        );
    }

    #[test]
    fn chain_merge() {
        assert_both(
            vec![
                vec!["Ann".into(), "a1@x.com".into(), "a2@x.com".into()],
                vec!["Ann".into(), "a2@x.com".into(), "a3@x.com".into()],
                vec!["Ann".into(), "a3@x.com".into(), "a4@x.com".into()],
            ],
            vec![vec![
                "Ann".into(),
                "a1@x.com".into(),
                "a2@x.com".into(),
                "a3@x.com".into(),
                "a4@x.com".into(),
            ]],
        );
    }
}
