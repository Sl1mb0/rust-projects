use rand::Rng;
use toy_rsa;
use std::thread;

pub const MAX : u32 = 6400;
pub const NTHREADS : u32 = 8;

#[test]
fn single_check() {
    let (p,q) = toy_rsa::genkey(); 
    let public = p as u64 * q as u64;
 
    let msg : u32 = rand::thread_rng().gen_range(1, MAX);
    let encrypted_msg : u64 = toy_rsa::encrypt(public,msg);
    let decrypted_msg: u32 = toy_rsa::decrypt((p,q),encrypted_msg);

    assert!(msg == decrypted_msg);
}

#[test]
fn multiple_checks() {
    // encrypt and decrypt a
    // random number exactly: 
    // 6400 times...
    // ......................
    // WITH MULTITHREADING!!!
    let mut num_checks: u32 = MAX/NTHREADS;
    let mut children = vec![]; // store threads

    for _i in 1..NTHREADS {
        children.push(thread::spawn(move || {
            while num_checks > 0 {
                single_check();
                num_checks = num_checks-1;
            }
        }));
    }

    for child in children {
      // wait for thread to finish
      let _ = child.join();
    }
}
