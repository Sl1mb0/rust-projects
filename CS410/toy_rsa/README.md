### What I did:

The goal of this assignment was to create a Rust Library crate that would be used for RSA encryption.

I used the Professor-Authored `toy_rsa_lib` library to build an Rust Crate that provides 3 functions:
- `genkey()` to generate a 32-bit RSA key pair.
- `encrypt()` to encrypt 32-bit messages using a 64-bit public key
- `decrypt()` to decrypt encrypted messages

### How it went:

It went surprisingly well, despite two things:

- when I initially wrote the code in `lib.rs`, I forgot about the need to convert between `u64` and `u32`. I also had to consider how to handle the possibility of overflow
  when casting from `u64` to `u32`. Though it wasn't that challenging, there was more complexity than I initially anticipated.

- I mistakenly thought that the key-pair generated a private AND public key, instead of the public key being the product of the generated key-pair. This caused
  an overflow in my `decrypt()` function when storing the result of `modexp(msg,d,p* q)`. Thankfully, there is an awesome TA in this class who helped point that out.

### How it was tested:
 
I wrote a `single_check()` function that generates a random 32-bit number, encrypts and decrypts it, and asserts that the decrypted number is the same as the original "message".
I then wrote a `multiple_check()` function that uses multi-threading to call `single_check()` 6400 times. I did try to test it by calling `single_check()` u32::MAX times, 
but my computer nearly crashed from the amount of threads I was using to do it. What a great excuse to get a new PC.
