/**
  * Timothy Maloney
  * Homework 2 (toy-rsa)
  * CS410 (Rust) 
  * Spring 2021
  *
  **/

/// Documentation can be found at:
/// [Crate toy_rsa_lib](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/index.html)
use toy_rsa_lib::*;
use std::convert::TryFrom; // used for conversion

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**30..2**31`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    let mut p = rsa_prime();
    let mut q = rsa_prime();
    while EXP > lcm((p-1) as u64,(q-1) as u64) || gcd(EXP,lcm((p-1) as u64,(q-1) as u64)) != 1 {
      p = rsa_prime();
      q = rsa_prime();
    }
    (p,q)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64,EXP,key) 
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let (p,q) = key;
    let d = modinverse(EXP,lcm((p-1) as u64,(q-1) as u64));
    let r = modexp(msg as u64,d,p as u64 * q as u64);

    // modexp() returns u64, decrypt() returns u32
    // if r > u32::MAX
    // return value will be truncated
    match u32::try_from(r) {
        Ok(v) => v,
	Err(_) => panic!("Error: modexp() returned value greater than u32::MAX in decrypt()"),
    } 
}
