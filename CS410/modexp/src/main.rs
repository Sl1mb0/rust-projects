use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let x: u64 = args[1].trim().parse().expect("Error!");
    let y: u64 = args[2].trim().parse().expect("Error!");
    let m: u64 = args[3].trim().parse().expect("Error!");

    println!("modexp({}, {}, {}) = {}", x, y, m, modexp(x,y,m));
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if x == 0 {
        0
    } else if y == 0 {
        1
    }
    else {
        let mut z = modexp(x, y / 2, m); // z must be mutable
        z = (z * z) % m;

        if y % 2 == 1 {
            z = (z * x) % m;
        }

        z
    }
}

#[test]
fn test() {
    assert!(modexp(2,4,5) == 1);
    assert!(modexp(2,4,10) == 6);
    assert!(modexp(2,5,34) == 32);
}
