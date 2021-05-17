### What I did:

The goal of this assignment was to write a simple calculator that accepts
three numbers (x,y,m) and computes and prints the value (x^y) % m. 

First, I made sure I understood how to accept and manipulate command line arguments in 
Rust. Getting command line arguments in Rust is much different than any other language
I have programmed in. You first must introduce the evironment scope into the program  using `std::env`.
Once this scope is in the program, you can call the `args().collect()` method to read the command line arguments into a vector of strings.
This vector can later be accessed to access the individual command line arguments themeselves.

Next, I had to convert the arguments from strings to unsigned integers in order to perform arithmetic on them. 
This was as straightforward as using `let m: u64 = m.trim().parse().expect("Error: third arg is invalid");`

Once the command line arguments were taken care of, it was simply a matter of implementing the given pseudocode as a function,
and calling that function from within a print statement.

### How it went:

So far, I am enjoying Rust. Before I took this class I implemented the guessing game that is described in the Rust tutorial.
This helped me hit the ground running in this first week.

The biggest challenge I had with this program was implementing the actual `modexp(x,y,m)` function. My biggest misunderstanding was the 
difference between statements and expressions, and how to properly use those to return values from a function.

For example, in C, you don't need if an if statement will just be used to check and return if true, no blocks are needed; you can simply write a return statement underneath the if statement
and be done. For multiple checks, you can just use repeats of this statement each with the necessary conditions.

In Rust, all blocks are evaluated as expressions, and there are no return statements. You also must write all the conditions as a single branch. 

For example, this C program:

```
 if(condition)
   return var;
 if(condition)
   return other_var;

 // do stuff to var

 return var
```

and it's Rust equivalent:

```
 if condition {
   var
 }
 else if condition {
   other_var
 }
 else{
   // do stuff to var
   var
 }
```

While the idea is the same, and the syntax is only a little different, there are some implementation details that in my opinion, make for more readable, clearer code.

Once this part was figured out, I simply implemented the pseudocode we were given as part of the assignment.

### How it was tested:

To test the function, I computed the value by hand, and compared that with the output of the program. I also used an online calculator to verify my hand computed results.
The program's output and my hand computed values match, therefore the program is correct.
