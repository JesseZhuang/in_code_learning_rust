// lc 981 - Time Based Key-Value Store

use std::collections::{BTreeMap, HashMap};

// ─── Solution 1: HashMap + Vec with binary search (partition_point) ───────────
// set: O(1) amortized (append to vec)
// get: O(log n) via partition_point binary search

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>, // key -> sorted vec of (timestamp, value)
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        // O(1) amortized — timestamps are strictly increasing so append maintains sort order
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            None => String::new(),
            Some(entries) => {
                // O(log n) — partition_point finds first index where ts > timestamp
                let idx = entries.partition_point(|(ts, _)| *ts <= timestamp);
                if idx == 0 {
                    String::new() // no timestamp_prev <= timestamp
                } else {
                    entries[idx - 1].1.clone()
                }
            }
        }
    }
}

// ─── Solution 2: HashMap + BTreeMap ──────────────────────────────────────────
// set: O(log n) — BTreeMap insertion
// get: O(log n) — BTreeMap range query

struct TimeMapBTree {
    map: HashMap<String, BTreeMap<i32, String>>, // key -> BTreeMap<timestamp, value>
}

impl TimeMapBTree {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        // O(log n) — BTreeMap insert
        self.map.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            None => String::new(),
            Some(tree) => {
                // O(log n) — range query up to and including timestamp, take last
                tree.range(..=timestamp)
                    .next_back()
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timemap_basic() {
        let mut tm = TimeMap::new();
        tm.set("foo".into(), "bar".into(), 1);
        assert_eq!(tm.get("foo".into(), 1), "bar");
        assert_eq!(tm.get("foo".into(), 3), "bar"); // largest ts <= 3 is 1
        tm.set("foo".into(), "bar2".into(), 4);
        assert_eq!(tm.get("foo".into(), 4), "bar2");
        assert_eq!(tm.get("foo".into(), 5), "bar2");
    }

    #[test]
    fn test_timemap_no_match() {
        let mut tm = TimeMap::new();
        tm.set("foo".into(), "bar".into(), 2);
        assert_eq!(tm.get("foo".into(), 1), ""); // no ts <= 1
        assert_eq!(tm.get("baz".into(), 1), ""); // key doesn't exist
    }

    #[test]
    fn test_btreemap_basic() {
        let mut tm = TimeMapBTree::new();
        tm.set("foo".into(), "bar".into(), 1);
        assert_eq!(tm.get("foo".into(), 1), "bar");
        assert_eq!(tm.get("foo".into(), 3), "bar");
        tm.set("foo".into(), "bar2".into(), 4);
        assert_eq!(tm.get("foo".into(), 4), "bar2");
        assert_eq!(tm.get("foo".into(), 5), "bar2");
    }

    #[test]
    fn test_btreemap_no_match() {
        let mut tm = TimeMapBTree::new();
        tm.set("foo".into(), "bar".into(), 2);
        assert_eq!(tm.get("foo".into(), 1), "");
        assert_eq!(tm.get("baz".into(), 1), "");
    }
}
