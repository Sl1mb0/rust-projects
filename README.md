# rust-projects

This is just a collection of all the things I have written in Rust.

Rust is kewl.

## actix-modexp
 Web Server that runs on local host. User inputs 3 numbers: `a, b, c` and the server calculates and displays the result of `a^b % c`.

## mandelbrot
 Program to generate `mandelbrot.png`.
 
 The Mandelbrot set is defined as the set of complex numbers `c` for which the sequence of a function `f(z) = z^2 + c`
 remains bounded by some absolute value `n` such that `n > 0`
 
 For example, this sequence would be bounded by some nonzero absolute value.
 `f(0), f(f(0)), f(f(f(0))), ...`
 
 The image is generated through checking if a pixel (or point) belongs to the set,
 if it does, it colors the pixel black.
