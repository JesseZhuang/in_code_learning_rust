#![allow(dead_code)] // allow unused code for this whole crate

// leet code categories as mods
mod array;
mod sliding_window;
mod hash;
mod tree;
mod string;
mod structs;
mod dp;
mod deque;
mod bit;
mod graph;
mod heap;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
