//#![warn(rust_2018_idioms)]
//#![allow(elided_lifetimes_in_paths)]

use std::env;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 { // pass by value
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}

fn main() {
    let mut numbers = Vec::new(); // python list, javascript array

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument")); // exits program if this fails
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] { // slice, borrows a reference
        d = gcd(d, *m); // * dereferences m
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
