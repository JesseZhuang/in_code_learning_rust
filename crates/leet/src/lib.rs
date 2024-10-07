#![allow(dead_code)] // allow unused code for this whole crate

// leet code categories as mods
pub mod array;
pub mod sliding_window;
mod hash;
mod tree;

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
