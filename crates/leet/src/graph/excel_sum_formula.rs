// lc 631 — design excel sum formula
//
// Solution: cached cell values plus reverse dependency edges; O(1) get after updates.
// Solution2: store formulas only; evaluate lazily on get (recursive DAG eval with per-call memo).

use std::collections::{HashMap, VecDeque};

type Cell = (usize, usize);

fn parse_column(s: &str) -> usize {
    let mut v = 0usize;
    for ch in s.chars() {
        v = v * 26 + (ch as u8 - b'A' + 1) as usize;
    }
    v
}

fn cell(row: i32, column: &str) -> Cell {
    ((row - 1) as usize, parse_column(column) - 1)
}

fn parse_ref(reference: &str) -> Cell {
    let mut i = 0usize;
    let bytes = reference.as_bytes();
    while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
        i += 1;
    }
    let col_part = &reference[..i];
    let row_part: i32 = reference[i..].parse().expect("valid row suffix");
    ((row_part - 1) as usize, parse_column(col_part) - 1)
}

fn parse_formula(numbers: &[String]) -> HashMap<Cell, i32> {
    let mut refs: HashMap<Cell, i32> = HashMap::new();
    for token in numbers {
        if let Some((a, b)) = token.split_once(':') {
            let (sr, sc) = parse_ref(a);
            let (er, ec) = parse_ref(b);
            for r in sr..=er {
                for c in sc..=ec {
                    *refs.entry((r, c)).or_insert(0) += 1;
                }
            }
        } else {
            let c = parse_ref(token);
            *refs.entry(c).or_insert(0) += 1;
        }
    }
    refs
}

pub struct Solution {
    rows: usize,
    cols: usize,
    values: Vec<Vec<i32>>,
    formulas: HashMap<Cell, HashMap<Cell, i32>>,
    dependents: HashMap<Cell, HashMap<Cell, i32>>,
}

impl Solution {
    pub fn new(height: i32, width: &str) -> Self {
        let rows = height as usize;
        let cols = parse_column(width);
        Self {
            rows,
            cols,
            values: vec![vec![0; cols]; rows],
            formulas: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    pub fn set(&mut self, row: i32, column: &str, val: i32) {
        let target = cell(row, column);
        let old = self.values[target.0][target.1];
        self.clear_formula(target);
        self.values[target.0][target.1] = val;
        self.propagate(target, val - old);
    }

    pub fn get(&self, row: i32, column: &str) -> i32 {
        let t = cell(row, column);
        self.values[t.0][t.1]
    }

    pub fn sum(&mut self, row: i32, column: &str, numbers: Vec<String>) -> i32 {
        let target = cell(row, column);
        let old = self.values[target.0][target.1];
        self.clear_formula(target);

        let refs = parse_formula(&numbers);
        for (src, &weight) in &refs {
            *self
                .dependents
                .entry(*src)
                .or_default()
                .entry(target)
                .or_insert(0) += weight;
        }
        self.formulas.insert(target, refs.clone());

        let new_value: i32 = refs
            .iter()
            .map(|(src, w)| self.values[src.0][src.1] * w)
            .sum();
        self.values[target.0][target.1] = new_value;
        self.propagate(target, new_value - old);
        new_value
    }

    fn clear_formula(&mut self, target: Cell) {
        let Some(refs) = self.formulas.remove(&target) else {
            return;
        };
        for (src, weight) in refs {
            if let Some(dm) = self.dependents.get_mut(&src) {
                let e = dm.entry(target).or_insert(0);
                *e -= weight;
                if *e == 0 {
                    dm.remove(&target);
                }
                if dm.is_empty() {
                    self.dependents.remove(&src);
                }
            }
        }
    }

    fn collect_affected(&self, start: Cell) -> HashMap<Cell, ()> {
        let mut affected: HashMap<Cell, ()> = HashMap::new();
        let mut q: VecDeque<Cell> = VecDeque::new();
        q.push_back(start);
        while let Some(src) = q.pop_front() {
            if let Some(dsts) = self.dependents.get(&src) {
                for dst in dsts.keys() {
                    if affected.contains_key(dst) {
                        continue;
                    }
                    affected.insert(*dst, ());
                    q.push_back(*dst);
                }
            }
        }
        affected
    }

    fn propagate(&mut self, start: Cell, delta: i32) {
        if delta == 0 || !self.dependents.contains_key(&start) {
            return;
        }
        let affected_map = self.collect_affected(start);
        if affected_map.is_empty() {
            return;
        }

        let dependents = &self.dependents;
        let mut indegree: HashMap<Cell, usize> = affected_map.keys().map(|&c| (c, 0)).collect();
        let mut nodes: Vec<Cell> = Vec::with_capacity(1 + affected_map.len());
        nodes.push(start);
        nodes.extend(affected_map.keys().copied());
        for &src in &nodes {
            if let Some(dsts) = dependents.get(&src) {
                for dst in dsts.keys() {
                    if indegree.contains_key(dst) {
                        *indegree.get_mut(dst).unwrap() += 1;
                    }
                }
            }
        }

        let mut accumulated: HashMap<Cell, i32> = HashMap::new();
        accumulated.insert(start, delta);
        let mut queue: VecDeque<Cell> = VecDeque::new();
        queue.push_back(start);
        let mut value_deltas: HashMap<Cell, i32> = HashMap::new();

        while let Some(src) = queue.pop_front() {
            let acc_src = *accumulated.get(&src).unwrap_or(&0);
            if let Some(dsts) = dependents.get(&src) {
                for (dst, &weight) in dsts {
                    if !indegree.contains_key(dst) {
                        continue;
                    }
                    *accumulated.entry(*dst).or_insert(0) += acc_src * weight;
                    let deg = indegree.get_mut(dst).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        let add = *accumulated.get(dst).unwrap_or(&0);
                        value_deltas.insert(*dst, add);
                        queue.push_back(*dst);
                    }
                }
            }
        }

        for (dst, add) in value_deltas {
            self.values[dst.0][dst.1] += add;
        }
    }
}

pub struct Solution2 {
    rows: usize,
    cols: usize,
    values: Vec<Vec<i32>>,
    formulas: HashMap<Cell, HashMap<Cell, i32>>,
}

impl Solution2 {
    pub fn new(height: i32, width: &str) -> Self {
        let rows = height as usize;
        let cols = parse_column(width);
        Self {
            rows,
            cols,
            values: vec![vec![0; cols]; rows],
            formulas: HashMap::new(),
        }
    }

    pub fn set(&mut self, row: i32, column: &str, val: i32) {
        let target = cell(row, column);
        self.formulas.remove(&target);
        self.values[target.0][target.1] = val;
    }

    pub fn get(&self, row: i32, column: &str) -> i32 {
        let mut memo = HashMap::new();
        self.evaluate(cell(row, column), &mut memo)
    }

    pub fn sum(&mut self, row: i32, column: &str, numbers: Vec<String>) -> i32 {
        let target = cell(row, column);
        self.formulas
            .insert(target, parse_formula(&numbers));
        self.get(row, column)
    }

    fn evaluate(&self, c: Cell, memo: &mut HashMap<Cell, i32>) -> i32 {
        if let Some(&v) = memo.get(&c) {
            return v;
        }
        if !self.formulas.contains_key(&c) {
            return self.values[c.0][c.1];
        }
        let mut total: i32 = 0;
        for (src, &w) in &self.formulas[&c] {
            total += self.evaluate(*src, memo) * w;
        }
        memo.insert(c, total);
        total
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, Solution2};

    trait ExcelLike {
        fn new(height: i32, width: &str) -> Self;
        fn set(&mut self, row: i32, column: &str, val: i32);
        fn get(&self, row: i32, column: &str) -> i32;
        fn sum(&mut self, row: i32, column: &str, numbers: Vec<String>) -> i32;
    }

    impl ExcelLike for Solution {
        fn new(height: i32, width: &str) -> Self {
            Solution::new(height, width)
        }
        fn set(&mut self, row: i32, column: &str, val: i32) {
            Solution::set(self, row, column, val);
        }
        fn get(&self, row: i32, column: &str) -> i32 {
            Solution::get(self, row, column)
        }
        fn sum(&mut self, row: i32, column: &str, numbers: Vec<String>) -> i32 {
            Solution::sum(self, row, column, numbers)
        }
    }

    impl ExcelLike for Solution2 {
        fn new(height: i32, width: &str) -> Self {
            Solution2::new(height, width)
        }
        fn set(&mut self, row: i32, column: &str, val: i32) {
            Solution2::set(self, row, column, val);
        }
        fn get(&self, row: i32, column: &str) -> i32 {
            Solution2::get(self, row, column)
        }
        fn sum(&mut self, row: i32, column: &str, numbers: Vec<String>) -> i32 {
            Solution2::sum(self, row, column, numbers)
        }
    }

    fn assert_example<E: ExcelLike>() {
        let mut x = E::new(3, "C");
        x.set(1, "A", 2);
        assert_eq!(x.sum(3, "C", vec!["A1".into(), "A1:B2".into()]), 4);
        x.set(2, "B", 2);
        assert_eq!(x.get(3, "C"), 6);
    }

    fn assert_overwrite_formula<E: ExcelLike>() {
        let mut x = E::new(2, "C");
        x.set(1, "A", 5);
        x.set(1, "B", 3);
        assert_eq!(x.sum(2, "C", vec!["A1:B1".into()]), 8);
        assert_eq!(x.get(2, "C"), 8);
        x.set(2, "C", 100);
        x.set(1, "A", 20);
        assert_eq!(x.get(2, "C"), 100);
    }

    fn assert_replace_chain<E: ExcelLike>() {
        let mut x = E::new(2, "D");
        x.set(1, "A", 2);
        x.set(1, "B", 3);
        assert_eq!(x.sum(1, "C", vec!["A1".into(), "A1:B1".into()]), 7);
        assert_eq!(x.sum(2, "A", vec!["C1".into(), "B1".into()]), 10);
        assert_eq!(x.get(2, "A"), 10);
        assert_eq!(x.sum(1, "C", vec!["B1".into()]), 3);
        assert_eq!(x.get(2, "A"), 6);
        x.set(1, "A", 10);
        assert_eq!(x.get(1, "C"), 3);
        assert_eq!(x.get(2, "A"), 6);
        x.set(1, "B", 5);
        assert_eq!(x.get(1, "C"), 5);
        assert_eq!(x.get(2, "A"), 10);
    }

    fn assert_overlapping_ranges<E: ExcelLike>() {
        let mut x = E::new(3, "D");
        x.set(1, "A", 1);
        x.set(1, "B", 2);
        x.set(2, "A", 3);
        x.set(2, "B", 4);
        assert_eq!(
            x.sum(3, "D", vec!["A1:B2".into(), "B1:B2".into()]),
            16
        );
        x.set(2, "B", 5);
        assert_eq!(x.get(3, "D"), 18);
    }

    macro_rules! both {
        ($f:ident) => {
            $f::<Solution>();
            $f::<Solution2>();
        };
    }

    #[test]
    fn example() {
        both!(assert_example);
    }

    #[test]
    fn overwrite_formula_with_raw_value() {
        both!(assert_overwrite_formula);
    }

    #[test]
    fn replace_formula_and_update_chain() {
        both!(assert_replace_chain);
    }

    #[test]
    fn overlapping_ranges_count_multiple_times() {
        both!(assert_overlapping_ranges);
    }
}
