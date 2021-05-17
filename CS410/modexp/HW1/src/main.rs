//! Command line modular exponentiation tool
//!
//! Timothy Maloney

use std::env;

// get command line arguments, calculate modexp
fn main() {
    let args: Vec<String> = env::args().collect();

    let x: u64 = args[1].trim().parse().expect("Error!");
    let y: u64 = args[2].trim().parse().expect("Error!");
    let m: u64 = args[3].trim().parse().expect("Error!");

    println!("modexp({}, {}, {}) = {}", x, y, m, modexp(x, y, m));
}

// calculate (x^y) % m, return result
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if x == 0 {
        0
    } else if y == 0 {
        1
    } else {
        let mut z = modexp(x, y / 2, m); // z must be mutable
        z = (z * z) % m;

        if y % 2 == 1 {
            z = (z * x) % m;
        }

        z
    }
}

#[test]
// test modexp with various args, compare result to arithmetic
fn modexp_test() {
    assert!(modexp(2, 4, 5) == 2u64.pow(4) % 5);
    assert!(modexp(2, 4, 10) == 2u64.pow(4) % 10);
    assert!(modexp(2, 5, 34) == 2u64.pow(5) % 34);
    assert!(modexp(3, 7, 20) == 3u64.pow(7) % 20);
}
